use super::*;

impl Parser<'_> {
    fn clone_mapped_expression(
        &mut self,
        expression: &MappedExpression,
        span: Span,
    ) -> Result<MappedExpression, ParseError> {
        if matches!(*expression, MappedExpression::Pointer(_)) {
            self.parameter_expansion_budget
                .charge_slice::<PointerValue>(1, span)?;
        }
        Ok(expression.clone())
    }

    pub(super) fn name_invocation_expression(
        &mut self,
        source: &Function,
        handle: Handle<Expression>,
        mapped: Handle<Expression>,
        span: Span,
    ) -> Result<(), ParseError> {
        if let Some(name) = source.named_expressions.get(&handle) {
            self.parameter_expansion_budget
                .charge_map_entry::<Handle<Expression>, String>(1, span)?;
            self.parameter_expansion_budget
                .charge_bytes(name.len(), span)?;
            self.function
                .ir
                .named_expressions
                .insert(mapped, name.clone());
        }
        Ok(())
    }

    pub(super) fn cache_invocation_expression(
        &mut self,
        invocation: &mut Invocation<'_>,
        handle: Handle<Expression>,
        mapped: MappedExpression,
    ) -> Result<MappedExpression, ParseError> {
        if invocation.expressions[handle.index()].is_some() {
            return Err(expansion_error(invocation.span));
        }
        self.parameter_expansion_budget
            .charge_work(1, invocation.span)?;
        self.parameter_expansion_budget
            .charge_slice::<Handle<Expression>>(1, invocation.span)?;
        invocation.mapped_handles.push(handle);
        let cached = self.clone_mapped_expression(&mapped, invocation.span)?;
        invocation.expressions[handle.index()] = Some(cached);
        Ok(mapped)
    }

    pub(super) fn restore_invocation_expressions(
        &mut self,
        invocation: &mut Invocation<'_>,
        checkpoint: usize,
    ) -> Result<(), ParseError> {
        while invocation.mapped_handles.len() > checkpoint {
            self.parameter_expansion_budget
                .charge_work(1, invocation.span)?;
            let handle = invocation.mapped_handles.pop().unwrap();
            invocation.expressions[handle.index()] = None;
        }
        Ok(())
    }

    pub(super) fn invocation_value(
        &mut self,
        invocation: &mut Invocation<'_>,
        handle: Handle<Expression>,
    ) -> Result<Handle<Expression>, ParseError> {
        match self.invocation_expression(invocation, handle)? {
            MappedExpression::Value(value) => Ok(value),
            MappedExpression::Reference { .. } | MappedExpression::Pointer(_) => {
                Err(expansion_error(invocation.span))
            }
        }
    }

    fn projected_type(
        &self,
        ty: &TypeResolution,
        index: Option<u32>,
        span: Span,
    ) -> Result<TypeResolution, ParseError> {
        match *ty.inner_with(&self.module.types) {
            TypeInner::Struct { ref members, .. } => index
                .and_then(|index| members.get(index as usize))
                .map(|member| TypeResolution::Handle(member.ty))
                .ok_or_else(|| expansion_error(span)),
            TypeInner::Array { base, .. } => Ok(TypeResolution::Handle(base)),
            TypeInner::Vector { scalar, .. } => Ok(scalar_resolution(scalar)),
            _ => Err(expansion_error(span)),
        }
    }

    pub(super) fn invocation_expression(
        &mut self,
        invocation: &mut Invocation<'_>,
        handle: Handle<Expression>,
    ) -> Result<MappedExpression, ParseError> {
        self.parameter_expansion_budget
            .charge_work(1, invocation.span)?;
        if let Some(ref mapped) = invocation.expressions[handle.index()] {
            return self.clone_mapped_expression(mapped, invocation.span);
        }
        let source = invocation.source;
        let span = source.expressions.get_span(handle);
        let expression = match source.expressions[handle] {
            Expression::FunctionArgument(index) => {
                let mapped =
                    self.clone_mapped_expression(&invocation.arguments[index as usize], span)?;
                return self.cache_invocation_expression(invocation, handle, mapped);
            }
            Expression::LocalVariable(local) => {
                if let Some(state) = invocation.source_state {
                    if let Some(&access) = state.pointers.validation_addresses.get(&local) {
                        let reference = self.remap_pointer_access(state, access, invocation)?;
                        let ty = reference.ty.clone();
                        let mapped = match reference.reference {
                            Reference::Pointer(pointer) => MappedExpression::Value(pointer),
                            _ => MappedExpression::Reference {
                                binding: self.freeze_reference(reference)?,
                                ty,
                            },
                        };
                        return self.cache_invocation_expression(invocation, handle, mapped);
                    }
                }
                Expression::LocalVariable(
                    invocation.locals[local.index()].ok_or_else(|| expansion_error(span))?,
                )
            }
            Expression::Literal(literal) => Expression::Literal(literal),
            Expression::Constant(constant) => Expression::Constant(constant),
            Expression::Override(value) => Expression::Override(value),
            Expression::ZeroValue(ty) => Expression::ZeroValue(ty),
            Expression::GlobalVariable(global) => Expression::GlobalVariable(global),
            Expression::Compose { ty, ref components } => {
                self.parameter_expansion_budget
                    .charge_slice::<Handle<Expression>>(components.len(), span)?;
                let mut mapped = Vec::with_capacity(components.len());
                for &component in components {
                    mapped.push(self.invocation_value(invocation, component)?);
                }
                Expression::Compose {
                    ty,
                    components: mapped,
                }
            }
            Expression::AccessIndex { base, index } => {
                match self.invocation_expression(invocation, base)? {
                    MappedExpression::Reference { binding, ty } => {
                        let ty = self.projected_type(&ty, Some(index), span)?;
                        let binding = self.project_bound_reference(
                            binding,
                            reference_bindings::AddressProjection::Field(index),
                            span,
                        )?;
                        let mapped = MappedExpression::Reference { binding, ty };
                        return self.cache_invocation_expression(invocation, handle, mapped);
                    }
                    MappedExpression::Value(base) => Expression::AccessIndex { base, index },
                    MappedExpression::Pointer(_) => return Err(expansion_error(span)),
                }
            }
            Expression::Access { base, index } => {
                let base = self.invocation_expression(invocation, base)?;
                let index = self.invocation_value(invocation, index)?;
                match base {
                    MappedExpression::Reference { binding, ty } => {
                        let ty = self.projected_type(&ty, None, span)?;
                        let binding = self.project_bound_index(binding, index, span)?;
                        let mapped = MappedExpression::Reference { binding, ty };
                        return self.cache_invocation_expression(invocation, handle, mapped);
                    }
                    MappedExpression::Value(base) => Expression::Access { base, index },
                    MappedExpression::Pointer(_) => return Err(expansion_error(span)),
                }
            }
            Expression::Load { pointer } => {
                match self.invocation_expression(invocation, pointer)? {
                    MappedExpression::Reference { binding, ty } => {
                        let reference = ReferenceValue {
                            reference: Reference::Bound(binding),
                            writable: true,
                            ty,
                            span,
                            constant: ConstantValue::default(),
                        };
                        let value = self.rvalue(ExpressionValue::Reference(reference))?;
                        return self.cache_invocation_expression(
                            invocation,
                            handle,
                            MappedExpression::Value(value.handle),
                        );
                    }
                    MappedExpression::Value(pointer) => Expression::Load { pointer },
                    MappedExpression::Pointer(_) => return Err(expansion_error(span)),
                }
            }
            Expression::Unary { op, expr } => Expression::Unary {
                op,
                expr: self.invocation_value(invocation, expr)?,
            },
            Expression::Binary { op, left, right } => Expression::Binary {
                op,
                left: self.invocation_value(invocation, left)?,
                right: self.invocation_value(invocation, right)?,
            },
            Expression::Select {
                condition,
                accept,
                reject,
            } => Expression::Select {
                condition: self.invocation_value(invocation, condition)?,
                accept: self.invocation_value(invocation, accept)?,
                reject: self.invocation_value(invocation, reject)?,
            },
            Expression::Math {
                fun,
                arg,
                arg1,
                arg2,
                arg3,
            } => Expression::Math {
                fun,
                arg: self.invocation_value(invocation, arg)?,
                arg1: arg1
                    .map(|argument| self.invocation_value(invocation, argument))
                    .transpose()?,
                arg2: arg2
                    .map(|argument| self.invocation_value(invocation, argument))
                    .transpose()?,
                arg3: arg3
                    .map(|argument| self.invocation_value(invocation, argument))
                    .transpose()?,
            },
            Expression::Relational { fun, argument } => Expression::Relational {
                fun,
                argument: self.invocation_value(invocation, argument)?,
            },
            Expression::As {
                expr,
                kind,
                convert,
            } => Expression::As {
                expr: self.invocation_value(invocation, expr)?,
                kind,
                convert,
            },
            Expression::Splat { size, value } => Expression::Splat {
                size,
                value: self.invocation_value(invocation, value)?,
            },
            Expression::Swizzle {
                size,
                vector,
                pattern,
            } => Expression::Swizzle {
                size,
                vector: self.invocation_value(invocation, vector)?,
                pattern,
            },
            Expression::CallResult(_) => return Err(expansion_error(span)),
            _ => return Err(expansion_error(span)),
        };
        let value = self.append(expression, span)?;
        let mapped =
            self.cache_invocation_expression(invocation, handle, MappedExpression::Value(value))?;
        self.name_invocation_expression(source, handle, value, span)?;
        Ok(mapped)
    }
}
