use super::parameter_calls::MappedExpression;
use super::reference_bindings::{AddressProjection, BoundTarget};
use super::*;

mod remapping;
mod validation;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum PointerValueCategory {
    Variable,
    Value,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct ConstantPointerAddress {
    pub(super) extent: u32,
    pub(super) offset: i64,
}

#[derive(Clone, Debug)]
pub(super) struct PointerValue {
    pub(super) expression: Handle<PointerExpression>,
    pub(super) ty: TypeResolution,
    pub(super) is_const: bool,
    pub(super) span: Span,
    pub(super) constant: ConstantValue,
    pub(super) constant_address: Option<ConstantPointerAddress>,
    pub(super) category: PointerValueCategory,
}

#[derive(Clone, Debug)]
pub(super) struct PointerExpression {
    pub(super) kind: PointerExpressionKind,
    pub(super) pointee: Handle<Type>,
    pub(super) depth: u32,
}

#[derive(Clone, Debug)]
pub(super) enum PointerExpressionKind {
    Parameter(u32),
    Array(Handle<BoundReference>),
    Address(Handle<BoundReference>),
    Offset {
        base: Handle<PointerExpression>,
        offset: Handle<Expression>,
        subtract: bool,
    },
    Select {
        selector: Handle<Expression>,
        accept: Handle<PointerExpression>,
        reject: Handle<PointerExpression>,
    },
}

#[derive(Clone, Debug)]
pub(super) struct PointerAccess {
    pub(super) pointer: PointerValue,
    pub(super) projections: Vec<AddressProjection>,
    pub(super) ty: TypeResolution,
}

#[derive(Clone, Default)]
pub(super) struct PointerState {
    pub(super) expressions: Arena<PointerExpression>,
    pub(super) accesses: Arena<PointerAccess>,
    pub(super) validation_addresses: FastHashMap<Handle<LocalVariable>, Handle<PointerAccess>>,
    pub(super) access_addresses: FastHashMap<Handle<PointerAccess>, Handle<LocalVariable>>,
}

impl PointerState {
    pub(super) const fn is_empty(&self) -> bool {
        self.expressions.is_empty() && self.accesses.is_empty()
    }
}

pub(super) trait PointerRemapper {
    fn expression(
        &mut self,
        parser: &mut Parser<'_>,
        source: Handle<Expression>,
    ) -> Result<MappedExpression, ParseError>;
    fn parameter(&self, index: u32) -> Result<PointerValue, ParseError>;
}

impl<'a> Parser<'a> {
    pub(super) fn local_pointer_declaration(
        &mut self,
        specifier: TypeSpecifier,
        is_const: bool,
    ) -> Result<Span, ParseError> {
        self.take_ident("const")?;
        let (name, span) = self.identifier()?;
        self.check_name(name, span)?;
        self.insert_symbol(name, Symbol::PendingPointer, span)?;
        let pointee = self.lower_type(specifier, span);
        let pointee = self.type_handle(&pointee, span);
        let ty = self.pointer_type(pointee, span);
        let has_equals = self.lexer.next_if(Token::Operation('='))?;
        let has_braces = self.lexer.next_if(Token::Paren('{'))?;
        if !has_equals && !has_braces {
            return self.fail("pointer declaration requires an initializer");
        }
        let value = self.expression(0)?;
        if has_braces {
            self.lexer.next_if(Token::Separator(','))?;
            self.lexer.expect_span(Token::Paren('}'))?;
        }
        let value_span = value.span();
        let mut pointer = self.convert_pointer(value, &ty, is_const, value_span)?;
        pointer.span = span;
        pointer.category = PointerValueCategory::Variable;
        pointer.constant = ConstantValue {
            literal: None,
            is_discardable: true,
        };
        pointer.constant_address = None;
        let pointer = self.retain_pointer_symbol(pointer)?;
        self.function
            .symbol_table
            .add(name, Symbol::Pointer(pointer));
        Ok(span)
    }

    pub(super) fn boxed_pointer_value(
        &mut self,
        pointer: PointerValue,
    ) -> Result<Box<PointerValue>, ParseError> {
        self.parameter_expansion_budget
            .charge_slice::<PointerValue>(1, pointer.span)?;
        Ok(Box::new(pointer))
    }

    pub(super) fn pointer_result(
        &mut self,
        pointer: PointerValue,
    ) -> Result<ExpressionResult, ParseError> {
        let pointer = self.boxed_pointer_value(pointer)?;
        Ok(ExpressionResult::Value(ExpressionValue::Pointer(pointer)))
    }

