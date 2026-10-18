use super::*;

impl Parser<'_> {
    pub(super) fn absolute_value(
        &mut self,
        value: Handle<Expression>,
        span: Span,
    ) -> Result<Handle<Expression>, ParseError> {
        let input_bits = self.append(
            Expression::As {
                expr: value,
                kind: ScalarKind::Uint,
                convert: None,
            },
            span,
        )?;
        let magnitude_mask = self.append(Expression::Literal(Literal::U32(0x7fff_ffff)), span)?;
        let magnitude = self.append(
            Expression::Binary {
                op: BinaryOperator::And,
                left: input_bits,
                right: magnitude_mask,
            },
            span,
        )?;
        let infinity = self.append(Expression::Literal(Literal::U32(0x7f80_0000)), span)?;
        let is_nan = self.append(
            Expression::Binary {
                op: BinaryOperator::Greater,
                left: magnitude,
                right: infinity,
            },
            span,
        )?;
        let quiet_nan = self.append(Expression::Literal(Literal::U32(0x7fc0_0000)), span)?;
        let result_bits = self.append(
            Expression::Select {
                condition: is_nan,
                accept: quiet_nan,
                reject: magnitude,
            },
            span,
        )?;
        // Keep non-finite results out of WGSL constant evaluation.
        let pointer = self.temporary_pointer(&scalar_resolution(Scalar::U32), span)?;
        self.push_statement(
            Statement::Store {
                pointer,
                value: result_bits,
            },
            span,
        );
        let result_bits = self.append(Expression::Load { pointer }, span)?;
        self.append(
            Expression::As {
                expr: result_bits,
                kind: ScalarKind::Float,
                convert: None,
            },
            span,
        )
    }
}
