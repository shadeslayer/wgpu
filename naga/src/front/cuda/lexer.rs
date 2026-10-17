use alloc::{format, string::String, vec::Vec};

use crate::Span;

use super::ParseError;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum TokenKind<'a> {
    Ident(&'a str),
    Number(&'a str),
    String(&'a str),
    Operator(&'a str),
    Symbol(char),
    End,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub span: Span,
}

pub(super) fn lex(source: &str) -> Result<Vec<Token<'_>>, ParseError> {
    let length = u32::try_from(source.len()).map_err(|_| ParseError {
        message: String::from("CUDA source is too large"),
        span: Span::UNDEFINED,
    })?;
    let bytes = source.as_bytes();
    for (index, &byte) in bytes.iter().enumerate() {
        if byte == b'\\' && matches!(bytes.get(index + 1), Some(b'\n' | b'\r')) {
            return Err(ParseError {
                message: String::from("CUDA line continuations are unsupported"),
                span: Span::new(index as u32, (index + 2) as u32),
            });
        }
    }
    if let Some(index) = bytes.windows(3).position(|window| window == b"??/") {
        return Err(ParseError {
            message: String::from("CUDA trigraphs are unsupported"),
            span: Span::new(index as u32, (index + 3) as u32),
        });
    }
    let mut cursor = 0;
    let mut tokens = Vec::new();
    while cursor < bytes.len() {
        let start = cursor;
        let byte = bytes[cursor];
        if byte.is_ascii_whitespace() {
            cursor += 1;
            continue;
        }
        if byte == b'/' && bytes.get(cursor + 1) == Some(&b'/') {
            cursor += 2;
            while cursor < bytes.len() && !matches!(bytes[cursor], b'\n' | b'\r') {
                cursor += 1;
            }
            continue;
        }
        if byte == b'/' && bytes.get(cursor + 1) == Some(&b'*') {
            cursor += 2;
            while cursor + 1 < bytes.len() && !(bytes[cursor] == b'*' && bytes[cursor + 1] == b'/')
            {
                cursor += 1;
            }
            if cursor + 1 >= bytes.len() {
                return Err(ParseError {
                    message: String::from("unterminated block comment"),
                    span: Span::new(start as u32, length),
                });
            }
            cursor += 2;
            continue;
        }
        let kind = if byte.is_ascii_alphabetic() || byte == b'_' {
            cursor += 1;
            while cursor < bytes.len()
                && (bytes[cursor].is_ascii_alphanumeric() || bytes[cursor] == b'_')
            {
                cursor += 1;
            }
            TokenKind::Ident(&source[start..cursor])
        } else if byte.is_ascii_digit()
            || (byte == b'.' && bytes.get(cursor + 1).is_some_and(u8::is_ascii_digit))
        {
            cursor += 1;
            while cursor < bytes.len() {
                let next = bytes[cursor];
                if next.is_ascii_alphanumeric()
                    || matches!(next, b'.' | b'_')
                    || (matches!(next, b'+' | b'-') && matches!(bytes[cursor - 1], b'e' | b'E'))
                {
                    cursor += 1;
                } else {
                    break;
                }
            }
            TokenKind::Number(&source[start..cursor])
        } else if byte == b'"' {
            cursor += 1;
            while cursor < bytes.len() && bytes[cursor] != b'"' {
                if matches!(bytes[cursor], b'\\' | b'\n' | b'\r') {
                    return Err(ParseError {
                        message: String::from("unsupported string literal"),
                        span: Span::new(start as u32, (cursor + 1) as u32),
                    });
                }
                cursor += 1;
            }
            if cursor == bytes.len() {
                return Err(ParseError {
                    message: String::from("unterminated string literal"),
                    span: Span::new(start as u32, length),
                });
            }
            cursor += 1;
            TokenKind::String(&source[start + 1..cursor - 1])
        } else if matches!(
            (byte, bytes.get(cursor + 1)),
            (b'=' | b'!' | b'<' | b'>', Some(b'=')) | (b'&', Some(b'&')) | (b'|', Some(b'|'))
        ) {
            cursor += 2;
            TokenKind::Operator(&source[start..cursor])
        } else if b"(){}[],;=.+-*/<>!".contains(&byte) {
            if let Some(&next) = bytes.get(cursor + 1) {
                if matches!(
                    (byte, next),
                    (b'+', b'+' | b'=') | (b'-', b'-' | b'=' | b'>') | (b'*' | b'/', b'=')
                ) {
                    return Err(ParseError {
                        message: String::from("unsupported CUDA operator"),
                        span: Span::new(start as u32, (start + 2) as u32),
                    });
                }
            }
            cursor += 1;
            TokenKind::Symbol(char::from(byte))
        } else {
            let character = source[cursor..].chars().next().ok_or_else(|| ParseError {
                message: String::from("unexpected end of source"),
                span: Span::new(start as u32, length),
            })?;
            return Err(ParseError {
                message: format!("unsupported character `{}`", character.escape_debug()),
                span: Span::new(start as u32, (start + character.len_utf8()) as u32),
            });
        };
        tokens.push(Token {
            kind,
            span: Span::new(start as u32, cursor as u32),
        });
    }
    tokens.push(Token {
        kind: TokenKind::End,
        span: Span::new(length, length),
    });
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::{lex, TokenKind};

    #[test]
    fn cuda_comparison_and_logical_tokens() {
        let source = "a<=b && c!=d || e>=f == !g < h > i";
        let tokens = lex(source).unwrap();
        let kinds: alloc::vec::Vec<_> = tokens.iter().map(|token| token.kind).collect();
        assert_eq!(
            kinds,
            [
                TokenKind::Ident("a"),
                TokenKind::Operator("<="),
                TokenKind::Ident("b"),
                TokenKind::Operator("&&"),
                TokenKind::Ident("c"),
                TokenKind::Operator("!="),
                TokenKind::Ident("d"),
                TokenKind::Operator("||"),
                TokenKind::Ident("e"),
                TokenKind::Operator(">="),
                TokenKind::Ident("f"),
                TokenKind::Operator("=="),
                TokenKind::Symbol('!'),
                TokenKind::Ident("g"),
                TokenKind::Symbol('<'),
                TokenKind::Ident("h"),
                TokenKind::Symbol('>'),
                TokenKind::Ident("i"),
                TokenKind::End,
            ]
        );
        for token in &tokens {
            if let TokenKind::Operator(operator) = token.kind {
                assert_eq!(&source[token.span.to_range().unwrap()], operator);
            }
        }
    }

    #[test]
    fn cuda_operators_do_not_join_across_comments() {
        for source in ["a &/* split */& b", "a | | b"] {
            assert!(lex(source).is_err());
        }
        let tokens = lex("a < /* comment */ = b").unwrap();
        assert_eq!(tokens[1].kind, TokenKind::Symbol('<'));
        assert_eq!(tokens[2].kind, TokenKind::Symbol('='));
    }
}
