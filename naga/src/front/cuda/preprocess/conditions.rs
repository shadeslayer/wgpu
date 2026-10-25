use alloc::{string::String, vec::Vec};

use crate::{BinaryOperator, FastHashMap, Span, UnaryOperator};

use super::{
    check_predefined, error, expand,
    lex::{Kind, RawToken},
    literal::{self, Value},
    Budget, Definition, ExpandedToken, ParseError, MAX_DEPTH,
};

pub(super) fn is_alternative(name: &str) -> bool {
    matches!(
        name,
        "and"
            | "or"
            | "not"
            | "bitand"
            | "bitor"
            | "xor"
            | "compl"
            | "not_eq"
            | "and_eq"
            | "or_eq"
            | "xor_eq"
    )
}

struct Query {
    start: usize,
    end: usize,
    value: bool,
    span: Span,
}

fn queries(
    tokens: &[RawToken],
    definitions: &FastHashMap<String, Definition>,
    budget: &mut Budget,
) -> Result<Vec<Query>, ParseError> {
    let mut queries = Vec::new();
    let mut index = 0;
    while let Some(token) = tokens.get(index) {
        budget.step(token.span)?;
        if token.kind != Kind::Word || token.spelling != "defined" {
            index += 1;
            continue;
        }
        let parenthesized = tokens
            .get(index + 1)
            .is_some_and(|next| next.spelling == "(");
        let operand_index = index + if parenthesized { 2 } else { 1 };
        let operand = tokens.get(operand_index).ok_or_else(|| {
            error(
                "expected preprocessing macro name after defined",
                token.span,
            )
        })?;
        if operand.kind != Kind::Word || is_alternative(&operand.spelling) {
            return Err(error(
                "expected preprocessing macro name after defined",
                operand.span,
            ));
        }
        let end = operand_index + if parenthesized { 2 } else { 1 };
        let last = if parenthesized {
            tokens
                .get(operand_index + 1)
                .filter(|next| next.spelling == ")")
                .ok_or_else(|| error("expected closing parenthesis after defined", token.span))?
        } else {
            operand
        };
        budget.visit(operand.spelling.len(), operand.span)?;
        check_predefined(&operand.spelling, operand.span)?;
        queries.push(Query {
            start: index,
            end,
            value: definitions.contains_key(operand.spelling.as_str()),
            span: token.span.until(&last.span),
        });
        index = end;
    }
    Ok(queries)
}

pub(super) fn evaluate<'a>(
    tokens: &'a [RawToken],
    definitions: &'a FastHashMap<String, Definition>,
    budget: &mut Budget,
    span: Span,
) -> Result<bool, ParseError> {
    let queries = queries(tokens, definitions, budget)?;
    let mut expanded = Vec::new();
    let mut query_index = 0;
    let mut index = 0;
    while let Some(token) = tokens.get(index) {
        if let Some(query) = queries
            .get(query_index)
            .filter(|query| query.start == index)
        {
            budget.emit(query.span)?;
            expanded.push(ExpandedToken {
                kind: Kind::Number,
                spelling: if query.value { "1" } else { "0" },
                span: query.span,
            });
            index = query.end;
            query_index += 1;
        } else {
            expand(token, definitions, budget, &mut |token| {
                if token.kind == Kind::Word && token.spelling == "defined" {
                    return Err(error(
                        "macro expansion producing defined is unsupported",
                        token.span,
                    ));
                }
                expanded.push(token);
                Ok(())
            })?;
            index += 1;
        }
    }
    let mut parser = Parser {
        tokens: &expanded,
        next: 0,
        budget,
        span,
    };
    let value = parser.conditional(true, 0)?;
    if let Some(token) = parser.peek() {
        return Err(error(
            "unexpected token in preprocessing condition",
            token.span,
        ));
    }
    Ok(value.truth())
}

struct Parser<'a, 'tokens, 'budget> {
    tokens: &'tokens [ExpandedToken<'a>],
    next: usize,
    budget: &'budget mut Budget,
    span: Span,
}