    pub(super) fn retain_pointer_symbol(
        &mut self,
        pointer: PointerValue,
    ) -> Result<Handle<PointerValue>, ParseError> {
        let span = pointer.span;
        self.parameter_expansion_budget.charge_nodes(1, span)?;
        self.parameter_expansion_budget
            .charge_slice::<(PointerValue, Span)>(1, span)?;
        Ok(self.function.pointer_symbols.append(pointer, span))
    }

    pub(super) fn pointer_symbol_value(
        &mut self,
        symbol: Handle<PointerValue>,
        span: Span,
    ) -> Result<ExpressionResult, ParseError> {
        let mut pointer = self.function.pointer_symbols[symbol].clone();
        pointer.span = span;
        self.pointer_result(pointer)
    }

    pub(super) fn borrowed_array_value(
        &mut self,
        pointer: Handle<Expression>,
        writable: bool,
        span: Span,
    ) -> Result<ExpressionResult, ParseError> {
        let array_ty = match self.function.ir.expressions[pointer] {
            Expression::LocalVariable(local) => self.function.ir.local_variables[local].ty,
            Expression::GlobalVariable(global) => self.module.global_variables[global].ty,
            _ => unreachable!(),
        };
        let constant = self.pointer_constant(pointer);
        let reference = ReferenceValue {
            reference: Reference::Pointer(pointer),
            writable,
            ty: TypeResolution::Handle(array_ty),
            span,
            constant,
        };
        if matches!(self.function.ir.expressions[pointer], Expression::GlobalVariable(global) if matches!(self.module.global_variables[global].space, AddressSpace::Storage { .. }))
        {
            let mut pointer = self.decay_array(reference)?;
            pointer.category = PointerValueCategory::Variable;
            pointer.constant.is_discardable = true;
            return self.pointer_result(pointer);
        }
        Ok(ExpressionResult::Value(ExpressionValue::Reference(
            reference,
        )))
    }

    pub(super) fn pointer_operator(
        &mut self,
        token: Token<'_>,
        value: ExpressionValue,
        operator_span: Span,
    ) -> Result<ExpressionResult, ParseError> {
        match token {
            Token::Operation('&') => {
                let span = operator_span.until(&value.span());
                let pointer = self.address_of(value, span)?;
                self.pointer_result(pointer)
            }
            Token::Operation('*') => {
                let pointer = self.borrowed_pointer(value)?;
                let reference = self.pointer_reference(pointer)?;
                Ok(ExpressionResult::Value(ExpressionValue::Reference(
                    reference,
                )))
            }
            Token::Operation('+') => {
                let pointer = self.borrowed_pointer(value)?;
                self.pointer_result(pointer)
            }
            _ => unreachable!(),
        }
    }

    pub(super) fn array_parameter(
        &mut self,
        name: Option<&'a str>,
        span: Span,
        ty: &TypeResolution,
        is_const: bool,
        mode: ParameterMode,
    ) -> Result<(), ParseError> {
        if mode != ParameterMode::Value {
            return Err(ParseError {
                message: String::from("array reference parameters are unsupported"),
                span,
            });
        }
        self.lexer.expect_span(Token::Paren('['))?;
        if self.lexer.peek()?.0 != Token::Paren(']') {
            self.array_size()?;
        }
        self.lexer.expect_span(Token::Paren(']'))?;
        if self.lexer.peek()?.0 == Token::Paren('[') {
            return self.fail("multidimensional array parameters are unsupported");
        }
        self.activate_parameter_expansion(span)?;
        let pointee = self.type_handle(ty, span);
        let pointer_ty = self.pointer_type(pointee, span);
        self.device_parameter(
            name,
            span,
            &pointer_ty,
            false,
            ParameterMode::Pointer { is_const },
        )
    }

    pub(super) fn pointer_index(
        &mut self,
        value: ExpressionValue,
    ) -> Result<ExpressionValue, ParseError> {
        let span = value.span();
        let left_pointer = self.is_pointer_operand(&value);
        let (pointer, index) = if left_pointer {
            let pointer = self.borrowed_pointer(value)?;
            self.lexer.expect_span(Token::Paren('['))?;
            let index = self.expression(0)?;
            (pointer, self.integral_operand(index)?)
        } else {
            let index = self.integral_operand(value)?;
            self.lexer.expect_span(Token::Paren('['))?;
            let pointer = self.expression(0)?;
            (self.borrowed_pointer(pointer)?, index)
        };
        let end = self.lexer.expect_span(Token::Paren(']'))?;
        let pointer = self.offset_pointer(pointer, index, false, span.until(&end))?;
        self.pointer_reference(pointer)
            .map(ExpressionValue::Reference)
    }

