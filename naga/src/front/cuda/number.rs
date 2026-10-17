use alloc::borrow::Cow;

use crate::Literal;

pub(super) fn parse(input: &str) -> Option<Literal> {
    if matches!(input.get(..2), Some("0x" | "0X" | "0b" | "0B")) {
        return parse_integer(input);
    }
    if let Some(body) = input.strip_suffix(['f', 'F']) {
        if !body.contains(['.', 'e', 'E']) || body.contains('\'') {
            return None;
        }
        let value = body.parse::<f32>().ok()?;
        return value.is_finite().then_some(Literal::F32(value));
    }
    parse_integer(input)
}

pub(super) fn parse_integer(input: &str) -> Option<Literal> {
    let (body, unsigned) = match input.strip_suffix(['u', 'U']) {
        Some(body) => (body, true),
        None => (input, false),
    };
    let (digits, radix) = match body.get(..2) {
        Some("0x" | "0X") => (&body[2..], 16),
        Some("0b" | "0B") => (&body[2..], 2),
        _ if body.starts_with('0') => (body, 8),
        _ => (body, 10),
    };
    if !digits
        .split('\'')
        .all(|part| !part.is_empty() && part.chars().all(|c| c.is_ascii() && c.is_digit(radix)))
    {
        return None;
    }
    let digits = if digits.contains('\'') {
        Cow::Owned(digits.replace('\'', ""))
    } else {
        Cow::Borrowed(digits)
    };
    let value = u32::from_str_radix(&digits, radix).ok()?;
    if unsigned || (radix != 10 && value > i32::MAX as u32) {
        Some(Literal::U32(value))
    } else {
        i32::try_from(value).ok().map(Literal::I32)
    }
}
