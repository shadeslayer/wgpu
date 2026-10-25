use super::*;

impl Parser<'_> {
    pub(super) fn field(
        &mut self,
        value: ExpressionValue,
        field: &str,
        field_span: Span,
    ) -> Result<ExpressionValue, ParseError> {
        let invalid = || ParseError {
            message: format!("unknown or unsupported field `{field}`"),
            span: field_span,
        };
        let (index, ty) = match *value.ty().inner_with(&self.module.types) {
            TypeInner::Vector { size, scalar } => {
                let index = match field {
                    "x" => 0,
                    "y" => 1,
                    "z" => 2,
                    "w" => 3,
                    _ => return Err(invalid()),
                };
                if index >= size as u32 {
                    return Err(invalid());
                }
                (index, scalar_resolution(scalar))
            }
            TypeInner::Struct { ref members, .. } => {
                let index = members
                    .iter()
                    .position(|member| member.name.as_deref() == Some(field))
                    .ok_or_else(invalid)?;
                (index as u32, TypeResolution::Handle(members[index].ty))
            }
            _ => return Err(invalid()),
        };
        let span = value.span().until(&field_span);
        match value {
            ExpressionValue::Plain(value) => {
                let handle = self.append(
                    Expression::AccessIndex {
                        base: value.handle,
                        index,
                    },
                    span,
                )?;
                Ok(Value {
                    handle,
                    ty,
                    span,
                    constant: ConstantValue::default(),
                }
                .into())
            }
            ExpressionValue::Reference(reference) => self
                .project_reference(reference, index, &ty, span)
                .map(ExpressionValue::Reference),
        }
    }

    pub(super) fn array_index(
        &mut self,
        value: ExpressionValue,
    ) -> Result<ExpressionValue, ParseError> {
        let (element_ty, count) = match *value.ty().inner_with(&self.module.types) {
            TypeInner::Array { base, size, .. } => (
                TypeResolution::Handle(base),
                match size {
                    ArraySize::Constant(count) => Some(count),
                    _ => None,
                },
            ),
            _ => return self.fail("subscript requires an array"),
        };
        if matches!(&value, ExpressionValue::Reference(reference) if matches!(reference.reference, Reference::Conditional(_)))
        {
            return self.fail("conditional array subscripts are unsupported");
        }
        self.lexer.expect_span(Token::Paren('['))?;
        let index = self.expression(0)?;
        let index = self.rvalue(index)?;
        let scalar = self.require_scalar_type(&index.ty, index.span)?;
        if !matches!(scalar.kind, ScalarKind::Sint | ScalarKind::Uint) {
            return Err(ParseError {
                message: String::from("array index must be an integer"),
                span: index.span,
            });
        }
        let end = self.lexer.expect_span(Token::Paren(']'))?;
        let span = value.span().until(&end);
        let discardable =
            Self::constant_array_address(value.constant().is_discardable, count, index.constant);
        match value {
            ExpressionValue::Plain(value) => {
                let handle = self.append(
                    Expression::Access {
                        base: value.handle,
                        index: index.handle,
                    },
                    span,
                )?;
                Ok(Value {
                    handle,
                    ty: element_ty,
                    span,
                    constant: ConstantValue {
                        literal: None,
                        is_discardable: discardable,
                    },
                }
                .into())
            }
            ExpressionValue::Reference(mut reference) => {
                reference.reference = match reference.reference {
                    Reference::Pointer(base) => Reference::Pointer(self.append(
                        Expression::Access {
                            base,
                            index: index.handle,
                        },
                        span,
                    )?),
                    Reference::Bound(handle) => {
                        Reference::Bound(self.project_bound_index(handle, index.handle, span)?)
                    }
                    Reference::Conditional(_) | Reference::Constant(_) => {
                        return self.fail("unsupported array reference");
                    }
                };
                reference.ty = element_ty;
                reference.span = span;
                reference.constant = ConstantValue {
                    literal: None,
                    is_discardable: discardable,
                };
                Ok(ExpressionValue::Reference(reference))
            }
        }
    }

    fn project_reference(
        &mut self,
        mut reference: ReferenceValue,
        index: u32,
        ty: &TypeResolution,
        span: Span,
    ) -> Result<ReferenceValue, ParseError> {
        match reference.reference {
            Reference::Pointer(base) => {
                reference.reference =
                    Reference::Pointer(self.append(Expression::AccessIndex { base, index }, span)?);
            }
            Reference::Bound(handle) => {
                reference.reference = Reference::Bound(self.project_bound_reference(
                    handle,
                    reference_bindings::AddressProjection::Field(index),
                    span,
                )?);
            }
            Reference::Conditional(handle) => {
                let ConditionalReference {
                    condition,
                    constant_condition,
                    accept,
                    reject,
                    depth,
                } = self.function.conditional_references[handle].take().unwrap();
                let mut project = |branch: ReferenceBranch| -> Result<ReferenceBranch, ParseError> {
                    let (reference, body) = self.capture_block(|parser| {
                        parser.append_block(branch.body);
                        parser.project_reference(branch.reference, index, ty, span)
                    })?;
                    Ok(ReferenceBranch { body, reference })
                };
                let accept = project(accept)?;
                let reject = project(reject)?;
                self.function.conditional_references[handle] = Some(ConditionalReference {
                    condition,
                    constant_condition,
                    accept,
                    reject,
                    depth,
                });
            }
            Reference::Constant(_) => {
                return Err(ParseError {
                    message: String::from("constant has no supported fields"),
                    span,
                });
            }
        }
        reference.ty = ty.clone();
        reference.span = span;
        reference.constant.literal = None;
        Ok(reference)
    }

    pub(super) fn temporary_pointer(
        &mut self,
        ty: &TypeResolution,
        span: Span,
    ) -> Result<Handle<Expression>, ParseError> {
        let ty = self.type_handle(ty, span);
        let local = self.function.ir.local_variables.append(
            LocalVariable {
                name: None,
                ty,
                init: None,
            },
            span,
        );
        self.append(Expression::LocalVariable(local), span)
    }

    pub(super) fn reference_depth(&self, reference: &ReferenceValue) -> u32 {
        match reference.reference {
            Reference::Pointer(_) | Reference::Bound(_) | Reference::Constant(_) => 0,
            Reference::Conditional(handle) => {
                self.function.conditional_references[handle]
                    .as_ref()
                    .unwrap()
                    .depth
            }
        }
    }

    pub(super) fn rvalue(&mut self, expression: ExpressionValue) -> Result<Value, ParseError> {
        if matches!(
            *expression.ty().inner_with(&self.module.types),
            TypeInner::Array { .. }
        ) {
            return Err(ParseError {
                message: String::from("standalone array values and pointer decay are unsupported"),
                span: expression.span(),
            });
        }
        let reference = match expression {
            ExpressionValue::Plain(value) => return Ok(value),
            ExpressionValue::Reference(reference) => reference,
        };
        let ty = reference.ty.clone();
        let span = reference.span;
        let constant = reference.constant;
        if !matches!(
            reference.reference,
            Reference::Conditional(_) | Reference::Bound(_)
        ) {
            return self.read_reference(reference);
        }
        let pointer = self.temporary_pointer(&ty, span)?;
        self.consume_reference(reference, &mut |parser, source| {
            let span = source.span;
            let value = parser.read_reference(source)?;
            parser.push_statement(
                Statement::Store {
                    pointer,
                    value: value.handle,
                },
                span,
            );
            Ok(())
        })?;
        let handle = self.append(Expression::Load { pointer }, span)?;
        Ok(Value {
            handle,
            ty,
            span,
            constant: ConstantValue::evaluated(constant.literal),
        })
    }

    fn read_reference(&mut self, reference: ReferenceValue) -> Result<Value, ParseError> {
        let span = reference.span;
        let handle = match reference.reference {
            Reference::Pointer(pointer) => self.append(Expression::Load { pointer }, span)?,
            Reference::Constant(Some(handle)) => self.append(Expression::Constant(handle), span)?,
            Reference::Constant(None) => {
                // Unselected branches can reference the variable being initialized.
                let ty = self.type_handle(&reference.ty, span);
                self.append(Expression::ZeroValue(ty), span)?
            }
            Reference::Conditional(_) | Reference::Bound(_) => {
                return self.rvalue(ExpressionValue::Reference(reference))
            }
        };
        Ok(Value {
            handle,
            ty: reference.ty,
            span,
            constant: ConstantValue::evaluated(reference.constant.literal),
        })
    }

    pub(super) fn load_pointer(
        &mut self,
        pointer: Handle<Expression>,
        ty: &TypeResolution,
        span: Span,
    ) -> Result<Value, ParseError> {
        let handle = self.append(Expression::Load { pointer }, span)?;
        Ok(Value {
            handle,
            ty: ty.clone(),
            span,
            constant: ConstantValue::default(),
        })
    }

    pub(super) fn discard_result(&mut self, result: ExpressionResult) -> Result<(), ParseError> {
        if let ExpressionResult::Value(ExpressionValue::Reference(reference)) = result {
            self.consume_reference(reference, &mut |_, _| Ok(()))?;
        }
        Ok(())
    }

    pub(super) fn decorate_reference(
        &mut self,
        reference: &mut ReferenceValue,
        action: &mut impl FnMut(
            &mut Self,
            Handle<Expression>,
            &TypeResolution,
            Span,
        ) -> Result<(), ParseError>,
    ) -> Result<(), ParseError> {
        let handle = match reference.reference {
            Reference::Pointer(pointer) => {
                return action(self, pointer, &reference.ty, reference.span);
            }
            Reference::Bound(handle) => {
                return self.consume_bound_reference(
                    handle,
                    reference.clone(),
                    &mut |parser, source| {
                        let Reference::Pointer(pointer) = source.reference else {
                            unreachable!()
                        };
                        action(parser, pointer, &source.ty, source.span)
                    },
                );
            }
            Reference::Conditional(handle) => handle,
            Reference::Constant(_) => {
                return Err(ParseError {
                    message: String::from("cannot modify a const value"),
                    span: reference.span,
                })
            }
        };
        let mut conditional = self.function.conditional_references[handle].take().unwrap();
        for branch in [&mut conditional.accept, &mut conditional.reject] {
            let ((), body) = self.capture_block(|parser| {
                parser.append_block(core::mem::take(&mut branch.body));
                parser.decorate_reference(&mut branch.reference, action)
            })?;
            branch.body = body;
        }
        self.function.conditional_references[handle] = Some(conditional);
        Ok(())
    }

    pub(super) fn consume_reference(
        &mut self,
        reference: ReferenceValue,
        action: &mut impl FnMut(&mut Self, ReferenceValue) -> Result<(), ParseError>,
    ) -> Result<(), ParseError> {
        let handle = match reference.reference {
            Reference::Pointer(_) | Reference::Constant(_) => {
                return action(self, reference);
            }
            Reference::Bound(handle) => {
                return self.consume_bound_reference(handle, reference, action)
            }
            Reference::Conditional(handle) => handle,
        };
        let conditional = self.function.conditional_references[handle].take().unwrap();
        let ((), accept) = self.capture_block(|parser| {
            parser.append_block(conditional.accept.body);
            parser.consume_reference(conditional.accept.reference, action)
        })?;
        let ((), reject) = self.capture_block(|parser| {
            parser.append_block(conditional.reject.body);
            parser.consume_reference(conditional.reject.reference, action)
        })?;
        self.push_statement(
            Statement::If {
                condition: conditional.condition,
                accept,
                reject,
            },
            reference.span,
        );
        Ok(())
    }
}
