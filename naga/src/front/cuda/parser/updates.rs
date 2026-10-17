use super::*;

impl Parser<'_> {
    pub(super) fn rvalue(&mut self, expression: ScalarExpression) -> Result<Value, ParseError> {
        match expression {
            ScalarExpression::Value(value) => Ok(value),
            ScalarExpression::Reference(reference) => self.load_reference(&reference),
        }
    }

    fn load_reference(&mut self, reference: &ReferenceValue) -> Result<Value, ParseError> {
        let handle = self.append(
            Expression::Load {
                pointer: reference.pointer,
            },
            reference.span,
        )?;
        Ok(Value {
            handle,
            scalar: reference.scalar,
            span: reference.span,
        })
    }

    fn mutable_reference(
        &self,
        expression: ScalarExpression,
    ) -> Result<ReferenceValue, ParseError> {
        match expression {
            ScalarExpression::Reference(reference) if reference.writable => Ok(reference),
            ScalarExpression::Reference(reference) => Err(ParseError {
                message: String::from("cannot modify a const value"),
                span: reference.span,
            }),
            ScalarExpression::Value(value) => Err(ParseError {
                message: String::from("update requires a modifiable lvalue"),
                span: value.span,
            }),
        }
    }

    pub(super) fn assignment(
        &mut self,
        left: ScalarExpression,
        operator: Option<BinaryOperator>,
    ) -> Result<ScalarExpression, ParseError> {
        let left = self.mutable_reference(left)?;
        self.lexer.next()?;
        let (right, right_body) = self.capture_block(|parser| {
            let right = parser.expression(0)?;
            parser.rvalue(right)
        })?;
        let span = left.span.until(&right.span);
        self.prepend_block(right_body);
        let value = if let Some(op) = operator {
            let previous_value = self.load_reference(&left)?;
            self.binary_value(op, previous_value.into(), right.into())?
        } else {
            right
        };
        let value = self.convert_value(value.into(), left.scalar)?;
        self.push_statement(
            Statement::Store {
                pointer: left.pointer,
                value: value.handle,
            },
            span,
        );
        Ok(ScalarExpression::Reference(ReferenceValue { span, ..left }))
    }

    pub(super) fn update(
        &mut self,
        value: ScalarExpression,
        increment: bool,
        postfix: bool,
        span: Span,
    ) -> Result<ScalarExpression, ParseError> {
        let value = self.mutable_reference(value)?;
        if value.scalar == Scalar::BOOL {
            return Err(ParseError {
                message: String::from("boolean increment and decrement are invalid"),
                span,
            });
        }
        let previous_value = self.load_reference(&value)?;
        let one = self.append(Expression::Literal(Literal::I32(1)), span)?;
        let one = Value {
            handle: one,
            scalar: Scalar::I32,
            span,
        };
        let (left, right) = self.usual_arithmetic(previous_value.into(), one.into())?;
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
            }
            .into(),
            value.scalar,
        )?;
        self.push_statement(
            Statement::Store {
                pointer: value.pointer,
                value: updated_value.handle,
            },
            span,
        );
        Ok(if postfix {
            Value {
                span,
                ..previous_value
            }
            .into()
        } else {
            ScalarExpression::Reference(ReferenceValue { span, ..value })
        })
    }
}
