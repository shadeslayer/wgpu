use alloc::{format, string::String};

use crate::{front::consume_any, Span};

use super::{preprocess::PreparedSource, ParseError};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum Token<'a> {
    Word(&'a str),
    Number(&'a str),
    String(&'a str),
    Separator(char),
    Paren(char),
    Operation(char),
    LogicalOperation(char),
    ShiftOperation(char),
    AssignmentOperation(char),
    IncrementOperation,
    DecrementOperation,
    Trivia,
    End,
}

pub(super) type TokenSpan<'a> = (Token<'a>, Span);

#[derive(Clone, Copy)]
struct ScannedToken<'a> {
    token: TokenSpan<'a>,
    input: &'a str,
    prepared_index: usize,
    end_offset: usize,
}

#[derive(Clone)]
pub(super) struct Lexer<'a> {
    source: &'a str,
    input: &'a str,
    prepared: Option<&'a PreparedSource>,
    prepared_index: usize,
    last_end_offset: usize,
    lookahead: Option<ScannedToken<'a>>,
}

impl<'a> Lexer<'a> {
    pub(super) fn new(source: &'a str) -> Result<Self, ParseError> {
        check_source(source)?;
        Ok(Self {
            source,
            input: source,
            prepared: None,
            prepared_index: 0,
            last_end_offset: 0,
            lookahead: None,
        })
    }

    pub(super) const fn from_prepared(source: &'a str, prepared: &'a PreparedSource) -> Self {
        Self {
            source,
            input: source,
            prepared: Some(prepared),
            prepared_index: 0,
            last_end_offset: 0,
            lookahead: None,
        }
    }