    pub(super) fn pointer_binary(
        &mut self,
        left: ExpressionValue,
        right: ExpressionValue,
        right_body: crate::Block,
        operator: BinaryOperator,
    ) -> Result<ExpressionResult, ParseError> {
        let span = left.span().until(&right.span());
        let left_pointer = self.is_pointer_operand(&left);
        let right_pointer = self.is_pointer_operand(&right);
        if !matches!(operator, BinaryOperator::Add | BinaryOperator::Subtract)
            || (left_pointer && right_pointer)
            || (operator == BinaryOperator::Subtract && right_pointer)
        {
            return Err(ParseError {
                message: String::from("unsupported CUDA pointer arithmetic"),
                span,
            });
        }
        let pointer = if left_pointer {
            let pointer = self.borrowed_pointer(left)?;
            self.append_block(right_body);
            let offset = self.integral_operand(right)?;
            self.offset_pointer(pointer, offset, operator == BinaryOperator::Subtract, span)?
        } else {
            let offset = self.integral_operand(left)?;
            self.append_block(right_body);
            let pointer = self.borrowed_pointer(right)?;
            self.offset_pointer(pointer, offset, false, span)?
        };
        self.pointer_result(pointer)
    }

    pub(super) fn pointer_type(&mut self, pointee: Handle<Type>, span: Span) -> TypeResolution {
        TypeResolution::Handle(self.module.types.insert(
            Type {
                name: None,
                inner: TypeInner::Pointer {
                    base: pointee,
                    space: AddressSpace::Function,
                },
            },
            span,
        ))
    }

    pub(super) fn retain_pointer_expression(
        &mut self,
        kind: PointerExpressionKind,
        pointee: Handle<Type>,
        span: Span,
    ) -> Result<Handle<PointerExpression>, ParseError> {
        self.activate_parameter_expansion(span)?;
        self.prepare_parameter_bindings(span)?;
        let depth = match kind {
            PointerExpressionKind::Offset { base, .. } => {
                self.function.pointers.expressions[base].depth + 1
            }
            PointerExpressionKind::Select { accept, reject, .. } => {
                1 + self.function.pointers.expressions[accept]
                    .depth
                    .max(self.function.pointers.expressions[reject].depth)
            }
            _ => 1,
        };
        if depth > 64 {
            return Err(ParseError {
                message: String::from("CUDA pointer expression nesting limit exceeded"),
                span,
            });
        }
        self.parameter_expansion_budget.charge_nodes(1, span)?;
        self.parameter_expansion_budget
            .charge_slice::<(PointerExpression, Span)>(1, span)?;
        Ok(self.function.pointers.expressions.append(
            PointerExpression {
                kind,
                pointee,
                depth,
            },
            span,
        ))
    }

    pub(super) fn parameter_pointer(
        &mut self,
        index: u32,
        ty: &TypeResolution,
        is_const: bool,
        span: Span,
    ) -> Result<PointerValue, ParseError> {
        let TypeInner::Pointer { base, .. } = *ty.inner_with(&self.module.types) else {
            return Err(ParseError {
                message: String::from("expected a CUDA source pointer type"),
                span,
            });
        };
        let expression =
            self.retain_pointer_expression(PointerExpressionKind::Parameter(index), base, span)?;
        Ok(PointerValue {
            expression,
            ty: ty.clone(),
            is_const,
            span,
            constant: ConstantValue {
                literal: None,
                is_discardable: true,
            },
            constant_address: None,
            category: PointerValueCategory::Variable,
        })
    }

    pub(super) fn pointer_rvalue(&self, mut pointer: PointerValue) -> PointerValue {
        if pointer.category == PointerValueCategory::Variable {
            pointer.constant = ConstantValue::default();
            pointer.constant_address = None;
            pointer.category = PointerValueCategory::Value;
        }
        pointer
    }

