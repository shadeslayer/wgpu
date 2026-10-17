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

    pub(super) fn cast_type(&mut self) -> Result<Scalar, ParseError> {
        let is_const = self.take_ident("const");
        let scalar = self.scalar_type()?;
        self.const_suffix(is_const)?;
        Ok(scalar)
    }
}