impl<'a> Parser<'a, '_, '_> {
    fn peek(&self) -> Option<ExpandedToken<'a>> {
        self.tokens.get(self.next).copied()
    }

    fn next(&mut self) -> Result<ExpandedToken<'a>, ParseError> {
        let token = self
            .tokens
            .get(self.next)
            .copied()
            .ok_or_else(|| error("expected preprocessing condition expression", self.span))?;
        self.budget.step(token.span)?;
        self.next += 1;
        Ok(token)
    }

    fn take(&mut self, spelling: &str) -> Result<Option<Span>, ParseError> {
        if self.peek().is_some_and(|token| token.spelling == spelling) {
            Ok(Some(self.next()?.span))
        } else {
            Ok(None)
        }
    }

    fn expect(&mut self, spelling: &str) -> Result<(), ParseError> {
        if self.take(spelling)?.is_some() {
            Ok(())
        } else {
            Err(error(
                "unexpected token in preprocessing condition",
                self.peek().map_or(self.span, |token| token.span),
            ))
        }
    }

    fn nested(depth: usize, span: Span) -> Result<usize, ParseError> {
        if depth == MAX_DEPTH {
            Err(error(
                "preprocessing condition nesting limit exceeded",
                span,
            ))
        } else {
            Ok(depth + 1)
        }
    }

    fn comma(&mut self, live: bool, depth: usize) -> Result<Value, ParseError> {
        let mut value = self.conditional(live, depth)?;
        while self.take(",")?.is_some() {
            value = self.conditional(live, depth)?;
        }
        Ok(value)
    }

    fn conditional(&mut self, live: bool, depth: usize) -> Result<Value, ParseError> {
        let condition = self.binary(1, live, depth)?;
        let Some(span) = self.take("?")? else {
            return Ok(condition);
        };
        let depth = Self::nested(depth, span)?;
        let accept = self.comma(live && condition.truth(), depth)?;
        self.expect(":")?;
        let reject = self.conditional(live && !condition.truth(), depth)?;
        Ok(Value {
            bits: if !live {
                0
            } else if condition.truth() {
                accept.bits
            } else {
                reject.bits
            },
            unsigned: accept.unsigned || reject.unsigned,
        })
    }

    fn binary(&mut self, minimum: u8, live: bool, depth: usize) -> Result<Value, ParseError> {
        let mut left = self.unary(live, depth)?;
        while let Some(token) = self.peek() {
            let Some((operator, precedence)) = binary_operator(token.spelling) else {
                break;
            };
            if precedence < minimum {
                break;
            }
            self.next()?;
            let right_live = live
                && match operator {
                    BinaryOperator::LogicalAnd => left.truth(),
                    BinaryOperator::LogicalOr => !left.truth(),
                    _ => true,
                };
            let right = self.binary(precedence + 1, right_live, depth)?;
            left = binary_value(operator, left, right, live, token.span)?;
        }
        Ok(left)
    }

    fn unary(&mut self, live: bool, depth: usize) -> Result<Value, ParseError> {
        let token = self.next()?;
        let operator = match token.spelling {
            "+" => Some(None),
            "-" => Some(Some(UnaryOperator::Negate)),
            "!" | "not" => Some(Some(UnaryOperator::LogicalNot)),
            "~" | "compl" => Some(Some(UnaryOperator::BitwiseNot)),
            _ => None,
        };
        if let Some(operator) = operator {
            let depth = Self::nested(depth, token.span)?;
            let operand = self.unary(live, depth)?;
            let Some(operator) = operator else {
                return Ok(operand);
            };
            if operator == UnaryOperator::LogicalNot {
                return Ok(Value::signed(i64::from(live && !operand.truth())));
            }
            if !live {
                return Ok(Value {
                    bits: 0,
                    unsigned: operand.unsigned,
                });
            }
            let bits = match operator {
                UnaryOperator::BitwiseNot => !operand.bits,
                UnaryOperator::Negate if operand.unsigned => operand.bits.wrapping_neg(),
                UnaryOperator::Negate => (operand.bits as i64).checked_neg().ok_or_else(|| {
                    error("signed overflow in preprocessing condition", token.span)
                })? as u64,
                UnaryOperator::LogicalNot => unreachable!(),
            };
            return Ok(Value {
                bits,
                unsigned: operand.unsigned,
            });
        }
        if token.spelling == "(" {
            let depth = Self::nested(depth, token.span)?;
            let value = self.comma(live, depth)?;
            self.expect(")")?;
            return Ok(value);
        }
        match token.kind {
            Kind::Number => literal::integer(token.spelling, token.span, self.budget),
            Kind::Quoted => literal::character(token.spelling, token.span, self.budget),
            Kind::Word if !is_alternative(token.spelling) => {
                Ok(Value::signed(i64::from(token.spelling == "true")))
            }
            _ => Err(error(
                "unsupported preprocessing condition expression",
                token.span,
            )),
        }
    }
}

