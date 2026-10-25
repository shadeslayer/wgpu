use super::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(super) enum AddressProjection {
    Field(u32),
    Index(Handle<Expression>),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(super) struct BoundAddress {
    root: Handle<Expression>,
    projections: Vec<AddressProjection>,
}

#[derive(Clone, Debug)]
struct BoundTarget {
    // Projected views retain source tags while addressing different subobjects.
    selector: u32,
    address: Handle<BoundAddress>,
}

#[derive(Clone, Debug)]
pub(super) struct BoundReference {
    selector: Option<Handle<Expression>>,
    targets: Vec<BoundTarget>,
}

impl<'a> Parser<'a> {
    pub(super) fn local_reference_declaration(
        &mut self,
        name: &'a str,
        span: Span,
        specifier: TypeSpecifier,
        is_const: bool,
    ) -> Result<(), ParseError> {
        self.insert_symbol(name, Symbol::PendingReference, span)?;
        let ty = self.lower_type(specifier, span);
        let has_equals = self.lexer.next_if(Token::Operation('='))?;
        let value = if self.lexer.peek()?.0 == Token::Paren('{') {
            self.reference_list_initializer(&ty)?
        } else {
            if !has_equals {
                return self.fail("reference declaration requires an initializer");
            }
            self.expression(0)?
        };
        let (binding, constant) = match value {
            ExpressionValue::Reference(reference)
                if self.module.compare_types(&reference.ty, &ty) =>
            {
                if !is_const && !reference.writable {
                    return Err(ParseError {
                        message: String::from("mutable reference requires a writable lvalue"),
                        span: reference.span,
                    });
                }
                let is_discardable = self.reference_has_static_address(&reference)?;
                let binding = self.freeze_reference(reference)?;
                (
                    binding,
                    ConstantValue {
                        literal: None,
                        is_discardable,
                    },
                )
            }
            value => {
                if !is_const {
                    return Err(ParseError {
                        message: String::from("mutable reference requires an exact-type lvalue"),
                        span: value.span(),
                    });
                }
                let value = self.convert_value(value, &ty)?;
                let pointer = self.temporary_pointer(&ty, span)?;
                self.push_statement(
                    Statement::Store {
                        pointer,
                        value: value.handle,
                    },
                    span.until(&value.span),
                );
                let binding = self.freeze_reference(ReferenceValue {
                    reference: Reference::Pointer(pointer),
                    writable: false,
                    ty: ty.clone(),
                    span,
                    constant: ConstantValue::default(),
                })?;
                (binding, ConstantValue::default())
            }
        };
        self.function.symbol_table.add(
            name,
            Symbol::Reference {
                binding,
                ty,
                writable: !is_const,
                constant,
            },
        );
        Ok(())
    }

    fn reference_list_initializer(
        &mut self,
        ty: &TypeResolution,
    ) -> Result<ExpressionValue, ParseError> {
        let opening = self.lexer.expect_span(Token::Paren('{'))?;
        let actual_ty = self.type_handle(ty, opening);
        if self.lexer.next_if(Token::Paren('}'))? {
            let handle = self.append(Expression::ZeroValue(actual_ty), opening)?;
            return Ok(Value {
                handle,
                ty: ty.clone(),
                span: opening,
                constant: ConstantValue::default(),
            }
            .into());
        }
        let first = if self.lexer.peek()?.0 == Token::Paren('{') {
            None
        } else {
            Some(self.expression(0)?)
        };
        let mut closing = self.lexer.clone();
        closing.next_if(Token::Separator(','))?;
        let singleton = closing.peek()?.0 == Token::Paren('}');
        if singleton
            && first
                .as_ref()
                .is_some_and(|value| self.module.compare_types(value.ty(), ty))
        {
            self.lexer.next_if(Token::Separator(','))?;
            self.lexer.expect_span(Token::Paren('}'))?;
            return Ok(first.unwrap());
        }
        let pending = match first {
            Some(value) => Some(self.rvalue(value)?),
            None => None,
        };
        if let TypeInner::Scalar(scalar) = *ty.inner_with(&self.module.types) {
            let Some(value) = pending else {
                return self.fail("nested scalar initializer lists are unsupported");
            };
            let value = self.list_convert_scalar(value, scalar)?;
            self.lexer.next_if(Token::Separator(','))?;
            self.lexer.expect_span(Token::Paren('}'))?;
            return Ok(value.into());
        }
        let pointer = self.temporary_pointer(ty, opening)?;
        self.record_aggregate_initializer(pointer, actual_ty, true, opening, pending)?;
        Ok(self.load_pointer(pointer, ty, opening)?.into())
    }

    fn charge_reference_binding(&mut self, work: usize, span: Span) -> Result<(), ParseError> {
        let remaining = 65_536 - self.function.reference_binding_work;
        if work > remaining as usize {
            return Err(ParseError {
                message: String::from("CUDA reference binding expansion limit exceeded"),
                span,
            });
        }
        self.function.reference_binding_work += work as u32;
        Ok(())
    }

    fn retain_bound_address(
        &mut self,
        address: BoundAddress,
        span: Span,
    ) -> Result<Handle<BoundAddress>, ParseError> {
        self.charge_reference_binding(address.projections.len() + 1, span)?;
        if let Some(&handle) = self.function.bound_address_indices.get(&address) {
            return Ok(handle);
        }
        let handle = self.function.bound_addresses.append(address.clone(), span);
        self.function.bound_address_indices.insert(address, handle);
        Ok(handle)
    }

    fn retain_bound_reference(
        &mut self,
        selector: Option<Handle<Expression>>,
        targets: Vec<BoundTarget>,
        span: Span,
    ) -> Result<Handle<BoundReference>, ParseError> {
        self.charge_reference_binding(targets.len() + 1, span)?;
        Ok(self
            .function
            .bound_references
            .append(BoundReference { selector, targets }, span))
    }

    fn bound_reference_view(
        &mut self,
        handle: Handle<BoundReference>,
        span: Span,
    ) -> Result<BoundReference, ParseError> {
        self.charge_reference_binding(
            self.function.bound_references[handle].targets.len() + 1,
            span,
        )?;
        Ok(self.function.bound_references[handle].clone())
    }

    fn freeze_reference_index(
        &mut self,
        index: Handle<Expression>,
        span: Span,
    ) -> Result<Handle<Expression>, ParseError> {
        self.charge_reference_binding(1, span)?;
        let pointer = self.temporary_pointer(&scalar_resolution(Scalar::U32), span)?;
        let value = self.append(
            Expression::As {
                expr: index,
                kind: ScalarKind::Uint,
                convert: Some(4),
            },
            span,
        )?;
        self.push_statement(Statement::Store { pointer, value }, span);
        Ok(pointer)
    }

    fn freeze_pointer_address(
        &mut self,
        mut pointer: Handle<Expression>,
        span: Span,
    ) -> Result<Handle<BoundAddress>, ParseError> {
        let mut projections = Vec::new();
        loop {
            self.charge_reference_binding(1, span)?;
            match self.function.ir.expressions[pointer] {
                Expression::AccessIndex { base, index } => {
                    projections.push(AddressProjection::Field(index));
                    pointer = base;
                }
                Expression::Access { base, index } => {
                    let captured = self.freeze_reference_index(index, span)?;
                    projections.push(AddressProjection::Index(captured));
                    pointer = base;
                }
                Expression::LocalVariable(_) | Expression::GlobalVariable(_) => break,
                _ => {
                    return Err(ParseError {
                        message: String::from("unsupported CUDA reference address"),
                        span,
                    })
                }
            }
        }
        projections.reverse();
        self.retain_bound_address(
            BoundAddress {
                root: pointer,
                projections,
            },
            span,
        )
    }

    fn bound_address_pointer(
        &mut self,
        handle: Handle<BoundAddress>,
        span: Span,
    ) -> Result<Handle<Expression>, ParseError> {
        self.charge_reference_binding(
            self.function.bound_addresses[handle].projections.len() + 1,
            span,
        )?;
        let address = self.function.bound_addresses[handle].clone();
        let mut pointer = address.root;
        for projection in address.projections {
            pointer = match projection {
                AddressProjection::Field(index) => self.append(
                    Expression::AccessIndex {
                        base: pointer,
                        index,
                    },
                    span,
                )?,
                AddressProjection::Index(captured) => {
                    let index = self.append(Expression::Load { pointer: captured }, span)?;
                    self.append(
                        Expression::Access {
                            base: pointer,
                            index,
                        },
                        span,
                    )?
                }
            };
        }
        Ok(pointer)
    }

    fn with_bound_target(
        &mut self,
        handle: Handle<BoundReference>,
        span: Span,
        action: &mut impl FnMut(&mut Self, Handle<BoundAddress>) -> Result<(), ParseError>,
    ) -> Result<(), ParseError> {
        let view = self.bound_reference_view(handle, span)?;
        let Some(pointer) = view.selector else {
            return action(self, view.targets[0].address);
        };
        let selector = self.append(Expression::Load { pointer }, span)?;
        let last = view.targets.len() - 1;
        let mut cases = Vec::with_capacity(view.targets.len());
        for (index, target) in view.targets.into_iter().enumerate() {
            self.charge_reference_binding(1, span)?;
            let ((), body) = self.capture_block(|parser| action(parser, target.address))?;
            cases.push(crate::SwitchCase {
                value: if index == last {
                    crate::SwitchValue::Default
                } else {
                    crate::SwitchValue::U32(target.selector)
                },
                body,
                fall_through: false,
            });
        }
        self.push_statement(Statement::Switch { selector, cases }, span);
        Ok(())
    }

    pub(super) fn consume_bound_reference(
        &mut self,
        handle: Handle<BoundReference>,
        reference: ReferenceValue,
        action: &mut impl FnMut(&mut Self, ReferenceValue) -> Result<(), ParseError>,
    ) -> Result<(), ParseError> {
        self.with_bound_target(handle, reference.span, &mut |parser, address| {
            let pointer = parser.bound_address_pointer(address, reference.span)?;
            action(
                parser,
                ReferenceValue {
                    reference: Reference::Pointer(pointer),
                    ..reference.clone()
                },
            )
        })
    }

    pub(super) fn project_bound_reference(
        &mut self,
        handle: Handle<BoundReference>,
        projection: AddressProjection,
        span: Span,
    ) -> Result<Handle<BoundReference>, ParseError> {
        let view = self.bound_reference_view(handle, span)?;
        let mut targets = Vec::with_capacity(view.targets.len());
        for target in view.targets {
            self.charge_reference_binding(
                self.function.bound_addresses[target.address]
                    .projections
                    .len()
                    + 1,
                span,
            )?;
            let mut address = self.function.bound_addresses[target.address].clone();
            address.projections.push(projection.clone());
            let address = self.retain_bound_address(address, span)?;
            targets.push(BoundTarget {
                selector: target.selector,
                address,
            });
        }
        self.retain_bound_reference(view.selector, targets, span)
    }

    pub(super) fn project_bound_index(
        &mut self,
        handle: Handle<BoundReference>,
        index: Handle<Expression>,
        span: Span,
    ) -> Result<Handle<BoundReference>, ParseError> {
        let captured = self.freeze_reference_index(index, span)?;
        self.project_bound_reference(handle, AddressProjection::Index(captured), span)
    }

    fn freeze_reference(
        &mut self,
        reference: ReferenceValue,
    ) -> Result<Handle<BoundReference>, ParseError> {
        let span = reference.span;
        self.charge_reference_binding(1, span)?;
        match reference.reference {
            Reference::Bound(handle) => Ok(handle),
            Reference::Pointer(pointer) => {
                let address = self.freeze_pointer_address(pointer, span)?;
                self.retain_bound_reference(None, alloc::vec![BoundTarget {
                    selector: address.index() as u32,
                    address,
                }], span)
            }
            Reference::Constant(_) => Err(ParseError {
                message: String::from("reference binding to a host namespace constant is unsupported in CUDA device code"),
                span,
            }),
            Reference::Conditional(handle) => {
                let conditional = self.function.conditional_references[handle].take().unwrap();
                let selector = self.temporary_pointer(&scalar_resolution(Scalar::U32), span)?;
                let mut addresses = Vec::new();
                let mut branch = |parser: &mut Self, branch: ReferenceBranch| {
                    parser.append_block(branch.body);
                    let handle = parser.freeze_reference(branch.reference)?;
                    parser.with_bound_target(handle, span, &mut |parser, address| {
                        addresses.push(address);
                        let value = parser.append(Expression::Literal(Literal::U32(address.index() as u32)), span)?;
                        parser.push_statement(Statement::Store { pointer: selector, value }, span);
                        Ok(())
                    })
                };
                let ((), accept) = self.capture_block(|parser| branch(parser, conditional.accept))?;
                let ((), reject) = self.capture_block(|parser| branch(parser, conditional.reject))?;
                self.push_statement(Statement::If { condition: conditional.condition, accept, reject }, span);
                addresses.sort_unstable_by_key(|address| address.index());
                addresses.dedup();
                let targets = addresses.into_iter().map(|address| BoundTarget {
                    selector: address.index() as u32,
                    address,
                }).collect();
                self.retain_bound_reference(Some(selector), targets, span)
            }
        }
    }

    fn reference_has_static_address(
        &mut self,
        reference: &ReferenceValue,
    ) -> Result<bool, ParseError> {
        self.charge_reference_binding(1, reference.span)?;
        if !reference.constant.is_discardable {
            return Ok(false);
        }
        match reference.reference {
            Reference::Bound(_) => Ok(true),
            Reference::Constant(_) => Ok(false),
            Reference::Pointer(mut pointer) => loop {
                self.charge_reference_binding(1, reference.span)?;
                match self.function.ir.expressions[pointer] {
                    Expression::Access { base, .. } | Expression::AccessIndex { base, .. } => {
                        pointer = base
                    }
                    Expression::GlobalVariable(global) => {
                        return Ok(
                            self.module.global_variables[global].space == AddressSpace::WorkGroup
                        )
                    }
                    _ => return Ok(false),
                }
            },
            Reference::Conditional(handle) => {
                let conditional = self.function.conditional_references[handle]
                    .as_ref()
                    .unwrap();
                let selected = match conditional.constant_condition {
                    Some(true) => conditional.accept.reference.clone(),
                    Some(false) => conditional.reject.reference.clone(),
                    None => return Ok(false),
                };
                self.reference_has_static_address(&selected)
            }
        }
    }
}
