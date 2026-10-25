use super::*;

const UNARY_PRECEDENCE: u8 = 11;

impl Parser<'_> {
    pub(super) fn comma_expression(&mut self) -> Result<ExpressionValue, ParseError> {
        self.comma_expression_result()?.require_value()
    }

    pub(super) fn comma_expression_result(&mut self) -> Result<ExpressionResult, ParseError> {
        let mut result = self.expression_result(0)?;
        while self.lexer.next_if(Token::Separator(','))? {
            let earlier = match result {
                ExpressionResult::Value(ref value) => value.constant(),
                ExpressionResult::Void(_) => ConstantValue::default(),
            };
            self.discard_result(result)?;
            result = self.expression_result(0)?;
            if let ExpressionResult::Value(ref mut value) = result {
                value.set_constant(value.constant().after(earlier));
            }
        }
        Ok(result)
    }

    pub(super) fn expression(
        &mut self,
        minimum_precedence: u8,
    ) -> Result<ExpressionValue, ParseError> {
        self.expression_result(minimum_precedence)?.require_value()
    }

    pub(super) fn discard_expression(&mut self) -> Result<(), ParseError> {
        let result = self.comma_expression_result()?;
        self.discard_result(result)
    }

    pub(super) fn expression_result(
        &mut self,
        minimum_precedence: u8,
    ) -> Result<ExpressionResult, ParseError> {
        self.function.expression_depth += 1;
        if self.function.expression_depth > 64 {
            return self.fail("CUDA expression nesting limit exceeded");
        }
        let result = self.capture_block(|parser| parser.expression_inner(minimum_precedence));
        self.function.expression_depth -= 1;
        let (value, body) = result?;
        self.append_block(body);
        Ok(value)
    }

    fn expression_inner(&mut self, minimum_precedence: u8) -> Result<ExpressionResult, ParseError> {
        let start = self.lexer.peek()?.1.to_range().unwrap().start;
        let mut left = self.primary()?;
        loop {
            let (operator, precedence) = match self.lexer.peek()?.0 {
                Token::LogicalOperation('|') => (BinaryOperator::LogicalOr, 1),
                Token::LogicalOperation('&') => (BinaryOperator::LogicalAnd, 2),
                Token::Operation('|') => (BinaryOperator::InclusiveOr, 3),
                Token::Operation('^') => (BinaryOperator::ExclusiveOr, 4),
                Token::Operation('&') => (BinaryOperator::And, 5),
                Token::LogicalOperation('=') => (BinaryOperator::Equal, 6),
                Token::LogicalOperation('!') => (BinaryOperator::NotEqual, 6),
                Token::Paren('<') => (BinaryOperator::Less, 7),
                Token::Paren('>') => (BinaryOperator::Greater, 7),
                Token::LogicalOperation('<') => (BinaryOperator::LessEqual, 7),
                Token::LogicalOperation('>') => (BinaryOperator::GreaterEqual, 7),
                Token::ShiftOperation('<') => (BinaryOperator::ShiftLeft, 8),
                Token::ShiftOperation('>') => (BinaryOperator::ShiftRight, 8),
                Token::Operation('+') => (BinaryOperator::Add, 9),
                Token::Operation('-') => (BinaryOperator::Subtract, 9),
                Token::Operation('*') => (BinaryOperator::Multiply, 10),
                Token::Operation('/') => (BinaryOperator::Divide, 10),
                Token::Operation('%') => (BinaryOperator::Modulo, 10),
                _ => break,
            };
            if precedence < minimum_precedence {
                break;
            }
            let operator_span = self.lexer.next()?.1;
            let left_value = left.require_value()?;
            if matches!(
                operator,
                BinaryOperator::LogicalAnd | BinaryOperator::LogicalOr
            ) {
                left = self.logical(left_value, operator, precedence, operator_span)?;
                continue;
            }
            if operators::Operator::from_binary(operator).is_some() {
                left = self.binary_operator(left_value, operator, precedence, operator_span)?;
                continue;
            }
            let right = self.expression(precedence + 1)?;
            left = self.binary_value(operator, left_value, right)?.into();
        }
        if minimum_precedence == 0 {
            if self.lexer.next_if(Token::Operation('?'))? {
                left = self.conditional(left, start)?;
            }
            let operator = match self.lexer.peek()?.0 {
                Token::Operation('=') => Some(None),
                Token::AssignmentOperation(character) => Some(Some(match character {
                    '+' => BinaryOperator::Add,
                    '-' => BinaryOperator::Subtract,
                    '*' => BinaryOperator::Multiply,
                    '/' => BinaryOperator::Divide,
                    '%' => BinaryOperator::Modulo,
                    '&' => BinaryOperator::And,
                    '|' => BinaryOperator::InclusiveOr,
                    '^' => BinaryOperator::ExclusiveOr,
                    '<' => BinaryOperator::ShiftLeft,
                    '>' => BinaryOperator::ShiftRight,
                    _ => unreachable!(),
                })),
                _ => None,
            };
            if let Some(operator) = operator {
                return self
                    .assignment(left.require_value()?, operator)
                    .map(ExpressionResult::Value);
            }
        }
        Ok(left)
    }

    fn primary(&mut self) -> Result<ExpressionResult, ParseError> {
        let mut value = self.primary_inner()?;
        loop {
            if self.lexer.next_if(Token::Separator('.'))? {
                let (field, span) = self.identifier()?;
                value = ExpressionResult::Value(self.field(value.require_value()?, field, span)?);
            } else if self.lexer.peek()?.0 == Token::Paren('[') {
                value = ExpressionResult::Value(self.array_index(value.require_value()?)?);
            } else if let operator @ (Token::IncrementOperation | Token::DecrementOperation) =
                self.lexer.peek()?.0
            {
                let operand = value.require_value()?;
                let span = operand.span().until(&self.lexer.peek()?.1);
                self.lexer.next()?;
                value = ExpressionResult::Value(self.update(
                    operand,
                    operator == Token::IncrementOperation,
                    true,
                    span,
                )?);
            } else {
                break;
            }
        }
        Ok(value)
    }

    fn primary_inner(&mut self) -> Result<ExpressionResult, ParseError> {
        let token = self.lexer.peek()?;
        if let operator @ (Token::IncrementOperation | Token::DecrementOperation) = token.0 {
            self.lexer.next()?;
            let value = self.expression(UNARY_PRECEDENCE)?;
            let span = token.1.until(&value.span());
            return self
                .update(value, operator == Token::IncrementOperation, false, span)
                .map(ExpressionResult::Value);
        }
        if self.lexer.next_if(Token::Paren('('))? {
            if self.starts_type()? {
                let mut lookahead = self.lexer.clone();
                self.parse_type_from(&mut lookahead)?;
                if lookahead.next_if(Token::Paren(')'))? {
                    let target = self.cast_type()?;
                    self.lexer.expect_span(Token::Paren(')'))?;
                    let value = self.expression(UNARY_PRECEDENCE)?;
                    let mut value = self.cast_value(value, &target)?;
                    value.span = token.1.until(&value.span);
                    return Ok(value.into());
                }
            }
            let value = self.comma_expression_result()?;
            self.lexer.expect_span(Token::Paren(')'))?;
            return Ok(value);
        }
        if matches!(token.0, Token::Operation('-' | '+' | '!' | '~' | '*' | '&')) {
            self.lexer.next()?;
            let value = self.expression(UNARY_PRECEDENCE)?;
            if self.is_source_class(value.ty()) {
                return self.unary_operator(token.0, value, token.1);
            }
            if matches!(token.0, Token::Operation('*' | '&')) {
                return Err(ParseError {
                    message: String::from("builtin pointer operators are unsupported"),
                    span: token.1,
                });
            }
            if token.0 == Token::Operation('!') {
                let value = self.truthy(value)?;
                let span = token.1.until(&value.span);
                let handle = self.append(
                    Expression::Unary {
                        op: UnaryOperator::LogicalNot,
                        expr: value.handle,
                    },
                    span,
                )?;
                return Ok(Value {
                    constant: ConstantValue::evaluated(value.constant.literal.and_then(
                        |literal| constant_evaluation::unary(UnaryOperator::LogicalNot, literal),
                    )),
                    handle,
                    ty: scalar_resolution(Scalar::BOOL),
                    span,
                }
                .into());
            }
            if token.0 == Token::Operation('~') {
                let value = self.integral_operand(value)?;
                let span = token.1.until(&value.span);
                let handle = self.append(
                    Expression::Unary {
                        op: UnaryOperator::BitwiseNot,
                        expr: value.handle,
                    },
                    span,
                )?;
                return Ok(Value {
                    constant: ConstantValue::evaluated(value.constant.literal.and_then(
                        |literal| constant_evaluation::unary(UnaryOperator::BitwiseNot, literal),
                    )),
                    handle,
                    ty: value.ty,
                    span,
                }
                .into());
            }
            let value = self.promote_integral(value)?;
            if token.0 == Token::Operation('+') {
                return Ok(value.into());
            }
            let span = token.1.until(&value.span);
            let expression = if self.require_scalar_type(&value.ty, value.span)? == Scalar::U32 {
                let zero = self.append(Expression::Literal(Literal::U32(0)), span)?;
                Expression::Binary {
                    op: BinaryOperator::Subtract,
                    left: zero,
                    right: value.handle,
                }
            } else {
                Expression::Unary {
                    op: UnaryOperator::Negate,
                    expr: value.handle,
                }
            };
            let handle = self.append(expression, span)?;
            return Ok(Value {
                constant: ConstantValue::evaluated(value.constant.literal.and_then(|literal| {
                    constant_evaluation::unary(UnaryOperator::Negate, literal)
                })),
                handle,
                ty: value.ty,
                span,
            }
            .into());
        }
        match token.0 {
            Token::Number(text) => {
                self.lexer.next()?;
                self.number(text, token.1).map(Into::into)
            }
            Token::Word(name) => {
                self.lexer.next()?;
                if matches!(self.lookup_symbol(name), Some(Symbol::Member)) {
                    return Err(ParseError {
                        message: String::from("non-static structure member requires an object"),
                        span: token.1,
                    });
                }
                if name == "operator" {
                    let (operator, span) = self.operator_name(token.1)?;
                    return self.call(operator.name(), span);
                }
                if name == "static_cast" {
                    self.lexer.expect_span(Token::Paren('<'))?;
                    let target = self.cast_type()?;
                    self.lexer.expect_span(Token::Paren('>'))?;
                    self.lexer.expect_span(Token::Paren('('))?;
                    let value = self.comma_expression()?;
                    let end = self.lexer.expect_span(Token::Paren(')'))?;
                    let mut value = self.cast_value(value, &target)?;
                    value.span = token.1.until(&end);
                    return Ok(value.into());
                }
                if let Some(target) = self.type_name(name) {
                    if let TypeSpecifier::Structure(ty) = target.ty {
                        if self.lexer.peek()?.0 != Token::Paren('{') {
                            return self.fail("structure values require a typed initializer list");
                        }
                        return self.structure_value(ty, token.1).map(Into::into);
                    }
                    self.lexer.expect_span(Token::Paren('('))?;
                    let value = self.expression(0)?;
                    let end = self.lexer.expect_span(Token::Paren(')'))?;
                    if !matches!(target.ty, TypeSpecifier::Scalar(_)) {
                        return Err(ParseError {
                            message: String::from("vector casts are unsupported"),
                            span: token.1.until(&end),
                        });
                    }
                    let target = self.lower_type(target.ty, token.1);
                    let mut value = self.cast_value(value, &target)?;
                    value.span = token.1.until(&end);
                    return Ok(value.into());
                }
                if matches!(name, "true" | "false") {
                    let handle =
                        self.append(Expression::Literal(Literal::Bool(name == "true")), token.1)?;
                    return Ok(Value {
                        constant: Literal::Bool(name == "true").into(),
                        handle,
                        ty: scalar_resolution(Scalar::BOOL),
                        span: token.1,
                    }
                    .into());
                }
                if matches!(name, "threadIdx" | "blockIdx" | "blockDim" | "gridDim") {
                    return self.builtin(name, token.1).map(Into::into);
                }
                if self.lexer.peek()?.0 == Token::Paren('(') {
                    return self.call(name, token.1);
                }
                let symbol = self.lookup_symbol(name).ok_or_else(|| ParseError {
                    message: format!("unknown variable `{name}`"),
                    span: token.1,
                })?;
                let (pointer, ty, writable, span, constant) = match symbol {
                    Symbol::PendingReference => {
                        return Err(ParseError {
                            message: String::from(
                                "reference initializer cannot use its own binding",
                            ),
                            span: token.1,
                        });
                    }
                    Symbol::Reference {
                        binding,
                        ty,
                        writable,
                        constant,
                    } => {
                        return Ok(ExpressionResult::Value(ExpressionValue::Reference(
                            ReferenceValue {
                                reference: Reference::Bound(binding),
                                writable,
                                ty,
                                span: token.1,
                                constant,
                            },
                        )));
                    }
                    Symbol::Member => {
                        return Err(ParseError {
                            message: String::from("non-static structure member requires an object"),
                            span: token.1,
                        });
                    }
                    Symbol::Type(_) => {
                        return Err(ParseError {
                            message: format!("type `{name}` cannot be used as a value"),
                            span: token.1,
                        });
                    }
                    Symbol::Array {
                        pointer,
                        ty,
                        writable,
                    } => {
                        let (pointer, ty, is_discardable) =
                            self.array_element(pointer, ty, token.1)?;
                        (
                            pointer,
                            ty,
                            writable,
                            self.function.ir.expressions.get_span(pointer),
                            ConstantValue {
                                literal: None,
                                is_discardable,
                            },
                        )
                    }
                    Symbol::Variable {
                        pointer,
                        ty,
                        writable,
                    } => (
                        pointer,
                        ty,
                        writable,
                        token.1,
                        self.pointer_constant(pointer),
                    ),
                    Symbol::Constant {
                        handle,
                        scalar,
                        literal,
                    } => {
                        return Ok(ExpressionResult::Value(ExpressionValue::Reference(
                            ReferenceValue {
                                reference: Reference::Constant(handle),
                                writable: false,
                                ty: scalar_resolution(scalar),
                                span: token.1,
                                constant: ConstantValue {
                                    literal,
                                    is_discardable: true,
                                },
                            },
                        )));
                    }
                };
                Ok(ExpressionResult::Value(ExpressionValue::Reference(
                    ReferenceValue {
                        reference: Reference::Pointer(pointer),
                        writable,
                        ty,
                        span,
                        constant,
                    },
                )))
            }
            _ => self.fail("expected a supported scalar expression"),
        }
    }

    pub(super) fn truthy(&mut self, value: ExpressionValue) -> Result<Value, ParseError> {
        let value = self.rvalue(value)?;
        let scalar = self.require_scalar_type(&value.ty, value.span)?;
        if scalar == Scalar::BOOL {
            return Ok(value);
        }
        let literal = match scalar.kind {
            ScalarKind::Sint => Literal::I32(0),
            ScalarKind::Uint => Literal::U32(0),
            ScalarKind::Float => Literal::F32(0.0),
            _ => return self.fail("condition requires a scalar value"),
        };
        let zero = self.append(Expression::Literal(literal), value.span)?;
        let handle = self.append(
            Expression::Binary {
                op: BinaryOperator::NotEqual,
                left: value.handle,
                right: zero,
            },
            value.span,
        )?;
        Ok(Value {
            constant: ConstantValue::evaluated(value.constant.literal.and_then(|value| {
                constant_evaluation::binary(BinaryOperator::NotEqual, value, literal)
            })),
            handle,
            ty: scalar_resolution(Scalar::BOOL),
            span: value.span,
        })
    }

    fn logical(
        &mut self,
        left: ExpressionValue,
        operator: BinaryOperator,
        precedence: u8,
        operator_span: Span,
    ) -> Result<ExpressionResult, ParseError> {
        let name = operators::Operator::from_binary(operator).unwrap().name();
        if !self.is_source_class(left.ty()) && !self.has_function_candidate(name) {
            return self
                .builtin_logical(left, operator, |parser| parser.expression(precedence + 1))
                .map(Into::into);
        }
        let left = self.rvalue(left)?;
        let (right, right_body) = self.capture_block(|parser| parser.expression(precedence + 1))?;
        if self.is_source_class(&left.ty) || self.is_source_class(right.ty()) {
            self.append_block(right_body);
            let right = self.rvalue(right)?;
            let span = left.span.until(&right.span);
            self.call_with_arguments(name, alloc::vec![left, right], operator_span, span)
        } else {
            self.builtin_logical(left.into(), operator, |parser| {
                parser.append_block(right_body);
                Ok(right)
            })
            .map(Into::into)
        }
    }

    fn builtin_logical(
        &mut self,
        left: ExpressionValue,
        operator: BinaryOperator,
        right: impl FnOnce(&mut Self) -> Result<ExpressionValue, ParseError>,
    ) -> Result<Value, ParseError> {
        let left = self.truthy(left)?;
        let ty = self.scalar_handle(Scalar::BOOL, left.span);
        let local = self.function.ir.local_variables.append(
            LocalVariable {
                name: None,
                ty,
                init: None,
            },
            left.span,
        );
        let pointer = self.append(Expression::LocalVariable(local), left.span)?;
        self.push_statement(
            Statement::Store {
                pointer,
                value: left.handle,
            },
            left.span,
        );
        let (right, right_body) = self.capture_block(|parser| {
            let right = right(parser)?;
            let right = parser.truthy(right)?;
            parser.push_statement(
                Statement::Store {
                    pointer,
                    value: right.handle,
                },
                right.span,
            );
            Ok(right)
        })?;
        let span = left.span.until(&right.span);
        let (accept, reject) = if operator == BinaryOperator::LogicalAnd {
            (right_body, crate::Block::new())
        } else {
            (crate::Block::new(), right_body)
        };
        self.push_statement(
            Statement::If {
                condition: left.handle,
                accept,
                reject,
            },
            span,
        );
        let handle = self.append(Expression::Load { pointer }, span)?;
        let constant = match left.constant.literal {
            Some(Literal::Bool(value)) if value == (operator == BinaryOperator::LogicalOr) => {
                Some(Literal::Bool(value))
            }
            Some(Literal::Bool(_)) => right.constant.literal,
            _ => None,
        };
        Ok(Value {
            constant: ConstantValue::evaluated(constant),
            handle,
            ty: scalar_resolution(Scalar::BOOL),
            span,
        })
    }
}