    pub(super) fn borrowed_pointer(
        &mut self,
        value: ExpressionValue,
    ) -> Result<PointerValue, ParseError> {
        match value {
            ExpressionValue::Pointer(pointer) => Ok(self.pointer_rvalue(*pointer)),
            ExpressionValue::Reference(reference)
                if matches!(
                    *reference.ty.inner_with(&self.module.types),
                    TypeInner::Array { .. }
                ) =>
            {
                self.decay_array(reference)
            }
            value => Err(ParseError {
                message: String::from("expected a borrowed CUDA pointer or array lvalue"),
                span: value.span(),
            }),
        }
    }

    pub(super) fn is_pointer_operand(&self, value: &ExpressionValue) -> bool {
        matches!(value, ExpressionValue::Pointer(_))
            || matches!(
                *value.ty().inner_with(&self.module.types),
                TypeInner::Array { .. }
            )
    }

    pub(super) fn decay_array(
        &mut self,
        reference: ReferenceValue,
    ) -> Result<PointerValue, ParseError> {
        let TypeInner::Array { base, .. } = *reference.ty.inner_with(&self.module.types) else {
            return Err(ParseError {
                message: String::from("pointer decay requires an array lvalue"),
                span: reference.span,
            });
        };
        let span = reference.span;
        let is_const = !reference.writable;
        let constant_address = if self.reference_has_static_address(&reference)? {
            match *reference.ty.inner_with(&self.module.types) {
                TypeInner::Array {
                    size: ArraySize::Constant(extent),
                    ..
                } => Some(ConstantPointerAddress {
                    extent: extent.get(),
                    offset: 0,
                }),
                _ => None,
            }
        } else {
            None
        };
        let constant = ConstantValue {
            literal: None,
            is_discardable: constant_address.is_some(),
        };
        let binding = self.freeze_reference(reference)?;
        let expression =
            self.retain_pointer_expression(PointerExpressionKind::Array(binding), base, span)?;
        let ty = self.pointer_type(base, span);
        Ok(PointerValue {
            expression,
            ty,
            is_const,
            span,
            constant,
            constant_address,
            category: PointerValueCategory::Value,
        })
    }

    pub(super) fn address_of(
        &mut self,
        value: ExpressionValue,
        span: Span,
    ) -> Result<PointerValue, ParseError> {
        let ExpressionValue::Reference(reference) = value else {
            return Err(ParseError {
                message: String::from("address-of requires an object lvalue"),
                span,
            });
        };
        if let Reference::PointerAccess(access) = reference.reference {
            self.charge_pointer_access_clone(access, span)?;
            let access = self.function.pointers.accesses[access].clone();
            if access.projections.is_empty() {
                let mut pointer = access.pointer;
                pointer.span = span;
                return Ok(pointer);
            }
        }
        let pointee = self.type_handle(&reference.ty, span);
        let is_const = !reference.writable;
        let constant_address =
            self.reference_has_static_address(&reference)?
                .then_some(ConstantPointerAddress {
                    extent: 1,
                    offset: 0,
                });
        let constant = ConstantValue {
            literal: None,
            is_discardable: constant_address.is_some(),
        };
        let binding = self.freeze_reference(reference)?;
        let expression =
            self.retain_pointer_expression(PointerExpressionKind::Address(binding), pointee, span)?;
        let ty = self.pointer_type(pointee, span);
        Ok(PointerValue {
            expression,
            ty,
            is_const,
            span,
            constant,
            constant_address,
            category: PointerValueCategory::Value,
        })
    }

    pub(super) fn convert_pointer(
        &mut self,
        value: ExpressionValue,
        target: &TypeResolution,
        is_const: bool,
        span: Span,
    ) -> Result<PointerValue, ParseError> {
        let mut pointer = self.borrowed_pointer(value)?;
        if !self.module.compare_types(&pointer.ty, target) || (pointer.is_const && !is_const) {
            return Err(ParseError {
                message: String::from("incompatible CUDA pointer type"),
                span,
            });
        }
        pointer.is_const = is_const;
        Ok(pointer)
    }

