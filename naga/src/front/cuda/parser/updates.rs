use super::*;

impl Parser<'_> {
    pub(super) fn rvalue(&mut self, value: Value) -> Result<Value, ParseError> {
        let Some(lvalue) = value.lvalue else {
            return Ok(value);
        };
        let handle = self.append(
            Expression::Load {
                pointer: lvalue.pointer,
            },
            value.span,
        )?;
        Ok(Value {
            handle,
            lvalue: None,
            ..value
        })
    }

    fn mutable_lvalue(&self, value: Value) -> Result<LValue, ParseError> {
        match value.lvalue {
            Some(lvalue) if lvalue.writable => Ok(lvalue),
            Some(_) => Err(ParseError {
                message: String::from("cannot modify a const value"),
                span: value.span,
            }),
            None => Err(ParseError {
                message: String::from("update requires a modifiable lvalue"),
                span: value.span,
            }),
        }
    }

    pub(super) fn assignment(
        &mut self,
        left: Value,
        operator: Option<BinaryOperator>,
    ) -> Result<Value, ParseError> {
        let lvalue = self.mutable_lvalue(left)?;
        self.lexer.next()?;
        let left_body = core::mem::take(&mut self.function.body);
        let right = self.expression(0)?;
        let right = self.rvalue(right)?;
        let span = left.span.until(&right.span);
        self.function.body.extend_block(left_body);
        let value = if let Some(op) = operator {
            let previous_value = self.rvalue(left)?;
            self.binary_value(op, previous_value, right)?
        } else {
            right
        };
        let value = self.convert_value(value, left.scalar)?;
        self.function.body.push(
            Statement::Store {
                pointer: lvalue.pointer,
                value: value.handle,
            },
            span,
        );
        Ok(Value { span, ..left })
    }

    pub(super) fn update(
        &mut self,
        value: Value,
        increment: bool,
        postfix: bool,
        span: Span,
    ) -> Result<Value, ParseError> {
        let lvalue = self.mutable_lvalue(value)?;
        if value.scalar == Scalar::BOOL {
            return Err(ParseError {
                message: String::from("boolean increment and decrement are invalid"),
                span,
            });
        }
        let previous_value = self.rvalue(value)?;
        let one = self.append(Expression::Literal(Literal::I32(1)), span)?;
        let one = Value {
            handle: one,
            scalar: Scalar::I32,
            span,
            lvalue: None,
        };
        let (left, right) = self.usual_arithmetic(previous_value, one)?;
        let handle = self.append(
            Expression::Binary {
                op: if increment {
                    BinaryOperator::Add
                } else {
                    BinaryOperator::Subtract
                },
                left: left.handle,
                right: right.handle,
            },
            span,
        )?;
        let updated_value = self.convert_value(
            Value {
                handle,
                scalar: left.scalar,
                span,
                lvalue: None,
            },
            value.scalar,
        )?;
        self.function.body.push(
            Statement::Store {
                pointer: lvalue.pointer,
                value: updated_value.handle,
            },
            span,
        );
        Ok(Value {
            span,
            ..if postfix { previous_value } else { value }
        })
    }
}
