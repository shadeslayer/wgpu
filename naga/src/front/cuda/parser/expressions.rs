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
            let (converted_left, right) = self.usual_arithmetic(left, right)?;
            left = converted_left;
            let span = left.span.until(&right.span);
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
            let start = self.position;
            if matches!(
                self.current().kind,
                TokenKind::Ident("const" | "bool" | "int" | "unsigned" | "float")
            ) {
                let target = self.cast_type()?;
                if self.take_symbol(')') {
                    let value = self.expression(7)?;
                    let mut value = self.convert_value(value, target)?;
                    value.span = token.span.until(&value.span);
                    return Ok(value);
                }
                self.position = start;
            }
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
            });
        }
        match token.kind {
            TokenKind::Number(text) => {
                self.advance();
                self.number(text, token.span)
            }
            TokenKind::Ident(name) => {
                self.advance();
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
