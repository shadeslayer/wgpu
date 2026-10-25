use super::*;

use core::num::NonZeroU32;

impl Parser<'_> {
    pub(super) fn starts_list_initializer(&self) -> Result<bool, ParseError> {
        let mut lookahead = self.lexer.clone();
        lookahead.next_if(Token::Operation('='))?;
        Ok(lookahead.peek()?.0 == Token::Paren('{'))
    }

    pub(super) fn value_initializer(
        &mut self,
        target: &TypeResolution,
    ) -> Result<Value, ParseError> {
        let has_equals = self.lexer.next_if(Token::Operation('='))?;
        if self.lexer.peek()?.0 == Token::Paren('{') {
            let span = self.lexer.peek()?.1;
            let scalar = self.require_scalar_type(target, span)?;
            return self.scalar_list_initializer(scalar);
        }
        if !has_equals {
            self.lexer.expect_span(Token::Operation('='))?;
        }
        let value = self.expression(0)?;
        self.convert_value(value, target)
    }

    pub(super) fn aggregate_initializer(
        &mut self,
        pointer: Handle<Expression>,
        specifier: TypeSpecifier,
        actual_ty: Handle<Type>,
        array_count: Option<NonZeroU32>,
    ) -> Result<(), ParseError> {
        self.lexer.next_if(Token::Operation('='))?;
        let span = self.lexer.expect_span(Token::Paren('{'))?;
        if self.lexer.next_if(Token::Paren('}'))? {
            match self.module.types[actual_ty].inner {
                TypeInner::Array {
                    base,
                    size: ArraySize::Constant(count),
                    ..
                } if matches!(self.module.types[base].inner, TypeInner::Array { .. }) => {
                    self.zero_array_tail(pointer, base, 0, count, span)?;
                }
                _ => {
                    let value = self.append(Expression::ZeroValue(actual_ty), span)?;
                    self.push_statement(Statement::Store { pointer, value }, span);
                }
            }
            return Ok(());
        }

        let Some(array_count) = array_count else {
            if matches!(specifier, TypeSpecifier::Structure(_)) {
                return self.record_aggregate_initializer(pointer, actual_ty, true, span, None);
            }
            return self.vector_list_initializer(pointer, specifier, actual_ty, true, span);
        };
        let element_ty = match self.module.types[actual_ty].inner {
            TypeInner::Array { base, .. } => base,
            _ => return self.fail("expected an array initializer"),
        };
        let initialized_count = self.array_initializer_clauses(
            pointer,
            specifier,
            element_ty,
            Some(array_count),
            true,
            span,
        )?;
        let end = self.lexer.expect_span(Token::Paren('}'))?;
        if initialized_count < array_count.get() {
            self.zero_array_tail(
                pointer,
                element_ty,
                initialized_count,
                array_count,
                span.until(&end),
            )?;
        }
        Ok(())
    }

    pub(super) fn inferred_array_initializer(
        &mut self,
        pointer: Handle<Expression>,
        specifier: TypeSpecifier,
        element_ty: Handle<Type>,
    ) -> Result<NonZeroU32, ParseError> {
        self.lexer.next_if(Token::Operation('='))?;
        let span = self.lexer.expect_span(Token::Paren('{'))?;
        if self.lexer.peek()?.0 == Token::Paren('}') {
            return self.fail("cannot infer an array bound from an empty initializer list");
        }
        let count =
            self.array_initializer_clauses(pointer, specifier, element_ty, None, true, span)?;
        let end = self.lexer.expect_span(Token::Paren('}'))?;
        NonZeroU32::new(count).ok_or_else(|| ParseError {
            message: String::from("array initializer requires at least one element"),
            span: span.until(&end),
        })
    }

    fn array_initializer_clauses(
        &mut self,
        pointer: Handle<Expression>,
        specifier: TypeSpecifier,
        element_ty: Handle<Type>,
        known_count: Option<NonZeroU32>,
        has_braces: bool,
        span: Span,
    ) -> Result<u32, ParseError> {
        self.function.expression_depth += 1;
        let result = if self.function.expression_depth > 64 {
            self.fail("CUDA initializer nesting limit exceeded")
        } else {
            self.array_initializer_clauses_inner(
                pointer,
                specifier,
                element_ty,
                known_count,
                has_braces,
                span,
            )
        };
        self.function.expression_depth -= 1;
        result
    }

    fn array_initializer_clauses_inner(
        &mut self,
        pointer: Handle<Expression>,
        specifier: TypeSpecifier,
        element_ty: Handle<Type>,
        known_count: Option<NonZeroU32>,
        has_braces: bool,
        span: Span,
    ) -> Result<u32, ParseError> {
        let mut initialized_count = 0;
        loop {
            if known_count.is_some_and(|count| initialized_count == count.get()) {
                return self.fail("too many array initializer clauses");
            }
            let element = self.append(
                Expression::AccessIndex {
                    base: pointer,
                    index: initialized_count,
                },
                span,
            )?;
            self.array_initializer_clause(element, specifier, element_ty)?;
            initialized_count = initialized_count.checked_add(1).ok_or_else(|| ParseError {
                message: String::from("too many array initializer clauses"),
                span,
            })?;
            if !has_braces && known_count.is_some_and(|count| initialized_count == count.get()) {
                break;
            }
            if !self.lexer.next_if(Token::Separator(','))?
                || self.lexer.peek()?.0 == Token::Paren('}')
            {
                break;
            }
        }
        Ok(initialized_count)
    }

    fn array_initializer_clause(
        &mut self,
        pointer: Handle<Expression>,
        specifier: TypeSpecifier,
        actual_ty: Handle<Type>,
    ) -> Result<(), ParseError> {
        if let TypeInner::Array {
            base,
            size: ArraySize::Constant(count),
            ..
        } = self.module.types[actual_ty].inner
        {
            let opening = self.lexer.peek()?.1;
            let has_braces = self.lexer.next_if(Token::Paren('{'))?;
            let initialized_count = if has_braces && self.lexer.next_if(Token::Paren('}'))? {
                0
            } else {
                let initialized_count = self.array_initializer_clauses(
                    pointer,
                    specifier,
                    base,
                    Some(count),
                    has_braces,
                    opening,
                )?;
                if has_braces {
                    self.lexer.expect_span(Token::Paren('}'))?;
                }
                initialized_count
            };
            if initialized_count < count.get() {
                self.zero_array_tail(pointer, base, initialized_count, count, opening)?;
            }
        } else {
            match specifier {
                TypeSpecifier::Scalar(scalar) => {
                    let value = self.scalar_initializer_clause(scalar)?;
                    self.push_statement(
                        Statement::Store {
                            pointer,
                            value: value.handle,
                        },
                        value.span,
                    );
                }
                TypeSpecifier::Vector { .. } => {
                    let opening = self.lexer.peek()?.1;
                    let has_braces = self.lexer.next_if(Token::Paren('{'))?;
                    self.vector_list_initializer(
                        pointer, specifier, actual_ty, has_braces, opening,
                    )?;
                }
                TypeSpecifier::Structure(_) => {
                    let opening = self.lexer.peek()?.1;
                    let has_braces = self.lexer.next_if(Token::Paren('{'))?;
                    self.record_aggregate_initializer(
                        pointer, actual_ty, has_braces, opening, None,
                    )?;
                }
            }
        }
        Ok(())
    }

    pub(super) fn record_aggregate_initializer(
        &mut self,
        pointer: Handle<Expression>,
        ty: Handle<Type>,
        has_braces: bool,
        span: Span,
        pending_value: Option<Value>,
    ) -> Result<(), ParseError> {
        self.function.expression_depth += 1;
        let result = if self.function.expression_depth > 64 {
            self.fail("CUDA initializer nesting limit exceeded")
        } else {
            self.record_aggregate_initializer_inner(pointer, ty, has_braces, span, pending_value)
        };
        self.function.expression_depth -= 1;
        result
    }

    fn record_aggregate_initializer_inner(
        &mut self,
        pointer: Handle<Expression>,
        ty: Handle<Type>,
        has_braces: bool,
        span: Span,
        pending_value: Option<Value>,
    ) -> Result<(), ParseError> {
        if pending_value.is_none() && has_braces && self.lexer.next_if(Token::Paren('}'))? {
            let value = self.append(Expression::ZeroValue(ty), span)?;
            self.push_statement(Statement::Store { pointer, value }, span);
            return Ok(());
        }
        let mut pending_value =
            if pending_value.is_some() || self.lexer.peek()?.0 == Token::Paren('{') {
                pending_value
            } else {
                let expression = self.expression(0)?;
                Some(self.rvalue(expression)?)
            };
        if let Some(ref value) = pending_value {
            if !matches!(self.module.types[ty].inner, TypeInner::Array { .. })
                && self
                    .module
                    .compare_types(&value.ty, &TypeResolution::Handle(ty))
            {
                self.push_statement(
                    Statement::Store {
                        pointer,
                        value: value.handle,
                    },
                    value.span,
                );
                if has_braces {
                    self.lexer.next_if(Token::Separator(','))?;
                    self.lexer.expect_span(Token::Paren('}'))?;
                }
                return Ok(());
            }
        }
        let count = match self.module.types[ty].inner {
            TypeInner::Struct { ref members, .. } => members.len() as u32,
            TypeInner::Array {
                size: ArraySize::Constant(count),
                ..
            } => count.get(),
            TypeInner::Vector { size, .. } => size as u32,
            _ => return self.fail("expected an aggregate initializer"),
        };
        let mut initialized_count = 0;
        loop {
            let member_ty = self.record_initializer_member(ty, initialized_count, span);
            let member = self.append(
                Expression::AccessIndex {
                    base: pointer,
                    index: initialized_count,
                },
                span,
            )?;
            self.record_initializer_clause(member, member_ty, pending_value.take())?;
            initialized_count += 1;
            if initialized_count == count {
                break;
            }
            if !self.lexer.next_if(Token::Separator(','))?
                || self.lexer.peek()?.0 == Token::Paren('}')
            {
                break;
            }
        }
        if has_braces {
            if initialized_count == count {
                self.lexer.next_if(Token::Separator(','))?;
            }
            self.lexer.expect_span(Token::Paren('}'))?;
        }
        if let TypeInner::Array {
            base,
            size: ArraySize::Constant(count),
            ..
        } = self.module.types[ty].inner
        {
            if initialized_count < count.get() {
                self.zero_array_tail(pointer, base, initialized_count, count, span)?;
            }
        } else {
            for index in initialized_count..count {
                let member_ty = self.record_initializer_member(ty, index, span);
                let member = self.append(
                    Expression::AccessIndex {
                        base: pointer,
                        index,
                    },
                    span,
                )?;
                let value = self.append(Expression::ZeroValue(member_ty), span)?;
                self.push_statement(
                    Statement::Store {
                        pointer: member,
                        value,
                    },
                    span,
                );
            }
        }
        Ok(())
    }

    fn record_initializer_member(
        &mut self,
        ty: Handle<Type>,
        index: u32,
        span: Span,
    ) -> Handle<Type> {
        match self.module.types[ty].inner {
            TypeInner::Struct { ref members, .. } => members[index as usize].ty,
            TypeInner::Array { base, .. } => base,
            TypeInner::Vector { scalar, .. } => self.scalar_handle(scalar, span),
            _ => unreachable!(),
        }
    }

    fn record_initializer_clause(
        &mut self,
        pointer: Handle<Expression>,
        ty: Handle<Type>,
        pending_value: Option<Value>,
    ) -> Result<(), ParseError> {
        if let TypeInner::Scalar(scalar) = self.module.types[ty].inner {
            let value = match pending_value {
                Some(value) => self.list_convert_scalar(value, scalar)?,
                None => self.scalar_initializer_clause(scalar)?,
            };
            self.push_statement(
                Statement::Store {
                    pointer,
                    value: value.handle,
                },
                value.span,
            );
            Ok(())
        } else {
            let opening = self.lexer.peek()?.1;
            let has_braces = pending_value.is_none() && self.lexer.next_if(Token::Paren('{'))?;
            self.record_aggregate_initializer(pointer, ty, has_braces, opening, pending_value)
        }
    }

    fn scalar_initializer_clause(&mut self, scalar: Scalar) -> Result<Value, ParseError> {
        if self.lexer.peek()?.0 == Token::Paren('{') {
            self.scalar_list_initializer(scalar)
        } else {
            let expression = self.expression(0)?;
            let value = self.rvalue(expression)?;
            self.list_convert_scalar(value, scalar)
        }
    }

    fn scalar_list_initializer(&mut self, scalar: Scalar) -> Result<Value, ParseError> {
        let start = self.lexer.expect_span(Token::Paren('{'))?;
        let mut value = if self.lexer.peek()?.0 == Token::Paren('}') {
            let literal = match scalar {
                Scalar::BOOL => Literal::Bool(false),
                Scalar::I32 => Literal::I32(0),
                Scalar::U32 => Literal::U32(0),
                Scalar::F32 => Literal::F32(0.0),
                _ => return self.fail("unsupported scalar list initializer"),
            };
            Value {
                handle: self.append(Expression::Literal(literal), start)?,
                ty: scalar_resolution(scalar),
                span: start,
                constant: literal.into(),
            }
        } else {
            if self.lexer.peek()?.0 == Token::Paren('{') {
                return self.fail("nested scalar initializer lists are unsupported");
            }
            let expression = self.expression(0)?;
            let value = self.rvalue(expression)?;
            let value = self.list_convert_scalar(value, scalar)?;
            self.lexer.next_if(Token::Separator(','))?;
            value
        };
        let end = self.lexer.expect_span(Token::Paren('}'))?;
        value.span = start.until(&end);
        Ok(value)
    }

    pub(super) fn list_convert_scalar(
        &mut self,
        value: Value,
        target: Scalar,
    ) -> Result<Value, ParseError> {
        let source = self.require_scalar_type(&value.ty, value.span)?;
        let is_representable = source == target
            || match target {
                Scalar::I32 => {
                    source == Scalar::BOOL
                        || matches!(value.constant.literal, Some(Literal::U32(integer)) if integer <= i32::MAX as u32)
                }
                Scalar::U32 => {
                    source == Scalar::BOOL
                        || matches!(value.constant.literal, Some(Literal::I32(integer)) if integer >= 0)
                }
                Scalar::BOOL => matches!(
                    value.constant.literal,
                    Some(Literal::I32(0 | 1) | Literal::U32(0 | 1))
                ),
                Scalar::F32 => {
                    let integer = match value.constant.literal {
                        Some(Literal::Bool(boolean)) => Some(f64::from(u32::from(boolean))),
                        Some(Literal::I32(integer)) => Some(f64::from(integer)),
                        Some(Literal::U32(integer)) => Some(f64::from(integer)),
                        _ => None,
                    };
                    integer.is_some_and(|integer| f64::from(integer as f32) == integer)
                }
                _ => false,
            };
        if !is_representable {
            return Err(ParseError {
                message: String::from("narrowing conversion in initializer list"),
                span: value.span,
            });
        }
        self.convert_scalar(value.into(), target)
    }

    fn vector_list_initializer(
        &mut self,
        pointer: Handle<Expression>,
        specifier: TypeSpecifier,
        actual_ty: Handle<Type>,
        has_braces: bool,
        span: Span,
    ) -> Result<(), ParseError> {
        let TypeSpecifier::Vector { size, scalar } = specifier else {
            return self.fail("expected a vector initializer");
        };
        if has_braces && self.lexer.next_if(Token::Paren('}'))? {
            let value = self.append(Expression::ZeroValue(actual_ty), span)?;
            self.push_statement(Statement::Store { pointer, value }, span);
            return Ok(());
        }

        let mut pending_value = if self.lexer.peek()?.0 == Token::Paren('{') {
            None
        } else {
            let expression = self.expression(0)?;
            let value = self.rvalue(expression)?;
            if self
                .module
                .compare_types(&value.ty, &TypeResolution::Handle(actual_ty))
            {
                self.push_statement(
                    Statement::Store {
                        pointer,
                        value: value.handle,
                    },
                    value.span,
                );
                if has_braces {
                    self.lexer.next_if(Token::Separator(','))?;
                    self.lexer.expect_span(Token::Paren('}'))?;
                }
                return Ok(());
            }
            Some(value)
        };

        let mut initialized_count = 0;
        loop {
            let value = match pending_value.take() {
                Some(value) => self.list_convert_scalar(value, scalar)?,
                None => self.scalar_initializer_clause(scalar)?,
            };
            let field = self.append(
                Expression::AccessIndex {
                    base: pointer,
                    index: initialized_count,
                },
                span,
            )?;
            self.push_statement(
                Statement::Store {
                    pointer: field,
                    value: value.handle,
                },
                value.span,
            );
            initialized_count += 1;
            if initialized_count == size as u32 {
                break;
            }
            if !self.lexer.next_if(Token::Separator(','))?
                || self.lexer.peek()?.0 == Token::Paren('}')
            {
                break;
            }
        }

        if has_braces {
            if initialized_count == size as u32 {
                self.lexer.next_if(Token::Separator(','))?;
            }
            self.lexer.expect_span(Token::Paren('}'))?;
        }
        let scalar_ty = self.scalar_handle(scalar, span);
        for index in initialized_count..size as u32 {
            let field = self.append(
                Expression::AccessIndex {
                    base: pointer,
                    index,
                },
                span,
            )?;
            let value = self.append(Expression::ZeroValue(scalar_ty), span)?;
            self.push_statement(
                Statement::Store {
                    pointer: field,
                    value,
                },
                span,
            );
        }
        Ok(())
    }

    fn zero_array_tail(
        &mut self,
        pointer: Handle<Expression>,
        element_ty: Handle<Type>,
        initialized_count: u32,
        array_count: NonZeroU32,
        span: Span,
    ) -> Result<(), ParseError> {
        let counter = self.temporary_pointer(&scalar_resolution(Scalar::U32), span)?;
        let initial = self.append(Expression::Literal(Literal::U32(initialized_count)), span)?;
        self.push_statement(
            Statement::Store {
                pointer: counter,
                value: initial,
            },
            span,
        );
        let limit = self.append(Expression::Literal(Literal::U32(array_count.get())), span)?;
        let ((), body) = self.capture_block(|parser| {
            let index = parser.append(Expression::Load { pointer: counter }, span)?;
            let condition = parser.append(
                Expression::Binary {
                    op: BinaryOperator::GreaterEqual,
                    left: index,
                    right: limit,
                },
                span,
            )?;
            let mut accept = crate::Block::new();
            accept.push(Statement::Break, span);
            parser.push_statement(
                Statement::If {
                    condition,
                    accept,
                    reject: crate::Block::new(),
                },
                span,
            );
            let element = parser.append(
                Expression::Access {
                    base: pointer,
                    index,
                },
                span,
            )?;
            match parser.module.types[element_ty].inner {
                TypeInner::Array {
                    base,
                    size: ArraySize::Constant(count),
                    ..
                } => parser.zero_array_tail(element, base, 0, count, span)?,
                _ => {
                    let value = parser.append(Expression::ZeroValue(element_ty), span)?;
                    parser.push_statement(
                        Statement::Store {
                            pointer: element,
                            value,
                        },
                        span,
                    );
                }
            }
            Ok(())
        })?;
        let ((), continuing) = self.capture_block(|parser| {
            let index = parser.append(Expression::Load { pointer: counter }, span)?;
            let one = parser.append(Expression::Literal(Literal::U32(1)), span)?;
            let value = parser.append(
                Expression::Binary {
                    op: BinaryOperator::Add,
                    left: index,
                    right: one,
                },
                span,
            )?;
            parser.push_statement(
                Statement::Store {
                    pointer: counter,
                    value,
                },
                span,
            );
            Ok(())
        })?;
        self.push_statement(
            Statement::Loop {
                body,
                continuing,
                break_if: None,
            },
            span,
        );
        Ok(())
    }
}
