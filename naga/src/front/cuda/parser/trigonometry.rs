use super::*;

// Fixed-point values use little-endian 16-bit digits with 176 fractional bits.
const DIGITS: u32 = 12;
const FRACTION_DIGITS: u32 = 11;
const DIGIT_BITS: u32 = 16;
const DIGIT_MASK: u32 = 0xffff;
// The constants have 320 and 176 fractional bits, respectively.
const TWO_OVER_PI: [u32; 20] = [
    53984, 16973, 28218, 46884, 50529, 57019, 25515, 65105, 36929, 15427, 38297, 56162, 56768,
    62772, 22481, 64551, 5417, 20036, 33646, 41721,
];
const PI_OVER_TWO: [u32; 12] = [
    5327, 39953, 20996, 14754, 440, 50455, 35212, 33897, 17105, 46404, 37407, 1,
];

impl Parser<'_> {
    pub(super) fn trigonometry(
        &mut self,
        value: Handle<Expression>,
        cosine: bool,
        span: Span,
    ) -> Result<Handle<Expression>, ParseError> {
        let function = self.trigonometry_function(span)?;
        let bits = self.append(
            Expression::As {
                expr: value,
                kind: ScalarKind::Uint,
                convert: None,
            },
            span,
        )?;
        let mode = self.trig_literal(u32::from(cosine), span)?;
        let result = self.trig_call(function, &[bits, mode], span)?;
        self.append(
            Expression::As {
                expr: result,
                kind: ScalarKind::Float,
                convert: None,
            },
            span,
        )
    }

    fn trigonometry_function(&mut self, span: Span) -> Result<Handle<Function>, ParseError> {
        if let Some(function) = self.trigonometry_function {
            return Ok(function);
        }
        let digits = self.trig_array_type(DIGITS, span)?;
        let product = self.trig_array_type(22, span)?;
        let multiply = self.build_trig_multiply([DIGITS, DIGITS], FRACTION_DIGITS, DIGITS, span)?;
        let reduce_multiply = self.build_trig_multiply([2, 20], 0, 22, span)?;
        let divide = self.build_trig_divide(digits, span)?;
        let combine = self.build_trig_combine(digits, span)?;
        let extract_product = self.build_trig_extract(product, 22, span)?;
        let extract_fixed = self.build_trig_extract(digits, DIGITS, span)?;
        let kernel = self.build_trig_kernel(digits, [multiply, divide, combine], span)?;
        let round = self.build_trig_round(digits, extract_fixed, span)?;
        let function = self.build_trig_entry(
            digits,
            [
                reduce_multiply,
                multiply,
                combine,
                extract_product,
                kernel,
                round,
            ],
            span,
        )?;
        self.trigonometry_function = Some(function);
        Ok(function)
    }

    fn build_trig_multiply(
        &mut self,
        counts: [u32; 2],
        skipped: u32,
        output_count: u32,
        span: Span,
    ) -> Result<Handle<Function>, ParseError> {
        let left_ty = self.trig_array_type(counts[0], span)?;
        let right_ty = self.trig_array_type(counts[1], span)?;
        let product_ty = self.trig_array_type(counts[0] + counts[1], span)?;
        let output_ty = self.trig_array_type(output_count, span)?;
        let name = if skipped == 0 {
            "_naga_trig_reduce_multiply"
        } else {
            "_naga_trig_multiply"
        };
        self.trig_function(
            name,
            &[("left", left_ty), ("right", right_ty)],
            output_ty,
            span,
            |parser, arguments| {
                let left = parser.trig_local("left_digits", left_ty, span)?;
                let right = parser.trig_local("right_digits", right_ty, span)?;
                parser.trig_store(left, arguments[0], span);
                parser.trig_store(right, arguments[1], span);
                let product = parser.trig_local("product", product_ty, span)?;
                let carry = parser.trig_u32_local("carry", span)?;
                parser.trig_loop("left_index", counts[0], span, |parser, left_index| {
                    let zero = parser.trig_literal(0, span)?;
                    parser.trig_store(carry, zero, span);
                    let left_digit = parser.trig_read(left, left_index, span)?;
                    parser.trig_loop("right_index", counts[1], span, |parser, right_index| {
                        let right_digit = parser.trig_read(right, right_index, span)?;
                        let index = parser.trig_binary(
                            BinaryOperator::Add,
                            left_index,
                            right_index,
                            span,
                        )?;
                        let previous = parser.trig_read(product, index, span)?;
                        let carried = parser.trig_load(carry, span)?;
                        // Base 2^16 keeps this multiply-add at or below u32::MAX.
                        let multiplied = parser.trig_binary(
                            BinaryOperator::Multiply,
                            left_digit,
                            right_digit,
                            span,
                        )?;
                        let accumulated =
                            parser.trig_binary(BinaryOperator::Add, multiplied, previous, span)?;
                        let accumulated =
                            parser.trig_binary(BinaryOperator::Add, accumulated, carried, span)?;
                        let digit = parser.trig_binary_literal(
                            BinaryOperator::And,
                            accumulated,
                            DIGIT_MASK,
                            span,
                        )?;
                        parser.trig_write(product, index, digit, span)?;
                        let next_carry = parser.trig_binary_literal(
                            BinaryOperator::ShiftRight,
                            accumulated,
                            DIGIT_BITS,
                            span,
                        )?;
                        parser.trig_store(carry, next_carry, span);
                        Ok(())
                    })?;
                    let index = parser.trig_binary_literal(
                        BinaryOperator::Add,
                        left_index,
                        counts[1],
                        span,
                    )?;
                    let carried = parser.trig_load(carry, span)?;
                    parser.trig_write(product, index, carried, span)
                })?;
                let output = parser.trig_local("result", output_ty, span)?;
                parser.trig_loop("result_index", output_count, span, |parser, index| {
                    let source =
                        parser.trig_binary_literal(BinaryOperator::Add, index, skipped, span)?;
                    let digit = parser.trig_read(product, source, span)?;
                    parser.trig_write(output, index, digit, span)
                })?;
                let value = parser.trig_load(output, span)?;
                parser.push_statement(Statement::Return { value: Some(value) }, span);
                Ok(())
            },
        )
    }

    fn build_trig_divide(
        &mut self,
        digits: Handle<Type>,
        span: Span,
    ) -> Result<Handle<Function>, ParseError> {
        let scalar = self.scalar_handle(Scalar::U32, span);
        self.trig_function(
            "_naga_trig_divide",
            &[("value", digits), ("divisor", scalar)],
            digits,
            span,
            |parser, arguments| {
                let source = parser.trig_local("source", digits, span)?;
                parser.trig_store(source, arguments[0], span);
                let output = parser.trig_local("result", digits, span)?;
                let remainder = parser.trig_u32_local("remainder", span)?;
                parser.trig_loop("step", DIGITS, span, |parser, step| {
                    let last = parser.trig_literal(DIGITS - 1, span)?;
                    let index = parser.trig_binary(BinaryOperator::Subtract, last, step, span)?;
                    let previous = parser.trig_load(remainder, span)?;
                    let shifted = parser.trig_binary_literal(
                        BinaryOperator::ShiftLeft,
                        previous,
                        DIGIT_BITS,
                        span,
                    )?;
                    let digit = parser.trig_read(source, index, span)?;
                    let current =
                        parser.trig_binary(BinaryOperator::InclusiveOr, shifted, digit, span)?;
                    let quotient =
                        parser.trig_binary(BinaryOperator::Divide, current, arguments[1], span)?;
                    parser.trig_write(output, index, quotient, span)?;
                    let next =
                        parser.trig_binary(BinaryOperator::Modulo, current, arguments[1], span)?;
                    parser.trig_store(remainder, next, span);
                    Ok(())
                })?;
                let value = parser.trig_load(output, span)?;
                parser.push_statement(Statement::Return { value: Some(value) }, span);
                Ok(())
            },
        )
    }

    fn build_trig_combine(
        &mut self,
        digits: Handle<Type>,
        span: Span,
    ) -> Result<Handle<Function>, ParseError> {
        let scalar = self.scalar_handle(Scalar::U32, span);
        self.trig_function(
            "_naga_trig_combine",
            &[("left", digits), ("right", digits), ("subtract", scalar)],
            digits,
            span,
            |parser, arguments| {
                let left = parser.trig_local("left_digits", digits, span)?;
                let right = parser.trig_local("right_digits", digits, span)?;
                parser.trig_store(left, arguments[0], span);
                parser.trig_store(right, arguments[1], span);
                let output = parser.trig_local("result", digits, span)?;
                let carry = parser.trig_u32_local("carry", span)?;
                let subtract =
                    parser.trig_binary_literal(BinaryOperator::NotEqual, arguments[2], 0, span)?;
                parser.trig_loop("digit_index", DIGITS, span, |parser, index| {
                    let left_digit = parser.trig_read(left, index, span)?;
                    let right_digit = parser.trig_read(right, index, span)?;
                    let carried = parser.trig_load(carry, span)?;
                    let right_digit =
                        parser.trig_binary(BinaryOperator::Add, right_digit, carried, span)?;
                    let difference = parser.trig_binary(
                        BinaryOperator::Subtract,
                        left_digit,
                        right_digit,
                        span,
                    )?;
                    let sum =
                        parser.trig_binary(BinaryOperator::Add, left_digit, right_digit, span)?;
                    let borrow =
                        parser.trig_binary(BinaryOperator::Less, left_digit, right_digit, span)?;
                    let one = parser.trig_literal(1, span)?;
                    let zero = parser.trig_literal(0, span)?;
                    let borrow = parser.trig_select(borrow, one, zero, span)?;
                    let overflow = parser.trig_binary_literal(
                        BinaryOperator::ShiftRight,
                        sum,
                        DIGIT_BITS,
                        span,
                    )?;
                    let next_carry = parser.trig_select(subtract, borrow, overflow, span)?;
                    let value = parser.trig_select(subtract, difference, sum, span)?;
                    let digit =
                        parser.trig_binary_literal(BinaryOperator::And, value, DIGIT_MASK, span)?;
                    parser.trig_write(output, index, digit, span)?;
                    parser.trig_store(carry, next_carry, span);
                    Ok(())
                })?;
                let value = parser.trig_load(output, span)?;
                parser.push_statement(Statement::Return { value: Some(value) }, span);
                Ok(())
            },
        )
    }

    fn build_trig_extract(
        &mut self,
        array: Handle<Type>,
        count: u32,
        span: Span,
    ) -> Result<Handle<Function>, ParseError> {
        let scalar = self.scalar_handle(Scalar::U32, span);
        let name = if count == DIGITS {
            "_naga_trig_extract_fixed"
        } else {
            "_naga_trig_extract_product"
        };
        self.trig_function(
            name,
            &[("digits", array), ("offset", scalar)],
            scalar,
            span,
            |parser, arguments| {
                let source = parser.trig_local("source", array, span)?;
                parser.trig_store(source, arguments[0], span);
                let index = parser.trig_binary_literal(
                    BinaryOperator::Divide,
                    arguments[1],
                    DIGIT_BITS,
                    span,
                )?;
                let offset = parser.trig_binary_literal(
                    BinaryOperator::Modulo,
                    arguments[1],
                    DIGIT_BITS,
                    span,
                )?;
                let low = parser.trig_u32_local("low", span)?;
                let high = parser.trig_u32_local("high", span)?;
                for (pointer, advance) in [(low, 0), (high, 1)] {
                    let index =
                        parser.trig_binary_literal(BinaryOperator::Add, index, advance, span)?;
                    let in_bounds =
                        parser.trig_binary_literal(BinaryOperator::Less, index, count, span)?;
                    let ((), accept) = parser.capture_block(|parser| {
                        let value = parser.trig_read(source, index, span)?;
                        parser.trig_store(pointer, value, span);
                        Ok(())
                    })?;
                    parser.push_statement(
                        Statement::If {
                            condition: in_bounds,
                            accept,
                            reject: crate::Block::new(),
                        },
                        span,
                    );
                }
                let low = parser.trig_load(low, span)?;
                let high = parser.trig_load(high, span)?;
                let low = parser.trig_binary(BinaryOperator::ShiftRight, low, offset, span)?;
                let width = parser.trig_literal(DIGIT_BITS, span)?;
                let high_shift =
                    parser.trig_binary(BinaryOperator::Subtract, width, offset, span)?;
                let high = parser.trig_binary(BinaryOperator::ShiftLeft, high, high_shift, span)?;
                let combined = parser.trig_binary(BinaryOperator::InclusiveOr, low, high, span)?;
                let value =
                    parser.trig_binary_literal(BinaryOperator::And, combined, DIGIT_MASK, span)?;
                parser.push_statement(Statement::Return { value: Some(value) }, span);
                Ok(())
            },
        )
    }

    fn build_trig_kernel(
        &mut self,
        digits: Handle<Type>,
        operations: [Handle<Function>; 3],
        span: Span,
    ) -> Result<Handle<Function>, ParseError> {
        let [multiply, divide, combine] = operations;
        let scalar = self.scalar_handle(Scalar::U32, span);
        self.trig_function(
            "_naga_trig_kernel",
            &[("angle", digits), ("sine", scalar)],
            digits,
            span,
            |parser, arguments| {
                let term = parser.trig_local("term", digits, span)?;
                let total = parser.trig_local("total", digits, span)?;
                let sine =
                    parser.trig_binary_literal(BinaryOperator::NotEqual, arguments[1], 0, span)?;
                let one = parser.trig_one(digits, span)?;
                let ((), accept) = parser.capture_block(|parser| {
                    parser.trig_store(term, arguments[0], span);
                    Ok(())
                })?;
                let ((), reject) = parser.capture_block(|parser| {
                    parser.trig_store(term, one, span);
                    Ok(())
                })?;
                parser.push_statement(
                    Statement::If {
                        condition: sine,
                        accept,
                        reject,
                    },
                    span,
                );
                let first = parser.trig_load(term, span)?;
                parser.trig_store(total, first, span);
                let square = parser.trig_call(multiply, &[arguments[0], arguments[0]], span)?;
                parser.trig_loop("step", 8, span, |parser, step| {
                    let iteration =
                        parser.trig_binary_literal(BinaryOperator::Add, step, 1, span)?;
                    let twice =
                        parser.trig_binary_literal(BinaryOperator::Multiply, iteration, 2, span)?;
                    let plus = parser.trig_binary_literal(BinaryOperator::Add, twice, 1, span)?;
                    let minus =
                        parser.trig_binary_literal(BinaryOperator::Subtract, twice, 1, span)?;
                    let factor = parser.trig_select(sine, plus, minus, span)?;
                    let divisor =
                        parser.trig_binary(BinaryOperator::Multiply, twice, factor, span)?;
                    let previous = parser.trig_load(term, span)?;
                    let multiplied = parser.trig_call(multiply, &[previous, square], span)?;
                    let next = parser.trig_call(divide, &[multiplied, divisor], span)?;
                    parser.trig_store(term, next, span);
                    let previous_total = parser.trig_load(total, span)?;
                    let subtract =
                        parser.trig_binary_literal(BinaryOperator::And, iteration, 1, span)?;
                    let next_total =
                        parser.trig_call(combine, &[previous_total, next, subtract], span)?;
                    parser.trig_store(total, next_total, span);
                    Ok(())
                })?;
                let value = parser.trig_load(total, span)?;
                parser.push_statement(Statement::Return { value: Some(value) }, span);
                Ok(())
            },
        )
    }

    fn build_trig_round(
        &mut self,
        digits: Handle<Type>,
        extract: Handle<Function>,
        span: Span,
    ) -> Result<Handle<Function>, ParseError> {
        let scalar = self.scalar_handle(Scalar::U32, span);
        self.trig_function(
            "_naga_trig_round",
            &[("digits", digits)],
            scalar,
            span,
            |parser, arguments| {
                let source = parser.trig_local("source", digits, span)?;
                parser.trig_store(source, arguments[0], span);
                let highest = parser.trig_u32_local("highest_bit", span)?;
                let nonzero = parser.trig_u32_local("nonzero", span)?;
                parser.trig_loop("digit_index", DIGITS, span, |parser, index| {
                    let digit = parser.trig_read(source, index, span)?;
                    let previous = parser.trig_load(nonzero, span)?;
                    let combined =
                        parser.trig_binary(BinaryOperator::InclusiveOr, previous, digit, span)?;
                    parser.trig_store(nonzero, combined, span);
                    let present =
                        parser.trig_binary_literal(BinaryOperator::NotEqual, digit, 0, span)?;
                    let ((), accept) = parser.capture_block(|parser| {
                        let leading = parser.append(
                            Expression::Math {
                                fun: crate::MathFunction::CountLeadingZeros,
                                arg: digit,
                                arg1: None,
                                arg2: None,
                                arg3: None,
                            },
                            span,
                        )?;
                        let last = parser.trig_literal(31, span)?;
                        let bit =
                            parser.trig_binary(BinaryOperator::Subtract, last, leading, span)?;
                        let base = parser.trig_binary_literal(
                            BinaryOperator::Multiply,
                            index,
                            DIGIT_BITS,
                            span,
                        )?;
                        let bit = parser.trig_binary(BinaryOperator::Add, base, bit, span)?;
                        parser.trig_store(highest, bit, span);
                        Ok(())
                    })?;
                    parser.push_statement(
                        Statement::If {
                            condition: present,
                            accept,
                            reject: crate::Block::new(),
                        },
                        span,
                    );
                    Ok(())
                })?;
                let nonzero = parser.trig_load(nonzero, span)?;
                let is_zero =
                    parser.trig_binary_literal(BinaryOperator::Equal, nonzero, 0, span)?;
                let zero = parser.trig_literal(0, span)?;
                parser.trig_return_if(is_zero, zero, span);
                let highest = parser.trig_load(highest, span)?;
                let normal =
                    parser.trig_binary_literal(BinaryOperator::GreaterEqual, highest, 50, span)?;
                let normal_shift =
                    parser.trig_binary_literal(BinaryOperator::Subtract, highest, 23, span)?;
                let subnormal_shift = parser.trig_literal(27, span)?;
                let shift = parser.trig_select(normal, normal_shift, subnormal_shift, span)?;
                let low = parser.trig_call(extract, &[arguments[0], shift], span)?;
                let high_shift =
                    parser.trig_binary_literal(BinaryOperator::Add, shift, DIGIT_BITS, span)?;
                let high = parser.trig_call(extract, &[arguments[0], high_shift], span)?;
                let high = parser.trig_binary_literal(
                    BinaryOperator::ShiftLeft,
                    high,
                    DIGIT_BITS,
                    span,
                )?;
                let retained = parser.trig_binary(BinaryOperator::InclusiveOr, low, high, span)?;
                let guard_offset =
                    parser.trig_binary_literal(BinaryOperator::Subtract, shift, 1, span)?;
                let guard_digit = parser.trig_call(extract, &[arguments[0], guard_offset], span)?;
                let guard =
                    parser.trig_binary_literal(BinaryOperator::And, guard_digit, 1, span)?;
                let guard = parser.trig_binary_literal(BinaryOperator::NotEqual, guard, 0, span)?;
                let guard_index = parser.trig_binary_literal(
                    BinaryOperator::Divide,
                    guard_offset,
                    DIGIT_BITS,
                    span,
                )?;
                let guard_bit = parser.trig_binary_literal(
                    BinaryOperator::Modulo,
                    guard_offset,
                    DIGIT_BITS,
                    span,
                )?;
                let one = parser.trig_literal(1, span)?;
                let guard_mask =
                    parser.trig_binary(BinaryOperator::ShiftLeft, one, guard_bit, span)?;
                let guard_mask =
                    parser.trig_binary_literal(BinaryOperator::Subtract, guard_mask, 1, span)?;
                let sticky = parser.trig_u32_local("sticky", span)?;
                parser.trig_loop("sticky_index", DIGITS, span, |parser, index| {
                    let below =
                        parser.trig_binary(BinaryOperator::Less, index, guard_index, span)?;
                    let at_guard =
                        parser.trig_binary(BinaryOperator::Equal, index, guard_index, span)?;
                    let zero = parser.trig_literal(0, span)?;
                    let partial = parser.trig_select(at_guard, guard_mask, zero, span)?;
                    let full = parser.trig_literal(DIGIT_MASK, span)?;
                    let mask = parser.trig_select(below, full, partial, span)?;
                    let digit = parser.trig_read(source, index, span)?;
                    let discarded = parser.trig_binary(BinaryOperator::And, digit, mask, span)?;
                    let previous = parser.trig_load(sticky, span)?;
                    let next = parser.trig_binary(
                        BinaryOperator::InclusiveOr,
                        previous,
                        discarded,
                        span,
                    )?;
                    parser.trig_store(sticky, next, span);
                    Ok(())
                })?;
                let sticky = parser.trig_load(sticky, span)?;
                let odd = parser.trig_binary_literal(BinaryOperator::And, retained, 1, span)?;
                let sticky = parser.trig_binary(BinaryOperator::InclusiveOr, sticky, odd, span)?;
                let sticky =
                    parser.trig_binary_literal(BinaryOperator::NotEqual, sticky, 0, span)?;
                let round_up =
                    parser.trig_binary(BinaryOperator::LogicalAnd, guard, sticky, span)?;
                let zero = parser.trig_literal(0, span)?;
                let one = parser.trig_literal(1, span)?;
                let increment = parser.trig_select(round_up, one, zero, span)?;
                let rounded = parser.trig_binary(BinaryOperator::Add, retained, increment, span)?;
                let subnormal =
                    parser.trig_binary_literal(BinaryOperator::Less, highest, 50, span)?;
                parser.trig_return_if(subnormal, rounded, span);
                let overflow =
                    parser.trig_binary_literal(BinaryOperator::Equal, rounded, 1 << 24, span)?;
                let shifted =
                    parser.trig_binary_literal(BinaryOperator::ShiftRight, rounded, 1, span)?;
                let significand = parser.trig_select(overflow, shifted, rounded, span)?;
                let exponent =
                    parser.trig_binary_literal(BinaryOperator::Subtract, highest, 49, span)?;
                let carried = parser.trig_binary_literal(BinaryOperator::Add, exponent, 1, span)?;
                let exponent = parser.trig_select(overflow, carried, exponent, span)?;
                let exponent =
                    parser.trig_binary_literal(BinaryOperator::ShiftLeft, exponent, 23, span)?;
                let fraction = parser.trig_binary_literal(
                    BinaryOperator::And,
                    significand,
                    0x7f_ffff,
                    span,
                )?;
                let value =
                    parser.trig_binary(BinaryOperator::InclusiveOr, exponent, fraction, span)?;
                parser.push_statement(Statement::Return { value: Some(value) }, span);
                Ok(())
            },
        )
    }

    fn build_trig_entry(
        &mut self,
        digits: Handle<Type>,
        functions: [Handle<Function>; 6],
        span: Span,
    ) -> Result<Handle<Function>, ParseError> {
        let [reduce_multiply, multiply, combine, extract, kernel, round] = functions;
        let scalar = self.scalar_handle(Scalar::U32, span);
        let significand_ty = self.trig_array_type(2, span)?;
        let reciprocal_ty = self.trig_array_type(20, span)?;
        self.trig_function(
            "_naga_trigonometry",
            &[("input_bits", scalar), ("cosine", scalar)],
            scalar,
            span,
            |parser, arguments| {
                let magnitude = parser.trig_binary_literal(
                    BinaryOperator::And,
                    arguments[0],
                    0x7fff_ffff,
                    span,
                )?;
                let invalid = parser.trig_binary_literal(
                    BinaryOperator::GreaterEqual,
                    magnitude,
                    0x7f80_0000,
                    span,
                )?;
                let nan = parser.trig_literal(0x7fc0_0000, span)?;
                parser.trig_return_if(invalid, nan, span);
                let cosine =
                    parser.trig_binary_literal(BinaryOperator::NotEqual, arguments[1], 0, span)?;
                let one_bits = parser.trig_literal(0x3f80_0000, span)?;
                let small_result = parser.trig_select(cosine, one_bits, arguments[0], span)?;
                let small = parser.trig_binary_literal(
                    BinaryOperator::LessEqual,
                    magnitude,
                    0x3900_0000,
                    span,
                )?;
                parser.trig_return_if(small, small_result, span);
                let exponent =
                    parser.trig_binary_literal(BinaryOperator::ShiftRight, magnitude, 23, span)?;
                let offset = parser.trig_literal(470, span)?;
                let shift = parser.trig_binary(BinaryOperator::Subtract, offset, exponent, span)?;
                let significand =
                    parser.trig_binary_literal(BinaryOperator::And, magnitude, 0x7f_ffff, span)?;
                let significand = parser.trig_binary_literal(
                    BinaryOperator::InclusiveOr,
                    significand,
                    0x80_0000,
                    span,
                )?;
                let low = parser.trig_binary_literal(
                    BinaryOperator::And,
                    significand,
                    DIGIT_MASK,
                    span,
                )?;
                let high = parser.trig_binary_literal(
                    BinaryOperator::ShiftRight,
                    significand,
                    DIGIT_BITS,
                    span,
                )?;
                let significand = parser.append(
                    Expression::Compose {
                        ty: significand_ty,
                        components: alloc::vec![low, high],
                    },
                    span,
                )?;
                let reciprocal = parser.trig_digits(reciprocal_ty, &TWO_OVER_PI, span)?;
                let product =
                    parser.trig_call(reduce_multiply, &[significand, reciprocal], span)?;
                let quadrant = parser.trig_u32_local("quadrant", span)?;
                let integer = parser.trig_call(extract, &[product, shift], span)?;
                let integer = parser.trig_binary_literal(BinaryOperator::And, integer, 3, span)?;
                parser.trig_store(quadrant, integer, span);
                let fraction = parser.trig_local("fraction", digits, span)?;
                let fraction_start =
                    parser.trig_binary_literal(BinaryOperator::Subtract, shift, 176, span)?;
                parser.trig_loop("fraction_index", FRACTION_DIGITS, span, |parser, index| {
                    let offset = parser.trig_binary_literal(
                        BinaryOperator::Multiply,
                        index,
                        DIGIT_BITS,
                        span,
                    )?;
                    let offset =
                        parser.trig_binary(BinaryOperator::Add, fraction_start, offset, span)?;
                    let digit = parser.trig_call(extract, &[product, offset], span)?;
                    parser.trig_write(fraction, index, digit, span)
                })?;
                let top = parser.trig_literal(FRACTION_DIGITS - 1, span)?;
                let top = parser.trig_read(fraction, top, span)?;
                let negative =
                    parser.trig_binary_literal(BinaryOperator::GreaterEqual, top, 0x8000, span)?;
                let one = parser.trig_one(digits, span)?;
                let ((), accept) = parser.capture_block(|parser| {
                    let previous = parser.trig_load(quadrant, span)?;
                    let next =
                        parser.trig_binary_literal(BinaryOperator::Add, previous, 1, span)?;
                    let next = parser.trig_binary_literal(BinaryOperator::And, next, 3, span)?;
                    parser.trig_store(quadrant, next, span);
                    let previous = parser.trig_load(fraction, span)?;
                    let subtract = parser.trig_literal(1, span)?;
                    let next = parser.trig_call(combine, &[one, previous, subtract], span)?;
                    parser.trig_store(fraction, next, span);
                    Ok(())
                })?;
                parser.push_statement(
                    Statement::If {
                        condition: negative,
                        accept,
                        reject: crate::Block::new(),
                    },
                    span,
                );
                let fraction = parser.trig_load(fraction, span)?;
                let pi_over_two = parser.trig_digits(digits, &PI_OVER_TWO, span)?;
                let angle = parser.trig_call(multiply, &[fraction, pi_over_two], span)?;
                let quadrant = parser.trig_load(quadrant, span)?;
                let parity = parser.trig_binary_literal(BinaryOperator::And, quadrant, 1, span)?;
                let sine_kernel =
                    parser.trig_binary(BinaryOperator::Equal, parity, arguments[1], span)?;
                let one = parser.trig_literal(1, span)?;
                let zero = parser.trig_literal(0, span)?;
                let mode = parser.trig_select(sine_kernel, one, zero, span)?;
                let evaluated = parser.trig_call(kernel, &[angle, mode], span)?;
                let rounded = parser.trig_call(round, &[evaluated], span)?;
                let negative =
                    parser.trig_binary(BinaryOperator::LogicalAnd, negative, sine_kernel, span)?;
                let sign_mask = parser.trig_literal(0x8000_0000, span)?;
                let sign = parser.trig_select(negative, sign_mask, zero, span)?;
                let cosine_quadrant =
                    parser.trig_binary_literal(BinaryOperator::Add, quadrant, 1, span)?;
                let rotated = parser.trig_select(cosine, cosine_quadrant, quadrant, span)?;
                let rotated = parser.trig_binary_literal(BinaryOperator::And, rotated, 2, span)?;
                let rotated =
                    parser.trig_binary_literal(BinaryOperator::ShiftLeft, rotated, 30, span)?;
                let input_sign = parser.trig_binary_literal(
                    BinaryOperator::And,
                    arguments[0],
                    0x8000_0000,
                    span,
                )?;
                let input_sign = parser.trig_select(cosine, zero, input_sign, span)?;
                let sign = parser.trig_binary(BinaryOperator::ExclusiveOr, sign, rotated, span)?;
                let sign =
                    parser.trig_binary(BinaryOperator::ExclusiveOr, sign, input_sign, span)?;
                let value = parser.trig_binary(BinaryOperator::InclusiveOr, rounded, sign, span)?;
                parser.push_statement(Statement::Return { value: Some(value) }, span);
                Ok(())
            },
        )
    }

    fn trig_function(
        &mut self,
        name: &str,
        parameters: &[(&str, Handle<Type>)],
        result: Handle<Type>,
        span: Span,
        build: impl FnOnce(&mut Self, &[Handle<Expression>]) -> Result<(), ParseError>,
    ) -> Result<Handle<Function>, ParseError> {
        let caller = core::mem::replace(
            &mut self.function,
            FunctionContext::new(FunctionKind::Device),
        );
        self.function.ir.name = Some(String::from(name));
        self.function.ir.result = Some(crate::FunctionResult {
            ty: result,
            binding: None,
        });
        let built: Result<(), ParseError> = (|| {
            let mut arguments = Vec::with_capacity(parameters.len());
            for (index, &(name, ty)) in parameters.iter().enumerate() {
                self.function.ir.arguments.push(FunctionArgument {
                    name: Some(String::from(name)),
                    ty,
                    binding: None,
                });
                arguments.push(self.append(Expression::FunctionArgument(index as u32), span)?);
            }
            build(self, &arguments)?;
            self.flush_emitter();
            Ok(())
        })();
        let function = core::mem::replace(&mut self.function, caller);
        built?;
        Ok(self.module.functions.append(function.ir, span))
    }

    fn trig_loop(
        &mut self,
        name: &str,
        count: u32,
        span: Span,
        build: impl FnOnce(&mut Self, Handle<Expression>) -> Result<(), ParseError>,
    ) -> Result<(), ParseError> {
        let counter = self.trig_u32_local(name, span)?;
        let zero = self.trig_literal(0, span)?;
        self.trig_store(counter, zero, span);
        let ((), body) = self.capture_block(|parser| {
            let index = parser.trig_load(counter, span)?;
            let finished =
                parser.trig_binary_literal(BinaryOperator::GreaterEqual, index, count, span)?;
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
            build(parser, index)
        })?;
        let ((), continuing) = self.capture_block(|parser| {
            let previous = parser.trig_load(counter, span)?;
            let next = parser.trig_binary_literal(BinaryOperator::Add, previous, 1, span)?;
            parser.trig_store(counter, next, span);
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
        Ok(())
    }

    fn trig_array_type(&mut self, count: u32, span: Span) -> Result<Handle<Type>, ParseError> {
        let scalar = self.scalar_handle(Scalar::U32, span);
        self.array_type(
            scalar,
            ArraySize::Constant(core::num::NonZeroU32::new(count).unwrap()),
            span,
            span,
        )
    }

    fn trig_local(
        &mut self,
        name: &str,
        ty: Handle<Type>,
        span: Span,
    ) -> Result<Handle<Expression>, ParseError> {
        let zero = self.append(Expression::ZeroValue(ty), span)?;
        let local = self.function.ir.local_variables.append(
            LocalVariable {
                name: Some(String::from(name)),
                ty,
                init: Some(zero),
            },
            span,
        );
        self.append(Expression::LocalVariable(local), span)
    }

    fn trig_u32_local(&mut self, name: &str, span: Span) -> Result<Handle<Expression>, ParseError> {
        let ty = self.scalar_handle(Scalar::U32, span);
        self.trig_local(name, ty, span)
    }

    fn trig_literal(&mut self, value: u32, span: Span) -> Result<Handle<Expression>, ParseError> {
        self.append(Expression::Literal(Literal::U32(value)), span)
    }

    fn trig_digits(
        &mut self,
        ty: Handle<Type>,
        digits: &[u32],
        span: Span,
    ) -> Result<Handle<Expression>, ParseError> {
        let components = digits
            .iter()
            .map(|&digit| self.trig_literal(digit, span))
            .collect::<Result<Vec<_>, _>>()?;
        self.append(Expression::Compose { ty, components }, span)
    }

    fn trig_one(&mut self, ty: Handle<Type>, span: Span) -> Result<Handle<Expression>, ParseError> {
        let mut digits = [0; DIGITS as usize];
        digits[FRACTION_DIGITS as usize] = 1;
        self.trig_digits(ty, &digits, span)
    }

    fn trig_binary(
        &mut self,
        op: BinaryOperator,
        left: Handle<Expression>,
        right: Handle<Expression>,
        span: Span,
    ) -> Result<Handle<Expression>, ParseError> {
        self.append(Expression::Binary { op, left, right }, span)
    }

    fn trig_binary_literal(
        &mut self,
        op: BinaryOperator,
        left: Handle<Expression>,
        right: u32,
        span: Span,
    ) -> Result<Handle<Expression>, ParseError> {
        let right = self.trig_literal(right, span)?;
        self.trig_binary(op, left, right, span)
    }

    fn trig_select(
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

    fn trig_load(
        &mut self,
        pointer: Handle<Expression>,
        span: Span,
    ) -> Result<Handle<Expression>, ParseError> {
        self.append(Expression::Load { pointer }, span)
    }

    fn trig_store(&mut self, pointer: Handle<Expression>, value: Handle<Expression>, span: Span) {
        self.push_statement(Statement::Store { pointer, value }, span);
    }

    fn trig_read(
        &mut self,
        base: Handle<Expression>,
        index: Handle<Expression>,
        span: Span,
    ) -> Result<Handle<Expression>, ParseError> {
        let pointer = self.append(Expression::Access { base, index }, span)?;
        self.trig_load(pointer, span)
    }

    fn trig_write(
        &mut self,
        base: Handle<Expression>,
        index: Handle<Expression>,
        value: Handle<Expression>,
        span: Span,
    ) -> Result<(), ParseError> {
        let pointer = self.append(Expression::Access { base, index }, span)?;
        self.trig_store(pointer, value, span);
        Ok(())
    }

    fn trig_call(
        &mut self,
        function: Handle<Function>,
        arguments: &[Handle<Expression>],
        span: Span,
    ) -> Result<Handle<Expression>, ParseError> {
        let result = self.append_call_result(function, span)?;
        self.push_statement(
            Statement::Call {
                function,
                arguments: arguments.to_vec(),
                result: Some(result),
            },
            span,
        );
        Ok(result)
    }

    fn trig_return_if(
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
