use super::*;

fn remapping_error(span: Span) -> ParseError {
    ParseError {
        message: String::from("invalid CUDA pointer parameter mapping"),
        span,
    }
}

impl Parser<'_> {
    fn remap_pointer_snapshot(
        &mut self,
        source: Handle<Expression>,
        mapper: &mut impl PointerRemapper,
        span: Span,
    ) -> Result<Handle<Expression>, ParseError> {
        match mapper.expression(self, source)? {
            MappedExpression::Value(pointer) => Ok(pointer),
            _ => Err(remapping_error(span)),
        }
    }

    fn remap_pointer_expression(
        &mut self,
        source: &FunctionParameterState,
        handle: Handle<PointerExpression>,
        mapper: &mut impl PointerRemapper,
        depth: u32,
        span: Span,
    ) -> Result<Handle<PointerExpression>, ParseError> {
        self.parameter_expansion_budget.charge_work(1, span)?;
        if depth > 64 {
            return Err(ParseError {
                message: String::from("CUDA pointer expression nesting limit exceeded"),
                span,
            });
        }
        let expression = source
            .pointers
            .expressions
            .try_get(handle)
            .map_err(|_| remapping_error(span))?;
        let kind = match expression.kind {
            PointerExpressionKind::Parameter(index) => {
                let actual = mapper.parameter(index)?;
                if self.function.pointers.expressions[actual.expression].pointee
                    != expression.pointee
                {
                    return Err(remapping_error(span));
                }
                return Ok(actual.expression);
            }
            PointerExpressionKind::Array(binding) => PointerExpressionKind::Array(
                self.remap_bound_binding(source, binding, mapper, span)?,
            ),
            PointerExpressionKind::Address(binding) => PointerExpressionKind::Address(
                self.remap_bound_binding(source, binding, mapper, span)?,
            ),
            PointerExpressionKind::Offset {
                base,
                offset,
                subtract,
            } => {
                let base = self.remap_pointer_expression(source, base, mapper, depth + 1, span)?;
                let offset = self.remap_pointer_snapshot(offset, mapper, span)?;
                PointerExpressionKind::Offset {
                    base,
                    offset,
                    subtract,
                }
            }
            PointerExpressionKind::Select {
                selector,
                accept,
                reject,
            } => {
                let selector = self.remap_pointer_snapshot(selector, mapper, span)?;
                let accept =
                    self.remap_pointer_expression(source, accept, mapper, depth + 1, span)?;
                let reject =
                    self.remap_pointer_expression(source, reject, mapper, depth + 1, span)?;
                PointerExpressionKind::Select {
                    selector,
                    accept,
                    reject,
                }
            }
        };
        self.retain_pointer_expression(kind, expression.pointee, span)
    }

    pub(in super::super) fn remap_pointer_value(
        &mut self,
        source: &FunctionParameterState,
        value: &PointerValue,
        mapper: &mut impl PointerRemapper,
    ) -> Result<PointerValue, ParseError> {
        let expression =
            self.remap_pointer_expression(source, value.expression, mapper, 1, value.span)?;
        Ok(PointerValue {
            expression,
            ..value.clone()
        })
    }

    pub(in super::super) fn remap_pointer_access(
        &mut self,
        source: &FunctionParameterState,
        handle: Handle<PointerAccess>,
        mapper: &mut impl PointerRemapper,
    ) -> Result<ReferenceValue, ParseError> {
        let access = source
            .pointers
            .accesses
            .try_get(handle)
            .map_err(|_| remapping_error(Span::default()))?;
        let span = source.pointers.accesses.get_span(handle);
        let pointer = self.remap_pointer_value(source, &access.pointer, mapper)?;
        self.parameter_expansion_budget
            .charge_slice::<AddressProjection>(access.projections.len(), span)?;
        let mut projections = Vec::with_capacity(access.projections.len());
        for projection in &access.projections {
            self.parameter_expansion_budget.charge_work(1, span)?;
            projections.push(match *projection {
                AddressProjection::Field(index) => AddressProjection::Field(index),
                AddressProjection::Index(captured) => {
                    AddressProjection::Index(self.remap_pointer_snapshot(captured, mapper, span)?)
                }
            });
        }
        let reference =
            self.retain_pointer_access(pointer, projections, access.ty.clone(), span)?;
        self.materialize_pointer_reference(reference)
    }

    fn remap_bound_address(
        &mut self,
        source: &FunctionParameterState,
        handle: Handle<BoundAddress>,
        mapper: &mut impl PointerRemapper,
        span: Span,
    ) -> Result<Handle<BoundReference>, ParseError> {
        let address = source
            .bound_addresses
            .try_get(handle)
            .map_err(|_| remapping_error(span))?;
        let mut binding = match mapper.expression(self, address.root)? {
            MappedExpression::Reference { binding, .. } => binding,
            MappedExpression::Value(root) => {
                let address = self.retain_bound_address(
                    BoundAddress {
                        root,
                        projections: Vec::new(),
                    },
                    span,
                )?;
                self.parameter_expansion_budget
                    .charge_slice::<BoundTarget>(1, span)?;
                self.retain_bound_reference(
                    None,
                    alloc::vec![BoundTarget {
                        selector: address.index() as u32,
                        address,
                    }],
                    span,
                )?
            }
            MappedExpression::Pointer(_) => return Err(remapping_error(span)),
        };
        for projection in &address.projections {
            self.parameter_expansion_budget.charge_work(1, span)?;
            let projection = match *projection {
                AddressProjection::Field(index) => AddressProjection::Field(index),
                AddressProjection::Index(captured) => {
                    AddressProjection::Index(self.remap_pointer_snapshot(captured, mapper, span)?)
                }
            };
            binding = self.project_bound_reference(binding, projection, span)?;
        }
        Ok(binding)
    }

    fn remap_bound_binding(
        &mut self,
        source: &FunctionParameterState,
        handle: Handle<BoundReference>,
        mapper: &mut impl PointerRemapper,
        span: Span,
    ) -> Result<Handle<BoundReference>, ParseError> {
        let reference = source
            .bound_references
            .try_get(handle)
            .map_err(|_| remapping_error(span))?;
        self.parameter_expansion_budget
            .charge_work(reference.targets.len() + 1, span)?;
        let Some(selector) = reference.selector else {
            if reference.targets.len() != 1 {
                return Err(remapping_error(span));
            }
            return self.remap_bound_address(source, reference.targets[0].address, mapper, span);
        };
        if reference.targets.is_empty() {
            return Err(remapping_error(span));
        }
        let captured = self.remap_pointer_snapshot(selector, mapper, span)?;
        let selector = self.append(Expression::Load { pointer: captured }, span)?;
        let selected = self.temporary_pointer(&scalar_resolution(Scalar::U32), span)?;
        self.parameter_expansion_budget
            .charge_slice::<Handle<BoundReference>>(reference.targets.len(), span)?;
        let mut bindings = Vec::with_capacity(reference.targets.len());
        for target in &reference.targets {
            bindings.push(self.remap_bound_address(source, target.address, mapper, span)?);
        }
        self.parameter_expansion_budget
            .charge_slice::<crate::SwitchCase>(reference.targets.len(), span)?;
        self.parameter_expansion_budget
            .charge_nodes(reference.targets.len(), span)?;
        let mut cases = Vec::with_capacity(reference.targets.len());
        let mut addresses = Vec::new();
        for (index, (target, &binding)) in reference.targets.iter().zip(&bindings).enumerate() {
            let (_, body) = self.capture_block(|parser| {
                parser.with_bound_target(binding, span, &mut |parser, address| {
                    parser
                        .parameter_expansion_budget
                        .charge_slice::<Handle<BoundAddress>>(1, span)?;
                    addresses.push(address);
                    let value = parser.append(
                        Expression::Literal(Literal::U32(address.index() as u32)),
                        span,
                    )?;
                    parser.generated_pointer_statement(
                        Statement::Store {
                            pointer: selected,
                            value,
                        },
                        span,
                    )
                })
            })?;
            cases.push(crate::SwitchCase {
                value: if index + 1 == reference.targets.len() {
                    crate::SwitchValue::Default
                } else {
                    crate::SwitchValue::U32(target.selector)
                },
                body,
                fall_through: false,
            });
        }
        self.generated_pointer_statement(Statement::Switch { selector, cases }, span)?;
        let sorting_work = addresses
            .len()
            .saturating_mul((usize::BITS - addresses.len().leading_zeros()) as usize + 1);
        self.parameter_expansion_budget
            .charge_work(sorting_work, span)?;
        addresses.sort_unstable_by_key(|address| address.index());
        addresses.dedup();
        self.parameter_expansion_budget
            .charge_slice::<BoundTarget>(addresses.len(), span)?;
        let targets = addresses
            .into_iter()
            .map(|address| BoundTarget {
                selector: address.index() as u32,
                address,
            })
            .collect();
        self.retain_bound_reference(Some(selected), targets, span)
    }

    fn generated_pointer_statement(
        &mut self,
        statement: Statement,
        span: Span,
    ) -> Result<(), ParseError> {
        self.parameter_expansion_budget.charge_nodes(1, span)?;
        self.parameter_expansion_budget
            .charge_slice::<(Statement, Span)>(1, span)?;
        self.push_statement(statement, span);
        Ok(())
    }

    pub(in super::super) fn remap_bound_reference(
        &mut self,
        source: &FunctionParameterState,
        binding: Handle<BoundReference>,
        ty: &TypeResolution,
        mapper: &mut impl PointerRemapper,
        span: Span,
    ) -> Result<ReferenceValue, ParseError> {
        let binding = self.remap_bound_binding(source, binding, mapper, span)?;
        Ok(ReferenceValue {
            reference: Reference::Bound(binding),
            writable: true,
            ty: ty.clone(),
            span,
            constant: ConstantValue::default(),
        })
    }
}
