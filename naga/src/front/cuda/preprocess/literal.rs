use crate::Span;

use super::{error, Budget, ParseError};

// Signed and unsigned values retain the same fixed-width bit representation.
#[derive(Clone, Copy)]
pub(super) struct Value {
    pub(super) bits: u64,
    pub(super) unsigned: bool,
}

impl Value {
    pub(super) const fn signed(value: i64) -> Self {
        Self {
            bits: value as u64,
            unsigned: false,
        }
    }

    pub(super) const fn truth(self) -> bool {
        self.bits != 0
    }
}

pub(super) fn integer(
    spelling: &str,
    span: Span,
    budget: &mut Budget,
) -> Result<Value, ParseError> {
    let invalid = || error("invalid preprocessing integer literal", span);
    let bytes = spelling.as_bytes();
    let (radix, mut offset) = match spelling.get(..2) {
        Some("0x" | "0X") => (16u64, 2),
        Some("0b" | "0B") => (2u64, 2),
        _ if spelling.starts_with('0') => (8u64, 0),
        _ => (10u64, 0),
    };
    let mut bits = 0u64;
    let mut previous_digit = false;
    let mut any_digit = false;
    while let Some(&byte) = bytes.get(offset) {
        budget.step(span)?;
        if let Some(digit) = (byte as char).to_digit(radix as u32) {
            bits = bits
                .checked_mul(radix)
                .and_then(|value| value.checked_add(u64::from(digit)))
                .ok_or_else(|| error("preprocessing integer literal exceeds 64 bits", span))?;
            previous_digit = true;
            any_digit = true;
        } else if byte == b'\'' {
            if !previous_digit
                || !bytes
                    .get(offset + 1)
                    .is_some_and(|&next| (next as char).is_digit(radix as u32))
            {
                return Err(invalid());
            }
            previous_digit = false;
        } else {
            break;
        }
        offset += 1;
    }
    if !any_digit || !previous_digit {
        return Err(invalid());
    }
    let suffix = &spelling[offset..];
    let (width_suffix, explicitly_unsigned) = if let Some(width) = suffix.strip_prefix(['u', 'U']) {
        (width, true)
    } else if let Some(width) = suffix.strip_suffix(['u', 'U']) {
        (width, true)
    } else {
        (suffix, false)
    };
    if !matches!(width_suffix, "" | "l" | "L" | "ll" | "LL" | "z" | "Z") {
        return Err(invalid());
    }
    if !explicitly_unsigned && radix == 10 && bits > i64::MAX as u64 {
        return Err(error(
            "preprocessing decimal literal exceeds signed 64 bits",
            span,
        ));
    }
    Ok(Value {
        bits,
        unsigned: explicitly_unsigned || bits > i64::MAX as u64,
    })
}

pub(super) fn character(
    spelling: &str,
    span: Span,
    budget: &mut Budget,
) -> Result<Value, ParseError> {
    let unsupported = || error("unsupported preprocessing character literal", span);
    let body = spelling
        .strip_prefix('\'')
        .and_then(|body| body.strip_suffix('\''))
        .ok_or_else(unsupported)?;
    if let Some(escape) = body.strip_prefix('\\') {
        if escape.len() == 1 {
            let value = match escape.as_bytes()[0] {
                b'\'' => Some(39),
                b'"' => Some(34),
                b'?' => Some(63),
                b'\\' => Some(92),
                b'a' => Some(7),
                b'b' => Some(8),
                b'f' => Some(12),
                b'n' => Some(10),
                b'r' => Some(13),
                b't' => Some(9),
                b'v' => Some(11),
                _ => None,
            };
            if let Some(value) = value {
                budget.step(span)?;
                return Ok(Value::signed(value));
            }
        }
        let (digits, radix) = if let Some(digits) = escape.strip_prefix('x') {
            (digits, 16u32)
        } else if !escape.is_empty() && escape.len() <= 3 {
            (escape, 8u32)
        } else {
            return Err(unsupported());
        };
        if digits.is_empty() {
            return Err(unsupported());
        }
        let mut value = 0u32;
        for byte in digits.bytes() {
            budget.step(span)?;
            let digit = (byte as char).to_digit(radix).ok_or_else(unsupported)?;
            value = value
                .checked_mul(radix)
                .and_then(|value| value.checked_add(digit))
                .filter(|&value| value <= 127)
                .ok_or_else(unsupported)?;
        }
        return Ok(Value::signed(i64::from(value)));
    }
    if body.len() != 1 || !body.is_ascii() {
        return Err(unsupported());
    }
    budget.step(span)?;
    Ok(Value::signed(i64::from(body.as_bytes()[0])))
}
