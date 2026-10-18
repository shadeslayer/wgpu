use super::*;

impl Parser<'_> {
    pub(super) fn floorf(
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
        let zero = self.floorf_literal(0, span)?;
        let one = self.floorf_literal(1, span)?;
        let magnitude_mask = self.floorf_literal(0x7fff_ffff, span)?;
        let sign_mask = self.floorf_literal(0x8000_0000, span)?;
        let magnitude =
            self.floorf_binary(BinaryOperator::And, input_bits, magnitude_mask, span)?;
        let sign = self.floorf_binary(BinaryOperator::And, input_bits, sign_mask, span)?;
        let is_negative = self.floorf_binary(BinaryOperator::NotEqual, sign, zero, span)?;

        let fraction_width = self.floorf_literal(23, span)?;
        let exponent_bits =
            self.floorf_binary(BinaryOperator::ShiftRight, magnitude, fraction_width, span)?;
        let exponent_offset = self.floorf_literal(150, span)?;
        let fractional_width = self.floorf_binary(
            BinaryOperator::Subtract,
            exponent_offset,
            exponent_bits,
            span,
        )?;
        let shift_mask = self.floorf_literal(31, span)?;
        // Select evaluates both sides, so every shift count must stay below 32.
        let shift = self.floorf_binary(BinaryOperator::And, fractional_width, shift_mask, span)?;
        let integral_unit = self.floorf_binary(BinaryOperator::ShiftLeft, one, shift, span)?;
        let fraction_mask =
            self.floorf_binary(BinaryOperator::Subtract, integral_unit, one, span)?;
        let discarded_bits =
            self.floorf_binary(BinaryOperator::And, input_bits, fraction_mask, span)?;
        let has_fraction =
            self.floorf_binary(BinaryOperator::NotEqual, discarded_bits, zero, span)?;
        let integral_mask = self.append(
            Expression::Unary {
                op: UnaryOperator::BitwiseNot,
                expr: fraction_mask,
            },
            span,
        )?;
        let truncated_bits =
            self.floorf_binary(BinaryOperator::And, input_bits, integral_mask, span)?;
        let adjusted_bits =
            self.floorf_binary(BinaryOperator::Add, truncated_bits, integral_unit, span)?;
        let round_down =
            self.floorf_binary(BinaryOperator::LogicalAnd, is_negative, has_fraction, span)?;
        let integral_bits = self.floorf_select(round_down, adjusted_bits, truncated_bits, span)?;

        let negative_one = self.floorf_literal(0xbf80_0000, span)?;
        let small_bits = self.floorf_select(is_negative, negative_one, zero, span)?;
        let is_zero = self.floorf_binary(BinaryOperator::Equal, magnitude, zero, span)?;
        let small_bits = self.floorf_select(is_zero, input_bits, small_bits, span)?;
        let one_bits = self.floorf_literal(0x3f80_0000, span)?;
        let is_small = self.floorf_binary(BinaryOperator::Less, magnitude, one_bits, span)?;
        let result_bits = self.floorf_select(is_small, small_bits, integral_bits, span)?;

        let integral_threshold = self.floorf_literal(0x4b00_0000, span)?;
        let is_large = self.floorf_binary(
            BinaryOperator::GreaterEqual,
            magnitude,
            integral_threshold,
            span,
        )?;
        let result_bits = self.floorf_select(is_large, input_bits, result_bits, span)?;
        let infinity = self.floorf_literal(0x7f80_0000, span)?;
        let is_nan = self.floorf_binary(BinaryOperator::Greater, magnitude, infinity, span)?;
        let quiet_nan = self.floorf_literal(0x7fc0_0000, span)?;
        let result_bits = self.floorf_select(is_nan, quiet_nan, result_bits, span)?;

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

    fn floorf_literal(&mut self, value: u32, span: Span) -> Result<Handle<Expression>, ParseError> {
        self.append(Expression::Literal(Literal::U32(value)), span)
    }

    fn floorf_binary(
        &mut self,
        op: BinaryOperator,
        left: Handle<Expression>,
        right: Handle<Expression>,
        span: Span,
    ) -> Result<Handle<Expression>, ParseError> {
        self.append(Expression::Binary { op, left, right }, span)
    }

    fn floorf_select(
        &mut self,
        condition: Handle<Expression>,
        accept: Handle<Expression>,
        reject: Handle<Expression>,
        span: Span,
    ) -> Result<Handle<Expression>, ParseError> {
        self.append(
            Expression::Select {
                condition,
                accept,
                reject,
            },
            span,
        )
    }
}
