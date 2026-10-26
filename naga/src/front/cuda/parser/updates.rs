use super::*;

impl Parser<'_> {
    fn mutable_reference(&self, expression: ExpressionValue) -> Result<ReferenceValue, ParseError> {
        match expression {
            ExpressionValue::Reference(reference) if reference.writable => Ok(reference),
            ExpressionValue::Reference(reference) => Err(ParseError {
                message: String::from("cannot modify a const value"),
                span: reference.span,
            }),
            ExpressionValue::Pointer(pointer) => Err(ParseError {
                message: String::from("pointer reassignment and increment are unsupported"),
                span: pointer.span,
            }),
            ExpressionValue::Plain(value) => Err(ParseError {
                message: String::from("update requires a modifiable lvalue"),
                span: value.span,
            }),
        }
    }

    pub(super) fn assignment(
        &mut self,
        left: ExpressionValue,
        operator: Option<BinaryOperator>,
    ) -> Result<ExpressionValue, ParseError> {
        let mut left = self.mutable_reference(left)?;
        self.lexer.next()?;
        let (right, right_body) = self.capture_block(|parser| {
            let right = parser.expression(0)?;
            parser.rvalue(right)
        })?;
        let span = left.span.until(&right.span);
        self.prepend_block(right_body);
        self.decorate_reference(&mut left, &mut |parser, pointer, ty, reference_span| {
            let value = if let Some(op) = operator {
                let previous_value = parser.load_pointer(pointer, ty, reference_span)?;
                parser.binary_value(op, previous_value.into(), right.clone().into())?
            } else {
                right.clone()
            };
            let value = parser.convert_value(value.into(), ty)?;
            parser.push_statement(
                Statement::Store {
                    pointer,
                    value: value.handle,
                },
                span,
            );
            Ok(())
        })?;
        Ok(ExpressionValue::Reference(ReferenceValue {
            span,
            constant: ConstantValue::default(),
            ..left
        }))
    }

    pub(super) fn update(
        &mut self,
        value: ExpressionValue,
        increment: bool,
        postfix: bool,
        span: Span,
    ) -> Result<ExpressionValue, ParseError> {
        let mut value = self.mutable_reference(value)?;
        if self.require_scalar_type(&value.ty, value.span)? == Scalar::BOOL {
            return Err(ParseError {
                message: String::from("boolean increment and decrement are invalid"),
                span,
            });
        }
        if !postfix {
            self.decorate_reference(&mut value, &mut |parser, pointer, ty, reference_span| {
                parser.update_pointer(pointer, ty, increment, reference_span, span)?;
                Ok(())
            })?;
            return Ok(ExpressionValue::Reference(ReferenceValue {
                span,
                constant: ConstantValue::default(),
                ..value
            }));
        }
        if let Reference::Pointer(pointer) = value.reference {
            let previous_value =
                self.update_pointer(pointer, &value.ty, increment, value.span, span)?;
            return Ok(Value {
                span,
                ..previous_value
            }
            .into());
        }
        let ty = value.ty.clone();
        let pointer = self.temporary_pointer(&ty, span)?;
        self.consume_reference(value, &mut |parser, source| {
            let Reference::Pointer(source_pointer) = source.reference else {
                return Err(ParseError {
                    message: String::from("cannot modify a const value"),
                    span: source.span,
                });
            };
            let previous_value =
                parser.update_pointer(source_pointer, &source.ty, increment, source.span, span)?;
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
            ty,
            span,
            constant: ConstantValue::default(),
        }
        .into())
    }

    fn update_pointer(
        &mut self,
        pointer: Handle<Expression>,
        ty: &TypeResolution,
        increment: bool,
        reference_span: Span,
        span: Span,
    ) -> Result<Value, ParseError> {
        let previous_value = self.load_pointer(pointer, ty, reference_span)?;
        let one = self.append(Expression::Literal(Literal::I32(1)), span)?;
        let one = Value {
            handle: one,
            ty: scalar_resolution(Scalar::I32),
            span,
            constant: Literal::I32(1).into(),
        };
        let (left, right) = self.usual_arithmetic(previous_value.clone().into(), one.into())?;
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
                ty: left.ty,
                span,
                constant: ConstantValue::default(),
            }
            .into(),
            ty,
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
