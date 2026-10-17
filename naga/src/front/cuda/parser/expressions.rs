use super::*;

const UNARY_PRECEDENCE: u8 = 11;

impl Parser<'_> {
    pub(super) fn expression(&mut self, minimum_precedence: u8) -> Result<Value, ParseError> {
        self.expression_depth += 1;
        if self.expression_depth > 64 {
            return self.fail("CUDA expression nesting limit exceeded");
        }
        let result = self.capture_block(|parser| parser.expression_inner(minimum_precedence));
        self.expression_depth -= 1;
        let (value, body) = result?;
        self.append_block(body);
        Ok(value)
    }

    fn expression_inner(&mut self, minimum_precedence: u8) -> Result<Value, ParseError> {
        let mut left = self.primary()?;
        loop {
            let (operator, precedence) = match self.lexer.peek()?.kind {
                TokenKind::Operator("||") => (BinaryOperator::LogicalOr, 1),
                TokenKind::Operator("&&") => (BinaryOperator::LogicalAnd, 2),
                TokenKind::Symbol('|') => (BinaryOperator::InclusiveOr, 3),
                TokenKind::Symbol('^') => (BinaryOperator::ExclusiveOr, 4),
                TokenKind::Symbol('&') => (BinaryOperator::And, 5),
                TokenKind::Operator("==") => (BinaryOperator::Equal, 6),
                TokenKind::Operator("!=") => (BinaryOperator::NotEqual, 6),
                TokenKind::Symbol('<') => (BinaryOperator::Less, 7),
                TokenKind::Symbol('>') => (BinaryOperator::Greater, 7),
                TokenKind::Operator("<=") => (BinaryOperator::LessEqual, 7),
                TokenKind::Operator(">=") => (BinaryOperator::GreaterEqual, 7),
                TokenKind::Operator("<<") => (BinaryOperator::ShiftLeft, 8),
                TokenKind::Operator(">>") => (BinaryOperator::ShiftRight, 8),
                TokenKind::Symbol('+') => (BinaryOperator::Add, 9),
                TokenKind::Symbol('-') => (BinaryOperator::Subtract, 9),
                TokenKind::Symbol('*') => (BinaryOperator::Multiply, 10),
                TokenKind::Symbol('/') => (BinaryOperator::Divide, 10),
                TokenKind::Symbol('%') => (BinaryOperator::Modulo, 10),
                _ => break,
            };
            if precedence < minimum_precedence {
                break;
            }
            self.lexer.next()?;
            if matches!(
                operator,
                BinaryOperator::LogicalAnd | BinaryOperator::LogicalOr
            ) {
                left = self.logical(left, operator, precedence)?;
                continue;
            }
            if matches!(
                operator,
                BinaryOperator::ShiftLeft | BinaryOperator::ShiftRight
            ) {
                left = self.rvalue(left)?;
            }
            let right = self.expression(precedence + 1)?;
            left = self.binary_value(operator, left, right)?;
        }
        if minimum_precedence == 0 {
            let operator = match self.lexer.peek()?.kind {
                TokenKind::Symbol('=') => Some(None),
                TokenKind::Operator("+=") => Some(Some(BinaryOperator::Add)),
                TokenKind::Operator("-=") => Some(Some(BinaryOperator::Subtract)),
                TokenKind::Operator("*=") => Some(Some(BinaryOperator::Multiply)),
                TokenKind::Operator("/=") => Some(Some(BinaryOperator::Divide)),
                TokenKind::Operator("%=") => Some(Some(BinaryOperator::Modulo)),
                TokenKind::Operator("&=") => Some(Some(BinaryOperator::And)),
                TokenKind::Operator("|=") => Some(Some(BinaryOperator::InclusiveOr)),
                TokenKind::Operator("^=") => Some(Some(BinaryOperator::ExclusiveOr)),
                TokenKind::Operator("<<=") => Some(Some(BinaryOperator::ShiftLeft)),
                TokenKind::Operator(">>=") => Some(Some(BinaryOperator::ShiftRight)),
                _ => None,
            };
            if let Some(operator) = operator {
                return self.assignment(left, operator);
            }
        }
        Ok(left)
    }

    fn primary(&mut self) -> Result<Value, ParseError> {
        let mut value = self.primary_inner()?;
        while let TokenKind::Operator(operator @ ("++" | "--")) = self.lexer.peek()?.kind {
            let span = value.span.until(&self.lexer.peek()?.span);
            self.lexer.next()?;
            value = self.update(value, operator == "++", true, span)?;
        }
        Ok(value)
    }

    fn primary_inner(&mut self) -> Result<Value, ParseError> {
        let token = self.lexer.peek()?;
        if let TokenKind::Operator(operator @ ("++" | "--")) = token.kind {
            self.lexer.next()?;
            let value = self.expression(UNARY_PRECEDENCE)?;
            let span = token.span.until(&value.span);
            return self.update(value, operator == "++", false, span);
        }
        if self.take_symbol('(')? {
            let checkpoint = self.lexer.clone();
            if matches!(
                self.lexer.peek()?.kind,
                TokenKind::Ident("const" | "bool" | "int" | "unsigned" | "float")
            ) {
                let target = self.cast_type()?;
                if self.take_symbol(')')? {
                    let value = self.expression(UNARY_PRECEDENCE)?;
                    let mut value = self.convert_value(value, target)?;
                    value.span = token.span.until(&value.span);
                    return Ok(value);
                }
                self.lexer = checkpoint;
            }
            let value = self.expression(0)?;
            self.expect_symbol(')')?;
            return Ok(value);
        }
        if self.take_symbol('-')?
            || self.take_symbol('+')?
            || self.take_symbol('!')?
            || self.take_symbol('~')?
        {
            let value = self.expression(UNARY_PRECEDENCE)?;
            if token.kind == TokenKind::Symbol('!') {
                let value = self.truthy(value)?;
                let span = token.span.until(&value.span);
                let handle = self.append(
                    Expression::Unary {
                        op: UnaryOperator::LogicalNot,
                        expr: value.handle,
                    },
                    span,
                )?;
                return Ok(Value {
                    handle,
                    scalar: Scalar::BOOL,
                    span,
                    lvalue: None,
                });
            }
            if token.kind == TokenKind::Symbol('~') {
                let value = self.integral_operand(value)?;
                let span = token.span.until(&value.span);
                let handle = self.append(
                    Expression::Unary {
                        op: UnaryOperator::BitwiseNot,
                        expr: value.handle,
                    },
                    span,
                )?;
                return Ok(Value {
                    handle,
                    scalar: value.scalar,
                    span,
                    lvalue: None,
                });
            }
            let value = self.promote_integral(value)?;
            if token.kind == TokenKind::Symbol('+') {
                return Ok(value);
            }
            let span = token.span.until(&value.span);
            let expression = if value.scalar == Scalar::U32 {
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
                handle,
                scalar: value.scalar,
                span,
                lvalue: None,
            });
        }
        match token.kind {
            TokenKind::Number(text) => {
                self.lexer.next()?;
                self.number(text, token.span)
            }
            TokenKind::Ident(name) => {
                self.lexer.next()?;
                if name == "static_cast" {
                    self.expect_symbol('<')?;
                    let target = self.cast_type()?;
                    self.expect_symbol('>')?;
                    self.expect_symbol('(')?;
                    let value = self.expression(0)?;
                    let end = self.expect_symbol(')')?;
                    let mut value = self.convert_value(value, target)?;
                    value.span = token.span.until(&end);
                    return Ok(value);
                }
                let functional_target = match name {
                    "bool" => Some(Scalar::BOOL),
                    "int" => Some(Scalar::I32),
                    "unsigned" => Some(Scalar::U32),
                    "float" => Some(Scalar::F32),
                    _ => None,
                };
                if let Some(target) = functional_target {
                    self.expect_symbol('(')?;
                    let value = self.expression(0)?;
                    let end = self.expect_symbol(')')?;
                    let mut value = self.convert_value(value, target)?;
                    value.span = token.span.until(&end);
                    return Ok(value);
                }
                if matches!(name, "true" | "false") {
                    let handle = self.append(
                        Expression::Literal(Literal::Bool(name == "true")),
                        token.span,
                    )?;
                    return Ok(Value {
                        handle,
                        scalar: Scalar::BOOL,
                        span: token.span,
                        lvalue: None,
                    });
                }
                if matches!(name, "threadIdx" | "blockIdx" | "blockDim" | "gridDim") {
                    return self.builtin(name, token.span);
                }
                let symbol = self.lookup_symbol(name).ok_or_else(|| ParseError {
                    message: format!("unknown variable `{name}`"),
                    span: token.span,
                })?;
                let (pointer, scalar, writable, span) = match symbol {
                    Symbol::Buffer {
                        pointer,
                        scalar,
                        writable,
                    } => {
                        let pointer = self.buffer_element(pointer, token.span)?;
                        (
                            pointer,
                            scalar,
                            writable,
                            self.function.expressions.get_span(pointer),
                        )
                    }
                    Symbol::Local {
                        pointer,
                        scalar,
                        writable,
                    } => (pointer, scalar, writable, token.span),
                };
                Ok(Value {
                    handle: pointer,
                    scalar,
                    span,
                    lvalue: Some(LValue { pointer, writable }),
                })
            }
            _ => self.fail("expected a supported scalar expression"),
        }
    }

    pub(super) fn truthy(&mut self, value: Value) -> Result<Value, ParseError> {
        let value = self.rvalue(value)?;
        if value.scalar == Scalar::BOOL {
            return Ok(value);
        }
        let literal = match value.scalar.kind {
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
            handle,
            scalar: Scalar::BOOL,
            span: value.span,
            lvalue: None,
        })
    }

    fn logical(
        &mut self,
        left: Value,
        operator: BinaryOperator,
        precedence: u8,
    ) -> Result<Value, ParseError> {
        let left = self.truthy(left)?;
        let ty = self.scalar_handle(Scalar::BOOL, left.span);
        let local = self.function.local_variables.append(
            LocalVariable {
                name: None,
                ty,
                init: None,
            },
            left.span,
        );
        let pointer = self.append(Expression::LocalVariable(local), left.span)?;
        self.function.body.push(
            Statement::Store {
                pointer,
                value: left.handle,
            },
            left.span,
        );
        let (right, right_body) = self.capture_block(|parser| {
            let right = parser.expression(precedence + 1)?;
            let right = parser.truthy(right)?;
            parser.function.body.push(
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
        self.function.body.push(
            Statement::If {
                condition: left.handle,
                accept,
                reject,
            },
            span,
        );
        let handle = self.append(Expression::Load { pointer }, span)?;
        Ok(Value {
            handle,
            scalar: Scalar::BOOL,
            span,
            lvalue: None,
        })
    }
}
