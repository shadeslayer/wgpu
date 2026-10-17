use super::*;

impl Parser<'_> {
    pub(super) fn convert_value(
        &mut self,
        value: ScalarExpression,
        target: Scalar,
    ) -> Result<Value, ParseError> {
        let value = self.rvalue(value)?;
        if value.scalar == target {
            return Ok(value);
        }
        if target == Scalar::BOOL {
            return self.truthy(value.into());
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
        })
    }

    pub(super) fn usual_arithmetic(
        &mut self,
        left: ScalarExpression,
        right: ScalarExpression,
    ) -> Result<(Value, Value), ParseError> {
        let common = Self::arithmetic_scalar(left.scalar(), right.scalar());
        Ok((
            self.convert_value(left, common)?,
            self.convert_value(right, common)?,
        ))
    }

    pub(super) fn arithmetic_scalar(left: Scalar, right: Scalar) -> Scalar {
        let scalars = [left, right];
        if scalars.contains(&Scalar::F32) {
            Scalar::F32
        } else if scalars.contains(&Scalar::U32) {
            Scalar::U32
        } else {
            Scalar::I32
        }
    }

    pub(super) fn promote_integral(
        &mut self,
        value: ScalarExpression,
    ) -> Result<Value, ParseError> {
        let value = self.rvalue(value)?;
        if value.scalar == Scalar::BOOL {
            self.convert_value(value.into(), Scalar::I32)
        } else {
            Ok(value)
        }
    }

    pub(super) fn integral_operand(
        &mut self,
        value: ScalarExpression,
    ) -> Result<Value, ParseError> {
        if value.scalar().kind == ScalarKind::Float {
            return Err(ParseError {
                message: String::from("operator requires integral operands"),
                span: value.span(),
            });
        }
        self.promote_integral(value)
    }

    pub(super) fn binary_value(
        &mut self,
        operator: BinaryOperator,
        left: ScalarExpression,
        right: ScalarExpression,
    ) -> Result<Value, ParseError> {
        let span = left.span().until(&right.span());
        let (left, right) = match operator {
            BinaryOperator::ShiftLeft | BinaryOperator::ShiftRight => {
                let left = self.integral_operand(left)?;
                let right = self.integral_operand(right)?;
                let mut right = self.convert_value(right.into(), Scalar::U32)?;
                if operator == BinaryOperator::ShiftLeft {
                    // Keep wrapping shifts out of WGSL constant evaluation.
                    let ty = self.scalar_handle(Scalar::U32, right.span);
                    let local = self.function.ir.local_variables.append(
                        LocalVariable {
                            name: None,
                            ty,
                            init: None,
                        },
                        right.span,
                    );
                    let pointer = self.append(Expression::LocalVariable(local), right.span)?;
                    self.push_statement(
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
                self.usual_arithmetic(left.into(), right.into())?
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
        })
    }

    pub(super) fn cast_type(&mut self) -> Result<Scalar, ParseError> {
        let is_const = self.take_ident("const")?;
        let scalar = self.scalar_type()?;
        self.const_suffix(is_const)?;
        Ok(scalar)
    }
}