    pub(super) fn offset_pointer(
        &mut self,
        pointer: PointerValue,
        offset: Value,
        subtract: bool,
        span: Span,
    ) -> Result<PointerValue, ParseError> {
        let mut pointer = self.pointer_rvalue(pointer);
        pointer.constant_address = pointer.constant_address.and_then(|address| {
            let delta = match offset.constant.literal {
                Some(Literal::I32(value)) => i64::from(value),
                Some(Literal::U32(value)) => i64::from(value),
                _ => return None,
            };
            let position = if subtract {
                address.offset.checked_sub(delta)
            } else {
                address.offset.checked_add(delta)
            }?;
            (position >= 0 && position <= i64::from(address.extent)).then_some(
                ConstantPointerAddress {
                    offset: position,
                    ..address
                },
            )
        });
        let captured = self.freeze_reference_index(offset.handle, offset.span)?;
        let pointee = self.function.pointers.expressions[pointer.expression].pointee;
        pointer.expression = self.retain_pointer_expression(
            PointerExpressionKind::Offset {
                base: pointer.expression,
                offset: captured,
                subtract,
            },
            pointee,
            span,
        )?;
        pointer.span = span;
        pointer.constant = ConstantValue {
            literal: None,
            is_discardable: pointer.constant.is_discardable
                && offset.constant.is_discardable
                && pointer.constant_address.is_some(),
        };
        Ok(pointer)
    }

    pub(super) fn retain_pointer_access(
        &mut self,
        pointer: PointerValue,
        projections: Vec<AddressProjection>,
        ty: TypeResolution,
        span: Span,
    ) -> Result<ReferenceValue, ParseError> {
        self.parameter_expansion_budget.charge_nodes(1, span)?;
        self.parameter_expansion_budget
            .charge_slice::<(PointerAccess, Span)>(1, span)?;
        self.parameter_expansion_budget
            .charge_slice::<AddressProjection>(projections.len(), span)?;
        let writable = !pointer.is_const;
        let constant = ConstantValue {
            literal: None,
            is_discardable: pointer.constant.is_discardable
                && pointer
                    .constant_address
                    .is_some_and(|address| address.offset < i64::from(address.extent)),
        };
        let access = self.function.pointers.accesses.append(
            PointerAccess {
                pointer,
                projections,
                ty: ty.clone(),
            },
            span,
        );
        Ok(ReferenceValue {
            reference: Reference::PointerAccess(access),
            writable,
            ty,
            span,
            constant,
        })
    }

    pub(super) fn pointer_reference(
        &mut self,
        pointer: PointerValue,
    ) -> Result<ReferenceValue, ParseError> {
        let pointer = self.pointer_rvalue(pointer);
        let pointee = self.function.pointers.expressions[pointer.expression].pointee;
        let span = pointer.span;
        let discardable = pointer.constant.is_discardable
            && pointer
                .constant_address
                .is_some_and(|address| address.offset < i64::from(address.extent));
        let mut reference =
            self.retain_pointer_access(pointer, Vec::new(), TypeResolution::Handle(pointee), span)?;
        reference.constant.is_discardable = discardable;
        Ok(reference)
    }

    pub(super) fn project_pointer_access(
        &mut self,
        access: Handle<PointerAccess>,
        projection: AddressProjection,
        ty: &TypeResolution,
        constant: ConstantValue,
        span: Span,
    ) -> Result<ReferenceValue, ParseError> {
        self.charge_pointer_access_clone(access, span)?;
        let access = self.function.pointers.accesses[access].clone();
        let mut pointer = access.pointer;
        pointer.constant = pointer.constant.after(constant);
        let mut projections = access.projections;
        projections.push(projection);
        let mut reference = self.retain_pointer_access(pointer, projections, ty.clone(), span)?;
        reference.constant = constant;
        Ok(reference)
    }

    pub(super) fn sequence_value(
        &mut self,
        value: &mut ExpressionValue,
        earlier: ConstantValue,
    ) -> Result<(), ParseError> {
        match *value {
            ExpressionValue::Plain(ref mut value) => value.constant = value.constant.after(earlier),
            ExpressionValue::Pointer(ref mut pointer) => {
                pointer.constant = pointer.constant.after(earlier)
            }
            ExpressionValue::Reference(ref mut reference) => {
                if !earlier.is_discardable {
                    if let Reference::PointerAccess(access) = reference.reference {
                        self.charge_pointer_access_clone(access, reference.span)?;
                        let access = self.function.pointers.accesses[access].clone();
                        let mut pointer = access.pointer;
                        pointer.constant = pointer.constant.after(earlier);
                        let sequenced = self.retain_pointer_access(
                            pointer,
                            access.projections,
                            access.ty,
                            reference.span,
                        )?;
                        reference.reference = sequenced.reference;
                    }
                }
                reference.constant = reference.constant.after(earlier);
            }
        }
        Ok(())
    }

