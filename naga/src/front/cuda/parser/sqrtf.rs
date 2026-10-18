use super::*;

impl Parser<'_> {
    pub(super) fn sqrtf_function(&mut self, span: Span) -> Result<Handle<Function>, ParseError> {
        if let Some(function) = self.sqrtf_function {
            return Ok(function);
        }
        let caller = core::mem::replace(
            &mut self.function,
            FunctionContext::new(FunctionKind::Device),
        );
        let result = self.build_sqrtf_function(span);
        let helper = core::mem::replace(&mut self.function, caller);
        result?;
        let function = self.module.functions.append(helper.ir, span);
        self.sqrtf_function = Some(function);
        Ok(function)
    }

    fn build_sqrtf_function(&mut self, span: Span) -> Result<(), ParseError> {
        let scalar_ty = self.scalar_handle(Scalar::U32, span);
        self.function.ir.name = Some(String::from("_naga_sqrtf"));
        self.function.ir.arguments.push(FunctionArgument {
            name: Some(String::from("input_bits")),
            ty: scalar_ty,
            binding: None,
        });
        self.function.ir.result = Some(crate::FunctionResult {
            ty: scalar_ty,
            binding: None,
        });

        let input_bits = self.append(Expression::FunctionArgument(0), span)?;
        let zero = self.sqrtf_literal(0, span)?;
        let one = self.sqrtf_literal(1, span)?;
        let magnitude_mask = self.sqrtf_literal(0x7fff_ffff, span)?;
        let infinity = self.sqrtf_literal(0x7f80_0000, span)?;
        let magnitude = self.sqrtf_binary(BinaryOperator::And, input_bits, magnitude_mask, span)?;
        let is_zero = self.sqrtf_binary(BinaryOperator::Equal, magnitude, zero, span)?;
        let is_positive_infinity =
            self.sqrtf_binary(BinaryOperator::Equal, input_bits, infinity, span)?;
        let preserve_input = self.sqrtf_binary(
            BinaryOperator::LogicalOr,
            is_zero,
            is_positive_infinity,
            span,
        )?;
        self.sqrtf_return_if(preserve_input, input_bits, span);

        let sign_mask = self.sqrtf_literal(0x8000_0000, span)?;
        let sign = self.sqrtf_binary(BinaryOperator::And, input_bits, sign_mask, span)?;
        let is_negative = self.sqrtf_binary(BinaryOperator::NotEqual, sign, zero, span)?;
        let is_nan = self.sqrtf_binary(BinaryOperator::Greater, magnitude, infinity, span)?;
        let invalid = self.sqrtf_binary(BinaryOperator::LogicalOr, is_negative, is_nan, span)?;
        let quiet_nan = self.sqrtf_literal(0x7fc0_0000, span)?;
        self.sqrtf_return_if(invalid, quiet_nan, span);

        let fraction_mask = self.sqrtf_literal(0x007f_ffff, span)?;
        let implicit_bit = self.sqrtf_literal(0x0080_0000, span)?;
        let twenty_three = self.sqrtf_literal(23, span)?;
        let exponent_bias = self.sqrtf_literal(127, span)?;
        let fraction = self.sqrtf_binary(BinaryOperator::And, input_bits, fraction_mask, span)?;
        let exponent_bits =
            self.sqrtf_binary(BinaryOperator::ShiftRight, magnitude, twenty_three, span)?;
        let significand = self.sqrtf_local("significand", span)?;
        // Keep the unbiased exponent plus 254 so subnormal exponents stay unsigned.
        let adjusted_exponent = self.sqrtf_local("adjusted_exponent", span)?;
        let normal_significand =
            self.sqrtf_binary(BinaryOperator::InclusiveOr, fraction, implicit_bit, span)?;
        self.sqrtf_store(significand, normal_significand, span);
        let normal_exponent =
            self.sqrtf_binary(BinaryOperator::Add, exponent_bits, exponent_bias, span)?;
        self.sqrtf_store(adjusted_exponent, normal_exponent, span);

        let is_subnormal = self.sqrtf_binary(BinaryOperator::Equal, exponent_bits, zero, span)?;
        let ((), normalize) = self.capture_block(|parser| {
            let leading_zeros = parser.append(
                Expression::Math {
                    fun: crate::MathFunction::CountLeadingZeros,
                    arg: fraction,
                    arg1: None,
                    arg2: None,
                    arg3: None,
                },
                span,
            )?;
            let eight = parser.sqrtf_literal(8, span)?;
            let normalization_shift =
                parser.sqrtf_binary(BinaryOperator::Subtract, leading_zeros, eight, span)?;
            let normalized_significand = parser.sqrtf_binary(
                BinaryOperator::ShiftLeft,
                fraction,
                normalization_shift,
                span,
            )?;
            parser.sqrtf_store(significand, normalized_significand, span);
            let subnormal_bias = parser.sqrtf_literal(128, span)?;
            let normalized_exponent = parser.sqrtf_binary(
                BinaryOperator::Subtract,
                subnormal_bias,
                normalization_shift,
                span,
            )?;
            parser.sqrtf_store(adjusted_exponent, normalized_exponent, span);
            Ok(())
        })?;
        self.push_statement(
            Statement::If {
                condition: is_subnormal,
                accept: normalize,
                reject: crate::Block::new(),
            },
            span,
        );

        let normalized_significand = self.sqrtf_load(significand, span)?;
        let normalized_exponent = self.sqrtf_load(adjusted_exponent, span)?;
        let exponent_parity =
            self.sqrtf_binary(BinaryOperator::And, normalized_exponent, one, span)?;
        let lower_shift =
            self.sqrtf_binary(BinaryOperator::Add, twenty_three, exponent_parity, span)?;
        let nine = self.sqrtf_literal(9, span)?;
        let upper_shift =
            self.sqrtf_binary(BinaryOperator::Subtract, nine, exponent_parity, span)?;
        // The 48-bit radicand is significand << (23 + exponent parity).
        let upper_bits = self.sqrtf_binary(
            BinaryOperator::ShiftRight,
            normalized_significand,
            upper_shift,
            span,
        )?;
        let lower_bits = self.sqrtf_binary(
            BinaryOperator::ShiftLeft,
            normalized_significand,
            lower_shift,
            span,
        )?;
        let upper_word = self.sqrtf_local("upper_word", span)?;
        let lower_word = self.sqrtf_local("lower_word", span)?;
        let root = self.sqrtf_local("root", span)?;
        let remainder = self.sqrtf_local("remainder", span)?;
        let iteration = self.sqrtf_local("iteration", span)?;
        self.sqrtf_store(upper_word, upper_bits, span);
        self.sqrtf_store(lower_word, lower_bits, span);
        self.sqrtf_store(root, zero, span);
        self.sqrtf_store(remainder, zero, span);
        self.sqrtf_store(iteration, zero, span);

        let two = self.sqrtf_literal(2, span)?;
        let fourteen = self.sqrtf_literal(14, span)?;
        let thirty = self.sqrtf_literal(30, span)?;
        let upper_mask = self.sqrtf_literal(0xffff, span)?;
        let iteration_count = self.sqrtf_literal(24, span)?;
        let ((), body) = self.capture_block(|parser| {
            let current_iteration = parser.sqrtf_load(iteration, span)?;
            let finished = parser.sqrtf_binary(
                BinaryOperator::GreaterEqual,
                current_iteration,
                iteration_count,
                span,
            )?;
            let mut accept = crate::Block::new();
            accept.push(Statement::Break, span);
            parser.push_statement(
                Statement::If {
                    condition: finished,
                    accept,
                    reject: crate::Block::new(),
                },
                span,
            );

            let current_upper = parser.sqrtf_load(upper_word, span)?;
            let current_lower = parser.sqrtf_load(lower_word, span)?;
            let current_root = parser.sqrtf_load(root, span)?;
            let current_remainder = parser.sqrtf_load(remainder, span)?;
            let digit_pair =
                parser.sqrtf_binary(BinaryOperator::ShiftRight, current_upper, fourteen, span)?;
            let shifted_upper =
                parser.sqrtf_binary(BinaryOperator::ShiftLeft, current_upper, two, span)?;
            let carry_bits =
                parser.sqrtf_binary(BinaryOperator::ShiftRight, current_lower, thirty, span)?;
            let combined_upper = parser.sqrtf_binary(
                BinaryOperator::InclusiveOr,
                shifted_upper,
                carry_bits,
                span,
            )?;
            let next_upper =
                parser.sqrtf_binary(BinaryOperator::And, combined_upper, upper_mask, span)?;
            let next_lower =
                parser.sqrtf_binary(BinaryOperator::ShiftLeft, current_lower, two, span)?;
            let shifted_remainder =
                parser.sqrtf_binary(BinaryOperator::ShiftLeft, current_remainder, two, span)?;
            let expanded_remainder = parser.sqrtf_binary(
                BinaryOperator::InclusiveOr,
                shifted_remainder,
                digit_pair,
                span,
            )?;
            let shifted_trial =
                parser.sqrtf_binary(BinaryOperator::ShiftLeft, current_root, two, span)?;
            let trial =
                parser.sqrtf_binary(BinaryOperator::InclusiveOr, shifted_trial, one, span)?;
            let take_digit = parser.sqrtf_binary(
                BinaryOperator::GreaterEqual,
                expanded_remainder,
                trial,
                span,
            )?;
            let subtracted_remainder =
                parser.sqrtf_binary(BinaryOperator::Subtract, expanded_remainder, trial, span)?;
            let next_remainder = parser.append(
                Expression::Select {
                    condition: take_digit,
                    accept: subtracted_remainder,
                    reject: expanded_remainder,
                },
                span,
            )?;
            let doubled_root =
                parser.sqrtf_binary(BinaryOperator::ShiftLeft, current_root, one, span)?;
            let incremented_root =
                parser.sqrtf_binary(BinaryOperator::InclusiveOr, doubled_root, one, span)?;
            let next_root = parser.append(
                Expression::Select {
                    condition: take_digit,
                    accept: incremented_root,
                    reject: doubled_root,
                },
                span,
            )?;
            parser.sqrtf_store(upper_word, next_upper, span);
            parser.sqrtf_store(lower_word, next_lower, span);
            parser.sqrtf_store(remainder, next_remainder, span);
            parser.sqrtf_store(root, next_root, span);
            Ok(())
        })?;
        let ((), continuing) = self.capture_block(|parser| {
            let current_iteration = parser.sqrtf_load(iteration, span)?;
            let next_iteration =
                parser.sqrtf_binary(BinaryOperator::Add, current_iteration, one, span)?;
            parser.sqrtf_store(iteration, next_iteration, span);
            Ok(())
        })?;
        self.push_statement(
            Statement::Loop {
                body,
                continuing,
                break_if: None,
            },
            span,
        );

        let floor_root = self.sqrtf_load(root, span)?;
        let final_remainder = self.sqrtf_load(remainder, span)?;
        // Integer radicands cannot tie; the nearest root rounds up iff remainder > root.
        let round_up =
            self.sqrtf_binary(BinaryOperator::Greater, final_remainder, floor_root, span)?;
        let incremented_root = self.sqrtf_binary(BinaryOperator::Add, floor_root, one, span)?;
        let rounded_root = self.append(
            Expression::Select {
                condition: round_up,
                accept: incremented_root,
                reject: floor_root,
            },
            span,
        )?;
        let result_exponent =
            self.sqrtf_binary(BinaryOperator::ShiftRight, normalized_exponent, one, span)?;
        let fraction_bits =
            self.sqrtf_binary(BinaryOperator::And, rounded_root, fraction_mask, span)?;
        let exponent_bits = self.sqrtf_binary(
            BinaryOperator::ShiftLeft,
            result_exponent,
            twenty_three,
            span,
        )?;
        let result_bits = self.sqrtf_binary(
            BinaryOperator::InclusiveOr,
            exponent_bits,
            fraction_bits,
            span,
        )?;
        self.push_statement(
            Statement::Return {
                value: Some(result_bits),
            },
            span,
        );
        Ok(())
    }

    fn sqrtf_literal(&mut self, value: u32, span: Span) -> Result<Handle<Expression>, ParseError> {
        self.append(Expression::Literal(Literal::U32(value)), span)
    }

    fn sqrtf_binary(
        &mut self,
        op: BinaryOperator,
        left: Handle<Expression>,
        right: Handle<Expression>,
        span: Span,
    ) -> Result<Handle<Expression>, ParseError> {
        self.append(Expression::Binary { op, left, right }, span)
    }

    fn sqrtf_local(&mut self, name: &str, span: Span) -> Result<Handle<Expression>, ParseError> {
        let ty = self.scalar_handle(Scalar::U32, span);
        let local = self.function.ir.local_variables.append(
            LocalVariable {
                name: Some(String::from(name)),
                ty,
                init: None,
            },
            span,
        );
        self.append(Expression::LocalVariable(local), span)
    }

    fn sqrtf_load(
        &mut self,
        pointer: Handle<Expression>,
        span: Span,
    ) -> Result<Handle<Expression>, ParseError> {
        self.append(Expression::Load { pointer }, span)
    }

    fn sqrtf_store(&mut self, pointer: Handle<Expression>, value: Handle<Expression>, span: Span) {
        self.push_statement(Statement::Store { pointer, value }, span);
    }

    fn sqrtf_return_if(
        &mut self,
        condition: Handle<Expression>,
        value: Handle<Expression>,
        span: Span,
    ) {
        let mut accept = crate::Block::new();
        accept.push(Statement::Return { value: Some(value) }, span);
        self.push_statement(
            Statement::If {
                condition,
                accept,
                reject: crate::Block::new(),
            },
            span,
        );
    }
}