fn binary_operator(spelling: &str) -> Option<(BinaryOperator, u8)> {
    Some(match spelling {
        "||" | "or" => (BinaryOperator::LogicalOr, 1),
        "&&" | "and" => (BinaryOperator::LogicalAnd, 2),
        "|" | "bitor" => (BinaryOperator::InclusiveOr, 3),
        "^" | "xor" => (BinaryOperator::ExclusiveOr, 4),
        "&" | "bitand" => (BinaryOperator::And, 5),
        "==" => (BinaryOperator::Equal, 6),
        "!=" | "not_eq" => (BinaryOperator::NotEqual, 6),
        "<" => (BinaryOperator::Less, 7),
        ">" => (BinaryOperator::Greater, 7),
        "<=" => (BinaryOperator::LessEqual, 7),
        ">=" => (BinaryOperator::GreaterEqual, 7),
        "<<" => (BinaryOperator::ShiftLeft, 8),
        ">>" => (BinaryOperator::ShiftRight, 8),
        "+" => (BinaryOperator::Add, 9),
        "-" => (BinaryOperator::Subtract, 9),
        "*" => (BinaryOperator::Multiply, 10),
        "/" => (BinaryOperator::Divide, 10),
        "%" => (BinaryOperator::Modulo, 10),
        _ => return None,
    })
}

fn binary_value(
    operator: BinaryOperator,
    left: Value,
    right: Value,
    live: bool,
    span: Span,
) -> Result<Value, ParseError> {
    let boolean = matches!(
        operator,
        BinaryOperator::LogicalAnd
            | BinaryOperator::LogicalOr
            | BinaryOperator::Equal
            | BinaryOperator::NotEqual
            | BinaryOperator::Less
            | BinaryOperator::LessEqual
            | BinaryOperator::Greater
            | BinaryOperator::GreaterEqual
    );
    let shift = matches!(
        operator,
        BinaryOperator::ShiftLeft | BinaryOperator::ShiftRight
    );
    let unsigned = !boolean
        && if shift {
            left.unsigned
        } else {
            left.unsigned || right.unsigned
        };
    if !live {
        // Unevaluated operands still determine the result type.
        return Ok(Value { bits: 0, unsigned });
    }
    if shift {
        if right.bits >= 64 {
            return Err(error(
                "invalid shift count in preprocessing condition",
                span,
            ));
        }
        let count = right.bits as u32;
        let bits = match operator {
            BinaryOperator::ShiftLeft => left.bits.wrapping_shl(count),
            BinaryOperator::ShiftRight if left.unsigned => left.bits >> count,
            BinaryOperator::ShiftRight => ((left.bits as i64) >> count) as u64,
            _ => unreachable!(),
        };
        return Ok(Value { bits, unsigned });
    }
    let common_unsigned = left.unsigned || right.unsigned;
    let ordering = if common_unsigned {
        left.bits.cmp(&right.bits)
    } else {
        (left.bits as i64).cmp(&(right.bits as i64))
    };
    let bits = match operator {
        BinaryOperator::LogicalAnd => u64::from(left.truth() && right.truth()),
        BinaryOperator::LogicalOr => u64::from(left.truth() || right.truth()),
        BinaryOperator::Equal => u64::from(left.bits == right.bits),
        BinaryOperator::NotEqual => u64::from(left.bits != right.bits),
        BinaryOperator::Less => u64::from(ordering.is_lt()),
        BinaryOperator::LessEqual => u64::from(ordering.is_le()),
        BinaryOperator::Greater => u64::from(ordering.is_gt()),
        BinaryOperator::GreaterEqual => u64::from(ordering.is_ge()),
        BinaryOperator::And => left.bits & right.bits,
        BinaryOperator::ExclusiveOr => left.bits ^ right.bits,
        BinaryOperator::InclusiveOr => left.bits | right.bits,
        _ if common_unsigned => match operator {
            BinaryOperator::Add => left.bits.wrapping_add(right.bits),
            BinaryOperator::Subtract => left.bits.wrapping_sub(right.bits),
            BinaryOperator::Multiply => left.bits.wrapping_mul(right.bits),
            BinaryOperator::Divide => left
                .bits
                .checked_div(right.bits)
                .ok_or_else(|| error("division by zero in preprocessing condition", span))?,
            BinaryOperator::Modulo => left
                .bits
                .checked_rem(right.bits)
                .ok_or_else(|| error("remainder by zero in preprocessing condition", span))?,
            _ => unreachable!(),
        },
        _ => {
            let left = left.bits as i64;
            let right = right.bits as i64;
            let value = match operator {
                BinaryOperator::Add => left.checked_add(right),
                BinaryOperator::Subtract => left.checked_sub(right),
                BinaryOperator::Multiply => left.checked_mul(right),
                BinaryOperator::Divide => left.checked_div(right),
                BinaryOperator::Modulo => left.checked_rem(right),
                _ => unreachable!(),
            }
            .ok_or_else(|| error("overflow or zero divisor in preprocessing condition", span))?;
            value as u64
        }
    };
    Ok(Value { bits, unsigned })
}