    pub(super) fn validation_pointer(
        &mut self,
        access: Handle<PointerAccess>,
        span: Span,
    ) -> Result<Handle<Expression>, ParseError> {
        if let Some(&local) = self.function.pointers.access_addresses.get(&access) {
            return self.append(Expression::LocalVariable(local), span);
        }
        let ty = self.function.pointers.accesses[access].ty.clone();
        let pointer = self.temporary_pointer(&ty, span)?;
        let Expression::LocalVariable(local) = self.function.ir.expressions[pointer] else {
            unreachable!()
        };
        self.parameter_expansion_budget
            .charge_map_entry::<Handle<LocalVariable>, Handle<PointerAccess>>(2, span)?;
        self.function
            .pointers
            .validation_addresses
            .insert(local, access);
        self.function
            .pointers
            .access_addresses
            .insert(access, local);
        Ok(pointer)
    }

    pub(super) fn select_pointer(
        &mut self,
        condition: Value,
        accept: ExpressionValue,
        accept_body: crate::Block,
        reject: ExpressionValue,
        reject_body: crate::Block,
        span: Span,
    ) -> Result<PointerValue, ParseError> {
        self.activate_parameter_expansion(span)?;
        self.prepare_parameter_bindings(span)?;
        let variable = matches!((&accept, &reject), (ExpressionValue::Pointer(accept), ExpressionValue::Pointer(reject)) if accept.category == PointerValueCategory::Variable && reject.category == PointerValueCategory::Variable && accept.is_const == reject.is_const && self.module.compare_types(&accept.ty, &reject.ty));
        let selector = self.temporary_pointer(&scalar_resolution(Scalar::U32), span)?;
        let branch = |parser: &mut Self, value, body, tag| {
            parser.capture_block(|parser| {
                parser.append_block(body);
                let pointer = if variable {
                    let ExpressionValue::Pointer(pointer) = value else {
                        unreachable!()
                    };
                    *pointer
                } else {
                    parser.borrowed_pointer(value)?
                };
                let value = parser.append(Expression::Literal(Literal::U32(tag)), span)?;
                parser.parameter_expansion_budget.charge_nodes(1, span)?;
                parser
                    .parameter_expansion_budget
                    .charge_slice::<(Statement, Span)>(1, span)?;
                parser.push_statement(
                    Statement::Store {
                        pointer: selector,
                        value,
                    },
                    span,
                );
                Ok(pointer)
            })
        };
        let (accept, accept_body) = branch(self, accept, accept_body, 1)?;
        let (reject, reject_body) = branch(self, reject, reject_body, 0)?;
        if !self.module.compare_types(&accept.ty, &reject.ty) {
            return Err(ParseError {
                message: String::from("conditional pointer arms require the same pointee type"),
                span,
            });
        }
        let constant = match condition.constant.literal {
            Some(Literal::Bool(true)) => accept.constant,
            Some(Literal::Bool(false)) => reject.constant,
            _ => ConstantValue::default(),
        };
        let constant_address = match condition.constant.literal {
            Some(Literal::Bool(true)) => accept.constant_address,
            Some(Literal::Bool(false)) => reject.constant_address,
            _ => None,
        };
        self.parameter_expansion_budget.charge_nodes(1, span)?;
        self.parameter_expansion_budget
            .charge_slice::<(Statement, Span)>(1, span)?;
        self.push_statement(
            Statement::If {
                condition: condition.handle,
                accept: accept_body,
                reject: reject_body,
            },
            span,
        );
        let pointee = self.function.pointers.expressions[accept.expression].pointee;
        let expression = self.retain_pointer_expression(
            PointerExpressionKind::Select {
                selector,
                accept: accept.expression,
                reject: reject.expression,
            },
            pointee,
            span,
        )?;
        Ok(PointerValue {
            expression,
            ty: accept.ty,
            is_const: accept.is_const || reject.is_const,
            span,
            constant,
            constant_address,
            category: if variable {
                PointerValueCategory::Variable
            } else {
                PointerValueCategory::Value
            },
        })
    }

