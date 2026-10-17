use super::*;

impl Parser<'_> {
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
        let mut left = self.mutable_reference(left)?;
        self.lexer.next()?;
        let (right, right_body) = self.capture_block(|parser| {
            let right = parser.expression(0)?;
            parser.rvalue(right)
        })?;
        let span = left.span.until(&right.span);
        self.prepend_block(right_body);
        self.decorate_reference(&mut left, &mut |parser, pointer, scalar, reference_span| {
            let value = if let Some(op) = operator {
                let previous_value = parser.load_pointer(pointer, scalar, reference_span)?;
                parser.binary_value(op, previous_value.into(), right.into())?
            } else {
                right
            };
            let value = parser.convert_value(value.into(), scalar)?;
            parser.push_statement(
                Statement::Store {
                    pointer,
                    value: value.handle,
                },
                span,
            );
            Ok(())
        })?;
        Ok(ScalarExpression::Reference(ReferenceValue { span, ..left }))
    }

    pub(super) fn update(
        &mut self,
        value: ScalarExpression,
        increment: bool,
        postfix: bool,
        span: Span,
    ) -> Result<ScalarExpression, ParseError> {
        let mut value = self.mutable_reference(value)?;
        if value.scalar == Scalar::BOOL {
            return Err(ParseError {
                message: String::from("boolean increment and decrement are invalid"),
                span,
            });
        }
        if !postfix {
            self.decorate_reference(
                &mut value,
                &mut |parser, pointer, scalar, reference_span| {
                    parser.update_pointer(pointer, scalar, increment, reference_span, span)?;
                    Ok(())
                },
            )?;
            return Ok(ScalarExpression::Reference(ReferenceValue {
                span,
                ..value
            }));
        }
        if let Reference::Pointer(pointer) = value.reference {
            let previous_value =
                self.update_pointer(pointer, value.scalar, increment, value.span, span)?;
            return Ok(Value {
                span,
                ..previous_value
            }
            .into());
        }
        let scalar = value.scalar;
        let pointer = self.temporary_pointer(scalar, span)?;
        self.consume_reference(value, &mut |parser, source, scalar, reference_span| {
            let previous_value =
                parser.update_pointer(source, scalar, increment, reference_span, span)?;
            parser.push_statement(
                Statement::Store {
                    pointer,
                    value: previous_value.handle,
                },
                span,
            );
            Ok(())
        })?;
        let handle = self.append(Expression::Load { pointer }, span)?;
        Ok(Value {
            handle,
            scalar,
            span,
        }
        .into())
    }

    fn update_pointer(
        &mut self,
        pointer: Handle<Expression>,
        scalar: Scalar,
        increment: bool,
        reference_span: Span,
        span: Span,
    ) -> Result<Value, ParseError> {
        let previous_value = self.load_pointer(pointer, scalar, reference_span)?;
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
            scalar,
        )?;
        self.push_statement(
            Statement::Store {
                pointer,
                value: updated_value.handle,
            },
            span,
        );
        Ok(previous_value)
    }
}
