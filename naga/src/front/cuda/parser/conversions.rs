use super::*;

impl Parser<'_> {
    pub(super) fn convert_value(
        &mut self,
        value: Value,
        target: Scalar,
    ) -> Result<Value, ParseError> {
        let value = self.rvalue(value)?;
        if value.scalar == target {
            return Ok(value);
        }
        if target == Scalar::BOOL {
            return self.truthy(value);
        }
        if ![Scalar::I32, Scalar::U32, Scalar::F32].contains(&target) {
            return Err(ParseError {
                message: String::from("unsupported scalar conversion"),
                span: value.span,
            });
        }
        let handle = self.append(
            Expression::As {
                expr: value.handle,
                kind: target.kind,
                convert: Some(target.width),
            },
            value.span,
        )?;
        Ok(Value {
            handle,
            scalar: target,
            span: value.span,
            lvalue: None,
        })
    }

    pub(super) fn usual_arithmetic(
        &mut self,
        left: Value,
        right: Value,
    ) -> Result<(Value, Value), ParseError> {
        let scalars = [left.scalar, right.scalar];
        let common = if scalars.contains(&Scalar::F32) {
            Scalar::F32
        } else if scalars.contains(&Scalar::U32) {
            Scalar::U32
        } else {
            Scalar::I32
        };
        Ok((
            self.convert_value(left, common)?,
            self.convert_value(right, common)?,
        ))
    }

    pub(super) fn promote_integral(&mut self, value: Value) -> Result<Value, ParseError> {
        let value = self.rvalue(value)?;
        if value.scalar == Scalar::BOOL {
            self.convert_value(value, Scalar::I32)
        } else {
            Ok(value)
        }
    }

    pub(super) fn integral_operand(&mut self, value: Value) -> Result<Value, ParseError> {
        if value.scalar.kind == ScalarKind::Float {
            return Err(ParseError {
                message: String::from("operator requires integral operands"),
                span: value.span,
            });
        }
        self.promote_integral(value)
    }

    pub(super) fn binary_value(
        &mut self,
        operator: BinaryOperator,
        left: Value,
        right: Value,
    ) -> Result<Value, ParseError> {
        let span = left.span.until(&right.span);
        let (left, right) = match operator {
            BinaryOperator::ShiftLeft | BinaryOperator::ShiftRight => {
                let left = self.integral_operand(left)?;
                let right = self.integral_operand(right)?;
                let mut right = self.convert_value(right, Scalar::U32)?;
                if operator == BinaryOperator::ShiftLeft {
                    // Keep wrapping shifts out of WGSL constant evaluation.
                    let ty = self.scalar_handle(Scalar::U32, right.span);
                    let local = self.function.local_variables.append(
                        LocalVariable {
                            name: None,
                            ty,
                            init: None,
                        },
                        right.span,
                    );
                    let pointer = self.append(Expression::LocalVariable(local), right.span)?;
                    self.function.body.push(
                        Statement::Store {
                            pointer,
                            value: right.handle,
                        },
                        right.span,
                    );
                    right.handle = self.append(Expression::Load { pointer }, right.span)?;
                }
                (left, right)
            }
            BinaryOperator::Modulo
            | BinaryOperator::And
            | BinaryOperator::InclusiveOr
            | BinaryOperator::ExclusiveOr => {
                let left = self.integral_operand(left)?;
                let right = self.integral_operand(right)?;
                self.usual_arithmetic(left, right)?
            }
            _ => self.usual_arithmetic(left, right)?,
        };
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
        Ok(Value {
            handle,
            scalar,
            span,
            lvalue: None,
        })
    }

    pub(super) fn cast_type(&mut self) -> Result<Scalar, ParseError> {
        let is_const = self.take_ident("const")?;
        let scalar = self.scalar_type()?;
        self.const_suffix(is_const)?;
        Ok(scalar)
    }
}