    fn pointer_origin_is_known(
        &mut self,
        expression: Handle<PointerExpression>,
        span: Span,
    ) -> Result<bool, ParseError> {
        self.parameter_expansion_budget.charge_work(1, span)?;
        match self.function.pointers.expressions[expression].kind.clone() {
            PointerExpressionKind::Parameter(_) => Ok(false),
            PointerExpressionKind::Offset { base, .. } => self.pointer_origin_is_known(base, span),
            PointerExpressionKind::Select { accept, reject, .. } => Ok(self
                .pointer_origin_is_known(accept, span)?
                && self.pointer_origin_is_known(reject, span)?),
            PointerExpressionKind::Array(binding) | PointerExpressionKind::Address(binding) => {
                let view = self.bound_reference_view(binding, span)?;
                for target in view.targets {
                    let root = self.function.bound_addresses[target.address].root;
                    match self.function.ir.expressions[root] {
                        Expression::GlobalVariable(_) => {}
                        Expression::LocalVariable(local)
                            if !self
                                .function
                                .pointers
                                .validation_addresses
                                .contains_key(&local) => {}
                        _ => return Ok(false),
                    }
                }
                Ok(true)
            }
        }
    }

    pub(super) fn materialize_pointer_reference(
        &mut self,
        reference: ReferenceValue,
    ) -> Result<ReferenceValue, ParseError> {
        let Reference::PointerAccess(access) = reference.reference else {
            return Ok(reference);
        };
        self.charge_pointer_access_clone(access, reference.span)?;
        let access = self.function.pointers.accesses[access].clone();
        if !self.pointer_origin_is_known(access.pointer.expression, reference.span)? {
            return Ok(reference);
        }
        let root =
            self.resolve_pointer_expression(access.pointer.expression, None, reference.span)?;
        let mut binding = self.freeze_reference(root)?;
        for projection in access.projections {
            binding = self.project_bound_reference(binding, projection, reference.span)?;
        }
        Ok(ReferenceValue {
            reference: Reference::Bound(binding),
            ..reference
        })
    }

    fn address_root_type(&self, root: Handle<Expression>) -> Option<Handle<Type>> {
        match self.function.ir.expressions[root] {
            Expression::LocalVariable(local) => Some(self.function.ir.local_variables[local].ty),
            Expression::GlobalVariable(global) => Some(self.module.global_variables[global].ty),
            _ => None,
        }
    }

    fn projected_address_type(
        &mut self,
        ty: Handle<Type>,
        projection: &AddressProjection,
        span: Span,
    ) -> Result<Handle<Type>, ParseError> {
        match self.module.types[ty].inner {
            TypeInner::Array { base, .. } => Ok(base),
            TypeInner::Struct { ref members, .. } => match *projection {
                AddressProjection::Field(index) => members
                    .get(index as usize)
                    .map(|member| member.ty)
                    .ok_or_else(|| ParseError {
                        message: String::from("invalid CUDA address field"),
                        span,
                    }),
                AddressProjection::Index(_) => Err(ParseError {
                    message: String::from("invalid CUDA structure address index"),
                    span,
                }),
            },
            TypeInner::Vector { scalar, .. } => {
                Ok(self.type_handle(&scalar_resolution(scalar), span))
            }
            _ => Err(ParseError {
                message: String::from("invalid CUDA address projection"),
                span,
            }),
        }
    }