    pub(super) fn next(&mut self) -> Result<TokenSpan<'a>, ParseError> {
        let scanned = match self.lookahead.take() {
            Some(next) => next,
            None => self.scan()?,
        };
        self.input = scanned.input;
        self.prepared_index = scanned.prepared_index;
        self.last_end_offset = scanned.end_offset;
        Ok(scanned.token)
    }

    fn scan(&self) -> Result<ScannedToken<'a>, ParseError> {
        if let Some(prepared) = self.prepared {
            let Some(raw) = prepared.tokens.get(self.prepared_index) else {
                return Ok(ScannedToken {
                    token: (Token::End, Span::from(self.source.len()..self.source.len())),
                    input: self.input,
                    prepared_index: self.prepared_index,
                    end_offset: self.source.len(),
                });
            };
            let spelling = &prepared.spelling[raw.spelling.clone()];
            let (token, rest) = consume_token(spelling).map_err(|mut error| {
                error.span = raw.span;
                error
            })?;
            if !rest.is_empty() || matches!(token, Token::Trivia | Token::End) {
                return Err(ParseError {
                    message: String::from("unsupported CUDA preprocessing token"),
                    span: raw.span,
                });
            }
            return Ok(ScannedToken {
                token: (token, raw.span),
                input: self.input,
                prepared_index: self.prepared_index + 1,
                end_offset: raw.span.to_range().unwrap().end,
            });
        }
        let mut input = self.input;
        loop {
            let offset = self.source.len() - input.len();
            let (token, rest) = consume_token(input).map_err(|mut error| {
                let range = error.span.to_range().unwrap_or(0..0);
                error.span = Span::from(offset + range.start..offset + range.end);
                error
            })?;
            if token == Token::Trivia {
                input = rest;
                continue;
            }
            let end_offset = self.source.len() - rest.len();
            return Ok(ScannedToken {
                token: (token, Span::from(offset..end_offset)),
                input: rest,
                prepared_index: self.prepared_index,
                end_offset,
            });
        }
    }

    pub(super) fn peek(&mut self) -> Result<TokenSpan<'a>, ParseError> {
        let input = self.input;
        let prepared_index = self.prepared_index;
        let last_end_offset = self.last_end_offset;
        let token = self.next()?;
        self.lookahead = Some(ScannedToken {
            token,
            input: self.input,
            prepared_index: self.prepared_index,
            end_offset: self.last_end_offset,
        });
        self.input = input;
        self.prepared_index = prepared_index;
        self.last_end_offset = last_end_offset;
        Ok(token)
    }

    pub(super) fn next_if(&mut self, expected: Token<'_>) -> Result<bool, ParseError> {
        if self.peek()?.0 == expected {
            self.next()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub(super) fn expect_span(&mut self, expected: Token<'_>) -> Result<Span, ParseError> {
        let (token, span) = self.next()?;
        if token == expected {
            return Ok(span);
        }
        let message = match expected {
            Token::Word(text) => format!("expected `{text}`"),
            Token::Separator(character) | Token::Paren(character) | Token::Operation(character) => {
                format!("expected `{character}`")
            }
            Token::LogicalOperation(character) => {
                let second = if matches!(character, '&' | '|') {
                    character
                } else {
                    '='
                };
                format!("expected `{character}{second}`")
            }
            Token::ShiftOperation(character) => format!("expected `{character}{character}`"),
            Token::AssignmentOperation(character @ ('<' | '>')) => {
                format!("expected `{character}{character}=`")
            }
            Token::AssignmentOperation(character) => format!("expected `{character}=`"),
            Token::IncrementOperation => String::from("expected `++`"),
            Token::DecrementOperation => String::from("expected `--`"),
            Token::End => String::from("expected end of source"),
            _ => format!("expected {expected:?}"),
        };
        Err(ParseError { message, span })
    }

    pub(super) fn span_from(&self, offset: usize) -> Span {
        Span::from(offset..self.last_end_offset)
    }
}

fn check_source(source: &str) -> Result<(), ParseError> {
    u32::try_from(source.len()).map_err(|_| ParseError {
        message: String::from("CUDA source is too large"),
        span: Span::UNDEFINED,
    })?;
    let bytes = source.as_bytes();
    for (index, &byte) in bytes.iter().enumerate() {
        if byte == b'\\' && matches!(bytes.get(index + 1), Some(b'\n' | b'\r')) {
            return Err(ParseError {
                message: String::from("CUDA line continuations are unsupported"),
                span: Span::from(index..index + 2),
            });
        }
    }
    if let Some(index) = bytes.windows(3).position(|window| window == b"??/") {
        return Err(ParseError {
            message: String::from("CUDA trigraphs are unsupported"),
            span: Span::from(index..index + 3),
        });
    }
    Ok(())
}

fn consume_token(input: &str) -> Result<(Token<'_>, &str), ParseError> {
    let Some(character) = input.chars().next() else {
        return Ok((Token::End, input));
    };
    if character.is_ascii_whitespace() {
        return Ok((
            Token::Trivia,
            consume_any(input, |c| c.is_ascii_whitespace()).1,
        ));
    }
    if input.starts_with("//") {
        return Ok((
            Token::Trivia,
            consume_any(input, |c| !matches!(c, '\n' | '\r')).1,
        ));
    }
    if let Some(content) = input.strip_prefix("/*") {
        return match content.find("*/") {
            Some(end) => Ok((Token::Trivia, &content[end + 2..])),
            None => Err(ParseError {
                message: String::from("unterminated block comment"),
                span: Span::from(0..input.len()),
            }),
        };
    }
    if character.is_ascii_alphabetic() || character == '_' {
        let (word, rest) = consume_any(input, |c| c.is_ascii_alphanumeric() || c == '_');
        return Ok((Token::Word(word), rest));
    }
    if character.is_ascii_digit()
        || (character == '.' && input.as_bytes().get(1).is_some_and(u8::is_ascii_digit))
    {
        let is_number_part = |c: char| c.is_ascii_alphanumeric() || matches!(c, '.' | '_');
        let (mut number, mut rest) = consume_any(input, is_number_part);
        loop {
            let exponent_sign = number
                .strip_suffix(['e', 'E', 'p', 'P'])
                .is_some_and(|prefix| !prefix.ends_with('\''))
                && rest.starts_with(['+', '-']);
            let separator = rest.starts_with('\'')
                && rest
                    .as_bytes()
                    .get(1)
                    .is_some_and(|c| c.is_ascii_alphanumeric() || *c == b'_');
            if !exponent_sign && !separator {
                break;
            }
            rest = consume_any(&rest[1..], is_number_part).1;
            number = &input[..input.len() - rest.len()];
        }
        return Ok((Token::Number(number), rest));
    }
    if let Some(content) = input.strip_prefix('"') {
        return match content.find(['"', '\\', '\n', '\r']) {
            Some(end) if content.as_bytes()[end] == b'"' => {
                Ok((Token::String(&content[..end]), &content[end + 1..]))
            }
            Some(end) => Err(ParseError {
                message: String::from("unsupported string literal"),
                span: Span::from(0..end + 2),
            }),
            None => Err(ParseError {
                message: String::from("unterminated string literal"),
                span: Span::from(0..input.len()),
            }),
        };
    }
    if input.starts_with("->") {
        return Err(ParseError {
            message: String::from("unsupported CUDA operator"),
            span: Span::from(0..2),
        });
    }
    let rest = &input[character.len_utf8()..];
    Ok(match character {
        ':' | ';' | ',' | '.' => (Token::Separator(character), rest),
        '(' | ')' | '{' | '}' | '[' | ']' => (Token::Paren(character), rest),
        '<' | '>' => {
            if let Some(rest) = rest.strip_prefix('=') {
                (Token::LogicalOperation(character), rest)
            } else if let Some(rest) = rest.strip_prefix(character) {
                if let Some(rest) = rest.strip_prefix('=') {
                    (Token::AssignmentOperation(character), rest)
                } else {
                    (Token::ShiftOperation(character), rest)
                }
            } else {
                (Token::Paren(character), rest)
            }
        }
        '+' | '-' => {
            if let Some(rest) = rest.strip_prefix(character) {
                let token = if character == '+' {
                    Token::IncrementOperation
                } else {
                    Token::DecrementOperation
                };
                (token, rest)
            } else if let Some(rest) = rest.strip_prefix('=') {
                (Token::AssignmentOperation(character), rest)
            } else {
                (Token::Operation(character), rest)
            }
        }
        '&' | '|' => {
            if let Some(rest) = rest.strip_prefix(character) {
                (Token::LogicalOperation(character), rest)
            } else if let Some(rest) = rest.strip_prefix('=') {
                (Token::AssignmentOperation(character), rest)
            } else {
                (Token::Operation(character), rest)
            }
        }
        '=' | '!' => {
            if let Some(rest) = rest.strip_prefix('=') {
                (Token::LogicalOperation(character), rest)
            } else {
                (Token::Operation(character), rest)
            }
        }
        '*' | '/' | '%' | '^' => {
            if let Some(rest) = rest.strip_prefix('=') {
                (Token::AssignmentOperation(character), rest)
            } else {
                (Token::Operation(character), rest)
            }
        }
        '~' | '?' => (Token::Operation(character), rest),
        _ => {
            return Err(ParseError {
                message: format!("unsupported character `{}`", character.escape_debug()),
                span: Span::from(0..character.len_utf8()),
            })
        }
    })
}

#[cfg(test)]
mod tests {
    use super::{Lexer, ParseError, Token, TokenSpan};

    fn lex(source: &str) -> Result<alloc::vec::Vec<TokenSpan<'_>>, ParseError> {
        let mut lexer = Lexer::new(source)?;
        let mut tokens = alloc::vec::Vec::new();
        loop {
            let token = lexer.next()?;
            tokens.push(token);
            if token.0 == Token::End {
                return Ok(tokens);
            }
        }
    }

    #[test]
    fn cuda_scanner_integer_separators_remain_inside_preprocessing_numbers() {
        let source = "1'000 0'77u 0xF'FU 0B1'010 1'0.0f 1'bad";
        let tokens = lex(source).unwrap();
        let numbers: alloc::vec::Vec<_> = tokens
            .iter()
            .filter_map(|token| match token.0 {
                Token::Number(text) => {
                    assert_eq!(&source[token.1.to_range().unwrap()], text);
                    Some(text)
                }
                Token::End => None,
                other => panic!("unexpected token: {other:?}"),
            })
            .collect();
        assert_eq!(
            numbers,
            ["1'000", "0'77u", "0xF'FU", "0B1'010", "1'0.0f", "1'bad"]
        );
    }

    #[test]
    fn cuda_scanner_preprocessing_numbers_include_binary_exponent_signs() {
        for text in [
            "0x1e+2", "0X1E-2", "0x1p+2f", "0X1P-2F", "1p+2", "1P-2", "1e+2foo", "1e+2p-3",
        ] {
            let mut lexer = Lexer::new(text).unwrap();
            let token = lexer.next().unwrap();
            assert_eq!(token.0, Token::Number(text), "{text}");
            assert_eq!(&text[token.1.to_range().unwrap()], text);
            assert_eq!(lexer.next().unwrap().0, Token::End);
        }
    }

    #[test]
    fn cuda_scanner_number_boundaries_preserve_comments_and_operators() {
        let source = "0x1e/* 漢 */+2 0x1f+3 1e-2f-4 0b1<<2";
        let kinds: alloc::vec::Vec<_> = lex(source).unwrap().iter().map(|token| token.0).collect();
        assert_eq!(
            kinds,
            [
                Token::Number("0x1e"),
                Token::Operation('+'),
                Token::Number("2"),
                Token::Number("0x1f"),
                Token::Operation('+'),
                Token::Number("3"),
                Token::Number("1e-2f"),
                Token::Operation('-'),
                Token::Number("4"),
                Token::Number("0b1"),
                Token::ShiftOperation('<'),
                Token::Number("2"),
                Token::End,
            ]
        );
    }

    #[test]
    fn cuda_scanner_quoted_exponent_digits_do_not_join_following_signs() {
        for (source, number, operator) in [("0x1'e+2", "0x1'e", '+'), ("0x1'E-2", "0x1'E", '-')] {
            let tokens = lex(source).unwrap();
            assert_eq!(
                tokens
                    .iter()
                    .map(|token| token.0)
                    .collect::<alloc::vec::Vec<_>>(),
                [
                    Token::Number(number),
                    Token::Operation(operator),
                    Token::Number("2"),
                    Token::End
                ]
            );
            assert_eq!(&source[tokens[0].1.to_range().unwrap()], number);
        }
    }

    #[test]
    fn cuda_scanner_is_lazy_and_repeats_lexical_errors() {
        let source = "first /* 漢 */ @";
        let mut lexer = Lexer::new(source).unwrap();
        assert_eq!(lexer.next().unwrap().0, Token::Word("first"));
        let checkpoint = lexer.clone();
        for error in [
            lexer.peek().unwrap_err(),
            lexer.next().unwrap_err(),
            lexer.peek().unwrap_err(),
        ] {
            assert_eq!(&source[error.span.to_range().unwrap()], "@");
        }
        assert_eq!(lexer.input, checkpoint.input);
        assert_eq!(lexer.last_end_offset, checkpoint.last_end_offset);
    }

    #[test]
    fn cuda_scanner_peek_and_failed_next_if_preserve_position() {
        let source = "  first /* 漢 */ += second // tail\r\n";
        let mut lexer = Lexer::new(source).unwrap();
        for _ in 0..3 {
            let token = lexer.peek().unwrap();
            assert_eq!(token.0, Token::Word("first"));
            assert_eq!(&source[token.1.to_range().unwrap()], "first");
            assert_eq!(lexer.input, source);
            assert_eq!(lexer.last_end_offset, 0);
        }
        assert!(!lexer.next_if(Token::Paren('(')).unwrap());
        assert_eq!(lexer.next().unwrap().0, Token::Word("first"));
        assert_eq!(lexer.last_end_offset, 7);
        let before = lexer.input;
        assert_eq!(lexer.peek().unwrap().0, Token::AssignmentOperation('+'));
        assert_eq!(lexer.input, before);
        assert!(!lexer.next_if(Token::Operation('+')).unwrap());
        assert!(lexer.next_if(Token::AssignmentOperation('+')).unwrap());
        assert_eq!(lexer.next().unwrap().0, Token::Word("second"));
        let last_end = lexer.last_end_offset;
        assert_eq!(lexer.peek().unwrap().0, Token::End);
        assert_eq!(lexer.last_end_offset, last_end);
        assert!(!lexer.next_if(Token::Separator(';')).unwrap());
        for _ in 0..3 {
            assert_eq!(
                lexer.next().unwrap().1,
                crate::Span::from(source.len()..source.len())
            );
        }
    }

    #[test]
    fn cuda_scanner_checkpoint_restores_cached_lookahead() {
        let source = "( /* 漢 */ int) value";
        let mut lexer = Lexer::new(source).unwrap();
        lexer.expect_span(Token::Paren('(')).unwrap();
        assert_eq!(lexer.peek().unwrap().0, Token::Word("int"));
        let checkpoint = lexer.clone();
        lexer.expect_span(Token::Word("int")).unwrap();
        lexer.expect_span(Token::Paren(')')).unwrap();
        assert_eq!(
            &source[lexer.span_from(0).to_range().unwrap()],
            "( /* 漢 */ int)"
        );
        lexer = checkpoint;
        assert_eq!(lexer.next().unwrap().0, Token::Word("int"));
        assert_eq!(lexer.next().unwrap().0, Token::Paren(')'));
        assert_eq!(lexer.next().unwrap().0, Token::Word("value"));
    }

    #[test]
    fn cuda_scanner_errors_rebase_after_multibyte_trivia() {
        for (tail, expected) in [
            ("漢", "漢"),
            ("->field", "->"),
            ("/* unterminated", "/* unterminated"),
            ("\"unterminated", "\"unterminated"),
            ("\"x\\q\"", "\"x\\"),
        ] {
            let source = alloc::format!("first /* 漢 */ {tail}");
            let mut lexer = Lexer::new(&source).unwrap();
            lexer.next().unwrap();
            let error = lexer.next().unwrap_err();
            assert_eq!(
                &source[error.span.to_range().unwrap()],
                expected,
                "{source}"
            );
        }
    }

    #[test]
    fn cuda_scanner_expectations_consume_mismatches_and_propagate_lexical_errors() {
        let source = "first /* 漢 */ second @";
        let mut lexer = Lexer::new(source).unwrap();
        lexer.next().unwrap();
        let error = lexer.expect_span(Token::Separator(';')).unwrap_err();
        assert_eq!(&source[error.span.to_range().unwrap()], "second");
        for error in [
            lexer.next_if(Token::End).unwrap_err(),
            lexer.expect_span(Token::End).unwrap_err(),
        ] {
            assert_eq!(&source[error.span.to_range().unwrap()], "@");
        }
    }

    #[test]
    fn cuda_scanner_comments_do_not_nest_and_eof_spans_are_stable() {
        let mut lexer = Lexer::new("/* outer /* inner */ value").unwrap();
        assert_eq!(lexer.next().unwrap().0, Token::Word("value"));
        for source in ["", " \r\n", "/* 漢 */ // tail\r\n"] {
            let mut lexer = Lexer::new(source).unwrap();
            for _ in 0..2 {
                let token = lexer.peek().unwrap();
                assert_eq!(token.0, Token::End);
                assert_eq!(token.1, crate::Span::from(source.len()..source.len()));
                assert_eq!(lexer.next().unwrap().0, Token::End);
            }
        }
    }

    #[test]
    fn cuda_scanner_phase_checks_are_upfront() {
        for (source, offending) in [("@ // hidden \\\n", "\\\n"), ("@ /* ??/ */", "??/")] {
            let error = Lexer::new(source).err().unwrap();
            assert_eq!(&source[error.span.to_range().unwrap()], offending);
        }
    }

    #[test]
    fn cuda_comparison_and_logical_tokens() {
        let source = "a<=b && c!=d || e>=f == !g < h > i";
        let tokens = lex(source).unwrap();
        let kinds: alloc::vec::Vec<_> = tokens.iter().map(|token| token.0).collect();
        assert_eq!(
            kinds,
            [
                Token::Word("a"),
                Token::LogicalOperation('<'),
                Token::Word("b"),
                Token::LogicalOperation('&'),
                Token::Word("c"),
                Token::LogicalOperation('!'),
                Token::Word("d"),
                Token::LogicalOperation('|'),
                Token::Word("e"),
                Token::LogicalOperation('>'),
                Token::Word("f"),
                Token::LogicalOperation('='),
                Token::Operation('!'),
                Token::Word("g"),
                Token::Paren('<'),
                Token::Word("h"),
                Token::Paren('>'),
                Token::Word("i"),
                Token::End,
            ]
        );
        for (&(_, span), spelling) in tokens
            .iter()
            .filter(|token| matches!(token.0, Token::LogicalOperation(_)))
            .zip(["<=", "&&", "!=", "||", ">=", "=="])
        {
            assert_eq!(&source[span.to_range().unwrap()], spelling);
        }
    }

    #[test]
    fn cuda_operators_do_not_join_across_comments() {
        for (source, symbol) in [("a &/* split */& b", '&'), ("a | | b", '|')] {
            let tokens = lex(source).unwrap();
            assert_eq!(tokens[1].0, Token::Operation(symbol));
            assert_eq!(tokens[2].0, Token::Operation(symbol));
        }
        let tokens = lex("a < /* comment */ = b").unwrap();
        assert_eq!(tokens[1].0, Token::Paren('<'));
        assert_eq!(tokens[2].0, Token::Operation('='));
    }

    #[test]
    fn cuda_update_and_assignment_operators_use_maximal_munch() {
        let source = "a+++b---c+=d-=e*=f/=g";
        let tokens = lex(source).unwrap();
        let kinds: alloc::vec::Vec<_> = tokens.iter().map(|token| token.0).collect();
        assert_eq!(
            kinds,
            [
                Token::Word("a"),
                Token::IncrementOperation,
                Token::Operation('+'),
                Token::Word("b"),
                Token::DecrementOperation,
                Token::Operation('-'),
                Token::Word("c"),
                Token::AssignmentOperation('+'),
                Token::Word("d"),
                Token::AssignmentOperation('-'),
                Token::Word("e"),
                Token::AssignmentOperation('*'),
                Token::Word("f"),
                Token::AssignmentOperation('/'),
                Token::Word("g"),
                Token::End,
            ]
        );
        for (&(_, span), spelling) in tokens
            .iter()
            .filter(|token| {
                matches!(
                    token.0,
                    Token::AssignmentOperation(_)
                        | Token::IncrementOperation
                        | Token::DecrementOperation
                )
            })
            .zip(["++", "--", "+=", "-=", "*=", "/="])
        {
            assert_eq!(&source[span.to_range().unwrap()], spelling);
        }
    }

    #[test]
    fn cuda_update_operators_do_not_join_across_comments() {
        for (first, second) in [
            ('+', '+'),
            ('-', '-'),
            ('+', '='),
            ('-', '='),
            ('*', '='),
            ('/', '='),
            ('%', '='),
            ('&', '='),
            ('|', '='),
            ('^', '='),
            ('<', '<'),
            ('>', '>'),
        ] {
            for separator in [" /* split */ ", " // split\n "] {
                let source = alloc::format!("left {first}{separator}{second} right");
                let tokens = lex(&source).unwrap();
                assert_eq!(tokens.len(), 5);
                let first_token = if matches!(first, '<' | '>') {
                    Token::Paren(first)
                } else {
                    Token::Operation(first)
                };
                let second_token = if matches!(second, '<' | '>') {
                    Token::Paren(second)
                } else {
                    Token::Operation(second)
                };
                assert_eq!(tokens[1].0, first_token);
                assert_eq!(tokens[2].0, second_token);
                assert_eq!(
                    &source[tokens[1].1.to_range().unwrap()],
                    alloc::format!("{first}")
                );
                assert_eq!(
                    &source[tokens[2].1.to_range().unwrap()],
                    alloc::format!("{second}")
                );
            }
        }
    }

    #[test]
    fn cuda_arrow_operator_remains_unsupported() {
        let source = "pointer->field";
        let error = lex(source).unwrap_err();
        assert_eq!(error.message, "unsupported CUDA operator");
        assert_eq!(&source[error.span.to_range().unwrap()], "->");
    }

    #[test]
    fn cuda_integer_operators_use_maximal_munch() {
        for (source, expected) in [
            ("%", Token::Operation('%')),
            ("~", Token::Operation('~')),
            ("&", Token::Operation('&')),
            ("|", Token::Operation('|')),
            ("^", Token::Operation('^')),
            ("<<", Token::ShiftOperation('<')),
            (">>", Token::ShiftOperation('>')),
            ("%=", Token::AssignmentOperation('%')),
            ("&=", Token::AssignmentOperation('&')),
            ("|=", Token::AssignmentOperation('|')),
            ("^=", Token::AssignmentOperation('^')),
            ("<<=", Token::AssignmentOperation('<')),
            (">>=", Token::AssignmentOperation('>')),
        ] {
            let tokens = lex(source).unwrap();
            assert_eq!(tokens.len(), 2, "{source}");
            assert_eq!(tokens[0].0, expected);
            assert_eq!(&source[tokens[0].1.to_range().unwrap()], source);
            assert_eq!(tokens[1].0, Token::End);
        }
        let source = "<<=>>=<<>>";
        let tokens = lex(source).unwrap();
        let kinds: alloc::vec::Vec<_> = tokens.iter().map(|token| token.0).collect();
        assert_eq!(
            kinds,
            [
                Token::AssignmentOperation('<'),
                Token::AssignmentOperation('>'),
                Token::ShiftOperation('<'),
                Token::ShiftOperation('>'),
                Token::End,
            ]
        );
        for (&(_, span), spelling) in tokens
            .iter()
            .filter(|token| {
                matches!(
                    token.0,
                    Token::AssignmentOperation(_) | Token::ShiftOperation(_)
                )
            })
            .zip(["<<=", ">>=", "<<", ">>"])
        {
            assert_eq!(&source[span.to_range().unwrap()], spelling);
        }
    }

    #[test]
    fn cuda_shift_assignments_do_not_join_across_comments() {
        for (source, left, right, left_spelling, right_spelling) in [
            (
                "a < /* split */ <= b",
                Token::Paren('<'),
                Token::LogicalOperation('<'),
                "<",
                "<=",
            ),
            (
                "a > // split\n >= b",
                Token::Paren('>'),
                Token::LogicalOperation('>'),
                ">",
                ">=",
            ),
            (
                "a << /* split */ = b",
                Token::ShiftOperation('<'),
                Token::Operation('='),
                "<<",
                "=",
            ),
            (
                "a >> // split\n = b",
                Token::ShiftOperation('>'),
                Token::Operation('='),
                ">>",
                "=",
            ),
        ] {
            let tokens = lex(source).unwrap();
            assert_eq!(tokens.len(), 5, "{source}");
            assert_eq!(tokens[1].0, left);
            assert_eq!(tokens[2].0, right);
            assert_eq!(&source[tokens[1].1.to_range().unwrap()], left_spelling);
            assert_eq!(&source[tokens[2].1.to_range().unwrap()], right_spelling);
        }
    }
}
