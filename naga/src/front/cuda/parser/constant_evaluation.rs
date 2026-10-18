use crate::{
    proc::{ConstantEvaluator, ConstantEvaluatorError, ExpressionKindTracker, Layouter},
    BinaryOperator, Expression, Handle, Literal, Module, Scalar, Span, UnaryOperator,
};

#[allow(unused_imports)]
use num_traits::float::FloatCore as _;

pub(super) fn unary(op: UnaryOperator, value: Literal) -> Option<Literal> {
    if !supported_literal(value) {
        return None;
    }
    match (op, value) {
        (UnaryOperator::Negate, Literal::I32(value)) => {
            value.checked_neg()?;
        }
        (UnaryOperator::Negate, Literal::U32(_)) => {
            return binary(BinaryOperator::Subtract, Literal::U32(0), value);
        }
        _ => {}
    }
    evaluate(|evaluator| {
        let expr = evaluator.try_eval_and_append(Expression::Literal(value), Span::UNDEFINED)?;
        evaluator.try_eval_and_append(Expression::Unary { op, expr }, Span::UNDEFINED)
    })
}

pub(super) fn binary(op: BinaryOperator, left: Literal, mut right: Literal) -> Option<Literal> {
    if !supported_literal(left) || !supported_literal(right) {
        return None;
    }
    if matches!(op, BinaryOperator::ShiftLeft | BinaryOperator::ShiftRight) {
        let count = match right {
            Literal::I32(count) if (0..32).contains(&count) => count as u32,
            Literal::U32(count) if count < 32 => count,
            _ => return None,
        };
        if op == BinaryOperator::ShiftLeft {
            // C++20 shifts wrap; WGSL constant evaluation rejects lost bits.
            return match left {
                Literal::I32(value) => Some(Literal::I32(value.wrapping_shl(count))),
                Literal::U32(value) => Some(Literal::U32(value.wrapping_shl(count))),
                _ => None,
            };
        }
        right = Literal::U32(count);
    }
    match (op, left, right) {
        (BinaryOperator::Add, Literal::I32(left), Literal::I32(right)) => {
            left.checked_add(right)?;
        }
        (BinaryOperator::Subtract, Literal::I32(left), Literal::I32(right)) => {
            left.checked_sub(right)?;
        }
        (BinaryOperator::Multiply, Literal::I32(left), Literal::I32(right)) => {
            left.checked_mul(right)?;
        }
        (BinaryOperator::Divide, Literal::I32(left), Literal::I32(right)) => {
            left.checked_div(right)?;
        }
        (BinaryOperator::Modulo, Literal::I32(left), Literal::I32(right)) => {
            left.checked_rem(right)?;
        }
        (BinaryOperator::Divide | BinaryOperator::Modulo, Literal::U32(_), Literal::U32(0)) => {
            return None
        }
        (BinaryOperator::Divide, Literal::F32(_), Literal::F32(0.0)) => {
            return None;
        }
        (BinaryOperator::Modulo, Literal::F32(_), Literal::F32(_)) => return None,
        _ => {}
    }
    evaluate(|evaluator| {
        let left = evaluator.try_eval_and_append(Expression::Literal(left), Span::UNDEFINED)?;
        let right = evaluator.try_eval_and_append(Expression::Literal(right), Span::UNDEFINED)?;
        evaluator.try_eval_and_append(Expression::Binary { op, left, right }, Span::UNDEFINED)
    })
}

pub(super) fn convert(value: Literal, target: Scalar) -> Option<Literal> {
    if !supported_literal(value)
        || ![Scalar::BOOL, Scalar::I32, Scalar::U32, Scalar::F32].contains(&target)
    {
        return None;
    }
    if let Literal::F32(value) = value {
        let truncated = value.trunc();
        let in_range = match target {
            Scalar::I32 => (-2_147_483_648.0..2_147_483_648.0).contains(&truncated),
            Scalar::U32 => (0.0..4_294_967_296.0).contains(&truncated),
            _ => true,
        };
        if !in_range {
            return None;
        }
    }
    evaluate(|evaluator| {
        let expr = evaluator.try_eval_and_append(Expression::Literal(value), Span::UNDEFINED)?;
        evaluator.try_eval_and_append(
            Expression::As {
                expr,
                kind: target.kind,
                convert: Some(target.width),
            },
            Span::UNDEFINED,
        )
    })
}

const fn supported_literal(value: Literal) -> bool {
    match value {
        Literal::Bool(_) | Literal::I32(_) | Literal::U32(_) => true,
        Literal::F32(value) => value.is_finite(),
        _ => false,
    }
}

fn evaluate(
    operation: impl FnOnce(
        &mut ConstantEvaluator<'_>,
    ) -> Result<Handle<Expression>, ConstantEvaluatorError>,
) -> Option<Literal> {
    let mut module = Module::default();
    let mut expression_kind_tracker = ExpressionKindTracker::new();
    let mut layouter = Layouter::default();
    let handle = operation(&mut ConstantEvaluator::for_wgsl_module(
        &mut module,
        &mut expression_kind_tracker,
        &mut layouter,
        false,
    ))
    .ok()?;
    match module.global_expressions[handle] {
        Expression::Literal(value) if supported_literal(value) => Some(value),
        _ => None,
    }
}