    fn resolve_pointer_expression(
        &mut self,
        expression: Handle<PointerExpression>,
        offset: Option<Handle<Expression>>,
        span: Span,
    ) -> Result<ReferenceValue, ParseError> {
        self.parameter_expansion_budget.charge_work(1, span)?;
        let source = self.function.pointers.expressions[expression].clone();
        let ty = TypeResolution::Handle(source.pointee);
        match source.kind {
            PointerExpressionKind::Parameter(_) => Err(ParseError {
                message: String::from("unresolved CUDA pointer parameter"),
                span,
            }),
            PointerExpressionKind::Offset {
                base,
                offset: captured,
                subtract,
            } => {
                let value = self.append(Expression::Load { pointer: captured }, span)?;
                let offset = if subtract {
                    let left = match offset {
                        Some(value) => value,
                        None => self.append(Expression::Literal(Literal::U32(0)), span)?,
                    };
                    self.append(
                        Expression::Binary {
                            op: BinaryOperator::Subtract,
                            left,
                            right: value,
                        },
                        span,
                    )?
                } else if let Some(left) = offset {
                    self.append(
                        Expression::Binary {
                            op: BinaryOperator::Add,
                            left,
                            right: value,
                        },
                        span,
                    )?
                } else {
                    value
                };
                self.resolve_pointer_expression(base, Some(offset), span)
            }
            PointerExpressionKind::Select {
                selector,
                accept,
                reject,
            } => {
                let selector = self.append(Expression::Load { pointer: selector }, span)?;
                let zero = self.append(Expression::Literal(Literal::U32(0)), span)?;
                let condition = self.append(
                    Expression::Binary {
                        op: BinaryOperator::NotEqual,
                        left: selector,
                        right: zero,
                    },
                    span,
                )?;
                let (accept, accept_body) = self.capture_block(|parser| {
                    parser.resolve_pointer_expression(accept, offset, span)
                })?;
                let (reject, reject_body) = self.capture_block(|parser| {
                    parser.resolve_pointer_expression(reject, offset, span)
                })?;
                self.parameter_expansion_budget.charge_nodes(1, span)?;
                self.parameter_expansion_budget
                    .charge_slice::<Option<ConditionalReference>>(1, span)?;
                let handle = self.function.conditional_references.append(
                    Some(ConditionalReference {
                        condition,
                        constant_condition: None,
                        accept: ReferenceBranch {
                            body: accept_body,
                            reference: accept,
                        },
                        reject: ReferenceBranch {
                            body: reject_body,
                            reference: reject,
                        },
                        depth: source.depth,
                    }),
                    span,
                );
                Ok(ReferenceValue {
                    reference: Reference::Conditional(handle),
                    writable: true,
                    ty,
                    span,
                    constant: ConstantValue::default(),
                })
            }
            kind @ (PointerExpressionKind::Array(_) | PointerExpressionKind::Address(_)) => {
                let is_array = matches!(kind, PointerExpressionKind::Array(_));
                let (PointerExpressionKind::Array(binding)
                | PointerExpressionKind::Address(binding)) = kind
                else {
                    unreachable!()
                };
                let view = self.bound_reference_view(binding, span)?;
                self.parameter_expansion_budget
                    .charge_slice::<BoundTarget>(view.targets.len(), span)?;
                let mut targets = Vec::with_capacity(view.targets.len());
                for target in view.targets {
                    let count = self.function.bound_addresses[target.address]
                        .projections
                        .len();
                    self.parameter_expansion_budget
                        .charge_work(count + 1, span)?;
                    self.parameter_expansion_budget
                        .charge_slice::<AddressProjection>(count + 1, span)?;
                    let mut address = self.function.bound_addresses[target.address].clone();
                    let mut parent_ty =
                        self.address_root_type(address.root)
                            .ok_or_else(|| ParseError {
                                message: String::from("unknown CUDA pointer origin"),
                                span,
                            })?;
                    let mut last_parent = None;
                    for projection in &address.projections {
                        last_parent = Some(parent_ty);
                        parent_ty = self.projected_address_type(parent_ty, projection, span)?;
                    }
                    let mut initial = None;
                    let has_array_origin = if is_array {
                        true
                    } else if last_parent.is_some_and(|parent| {
                        matches!(self.module.types[parent].inner, TypeInner::Array { .. })
                    }) {
                        initial = address.projections.pop();
                        true
                    } else {
                        false
                    };
                    if has_array_origin {
                        let initial = match initial {
                            Some(AddressProjection::Index(pointer)) => {
                                Some(self.append(Expression::Load { pointer }, span)?)
                            }
                            Some(AddressProjection::Field(index)) => {
                                Some(self.append(Expression::Literal(Literal::U32(index)), span)?)
                            }
                            None => None,
                        };
                        let index = match (initial, offset) {
                            (Some(left), Some(right)) => self.append(
                                Expression::Binary {
                                    op: BinaryOperator::Add,
                                    left,
                                    right,
                                },
                                span,
                            )?,
                            (Some(value), None) | (None, Some(value)) => value,
                            (None, None) => {
                                self.append(Expression::Literal(Literal::U32(0)), span)?
                            }
                        };
                        let index = self.freeze_reference_index(index, span)?;
                        address.projections.push(AddressProjection::Index(index));
                    }
                    let address = self.retain_bound_address(address, span)?;
                    targets.push(BoundTarget {
                        selector: target.selector,
                        address,
                    });
                }
                let binding = self.retain_bound_reference(view.selector, targets, span)?;
                Ok(ReferenceValue {
                    reference: Reference::Bound(binding),
                    writable: true,
                    ty,
                    span,
                    constant: ConstantValue::default(),
                })
            }
        }
    }

    fn charge_pointer_access_clone(
        &mut self,
        access: Handle<PointerAccess>,
        span: Span,
    ) -> Result<(), ParseError> {
        let count = self.function.pointers.accesses[access].projections.len();
        self.parameter_expansion_budget
            .charge_work(count + 1, span)?;
        self.parameter_expansion_budget
            .charge_slice::<AddressProjection>(count + 1, span)
    }
}
