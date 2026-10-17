use super::*;

impl Parser<'_> {
    pub(super) fn expression(&mut self, minimum_precedence: u8) -> Result<Value, ParseError> {
        self.expression_depth += 1;
        if self.expression_depth > 64 {
            return Err(self.error("CUDA expression nesting limit exceeded"));
        }
        let result = self.expression_inner(minimum_precedence);
        self.expression_depth -= 1;
        result
    }

    fn expression_inner(&mut self, minimum_precedence: u8) -> Result<Value, ParseError> {
        let mut left = self.primary()?;
        loop {
            let (operator, precedence) = match self.current().kind {
                TokenKind::Operator("||") => (BinaryOperator::LogicalOr, 1),
                TokenKind::Operator("&&") => (BinaryOperator::LogicalAnd, 2),
                TokenKind::Operator("==") => (BinaryOperator::Equal, 3),
                TokenKind::Operator("!=") => (BinaryOperator::NotEqual, 3),
                TokenKind::Symbol('<') => (BinaryOperator::Less, 4),
                TokenKind::Symbol('>') => (BinaryOperator::Greater, 4),
                TokenKind::Operator("<=") => (BinaryOperator::LessEqual, 4),
                TokenKind::Operator(">=") => (BinaryOperator::GreaterEqual, 4),
                TokenKind::Symbol('+') => (BinaryOperator::Add, 5),
                TokenKind::Symbol('-') => (BinaryOperator::Subtract, 5),
                TokenKind::Symbol('*') => (BinaryOperator::Multiply, 6),
                TokenKind::Symbol('/') => (BinaryOperator::Divide, 6),
                _ => break,
            };
            if precedence < minimum_precedence {
                break;
            }
            self.advance();
            if matches!(
                operator,
                BinaryOperator::LogicalAnd | BinaryOperator::LogicalOr
            ) {
                left = self.logical(left, operator, precedence)?;
                continue;
            }
            let right = self.expression(precedence + 1)?;
            if left.scalar != right.scalar {
                return Err(ParseError {
                    message: String::from("binary operands require matching scalar types"),
                    span: left.span.until(&right.span),
                });
            }
            let span = left.span.until(&right.span);
            if left.scalar == Scalar::BOOL
                && !matches!(operator, BinaryOperator::Equal | BinaryOperator::NotEqual)
            {
                return Err(ParseError {
                    message: String::from("this operator requires numeric operands"),
                    span,
                });
            }
            let scalar = match operator {
                BinaryOperator::Equal
                | BinaryOperator::NotEqual
                | BinaryOperator::Less
                | BinaryOperator::LessEqual
                | BinaryOperator::Greater
                | BinaryOperator::GreaterEqual => Scalar::BOOL,
                _ => left.scalar,
            };
            let handle = self.append(
                Expression::Binary {
                    op: operator,
                    left: left.handle,
                    right: right.handle,
                },
                span,
            )?;
            left = Value {
                handle,
                scalar,
                span,
            };
        }
        Ok(left)
    }

    fn primary(&mut self) -> Result<Value, ParseError> {
        let token = self.current();
        if self.take_symbol('(') {
            let value = self.expression(0)?;
            self.expect_symbol(')')?;
            return Ok(value);
        }
        if self.take_symbol('-') || self.take_symbol('+') || self.take_symbol('!') {
            let value = self.expression(7)?;
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
                });
            }
            if value.scalar == Scalar::BOOL {
                return Err(ParseError {
                    message: String::from("unary arithmetic requires a numeric operand"),
                    span: token.span.until(&value.span),
                });
            }
            if token.kind == TokenKind::Symbol('+') {
                return Ok(value);
            }
            if value.scalar.kind == ScalarKind::Uint {
                return Err(ParseError {
                    message: String::from("unsigned negation is unsupported"),
                    span: token.span.until(&value.span),
                });
            }
            let span = token.span.until(&value.span);
            let handle = self.append(
                Expression::Unary {
                    op: UnaryOperator::Negate,
                    expr: value.handle,
                },
                span,
            )?;
            return Ok(Value {
                handle,
                scalar: value.scalar,
                span,
            });
        }
        match token.kind {
            TokenKind::Number(text) => {
                self.advance();
                self.number(text, token.span)
            }
            TokenKind::Ident(name) => {
                self.advance();
                if matches!(name, "true" | "false") {
                    let handle = self.append(
                        Expression::Literal(Literal::Bool(name == "true")),
                        token.span,
                    )?;
                    return Ok(Value {
                        handle,
                        scalar: Scalar::BOOL,
                        span: token.span,
                    });
                }
                if matches!(name, "threadIdx" | "blockIdx" | "blockDim" | "gridDim") {
                    return self.builtin(name, token.span);
                }
                let symbol = self.lookup_symbol(name).ok_or_else(|| ParseError {
                    message: format!("unknown variable `{name}`"),
                    span: token.span,
                })?;
                let (pointer, scalar) = match symbol {
                    Symbol::Buffer {
                        pointer, scalar, ..
                    } => (self.buffer_element(pointer, token.span)?, scalar),
                    Symbol::Local {
                        pointer, scalar, ..
                    } => (pointer, scalar),
                };
                let handle = self.append(Expression::Load { pointer }, token.span)?;
                Ok(Value {
                    handle,
                    scalar,
                    span: token.span,
                })
            }
            _ => Err(self.error("expected a supported scalar expression")),
        }
    }

    pub(super) fn truthy(&mut self, value: Value) -> Result<Value, ParseError> {
        if value.scalar == Scalar::BOOL {
            return Ok(value);
        }
        let literal = match value.scalar.kind {
            ScalarKind::Sint => Literal::I32(0),
            ScalarKind::Uint => Literal::U32(0),
            ScalarKind::Float => Literal::F32(0.0),
            _ => return Err(self.error("condition requires a scalar value")),
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
        let parent = core::mem::take(&mut self.function.body);
        let result = self
            .expression(precedence + 1)
            .and_then(|right| self.truthy(right));
        if let Ok(right) = result {
            self.function.body.push(
                Statement::Store {
                    pointer,
                    value: right.handle,
                },
                right.span,
            );
        }
        let right_body = core::mem::replace(&mut self.function.body, parent);
        let right = result?;
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
        })
    }
}
