use alloc::{string::String, vec::Vec};

use crate::Span;

use super::{error, Budget, ParseError};

#[derive(Clone, Copy, PartialEq)]
pub(super) enum Kind {
    Word,
    Number,
    Quoted,
    Punctuation,
    Newline,
}

#[derive(Clone)]
pub(super) struct RawToken {
    pub(super) kind: Kind,
    pub(super) spelling: String,
    pub(super) span: Span,
    pub(super) separated: bool,
    pub(super) line_start: bool,
}

#[derive(Clone, Copy)]
struct Character {
    value: char,
    start: usize,
    end: usize,
}

#[derive(Clone)]
struct Cursor<'a> {
    source: &'a str,
    offset: usize,
}

impl Cursor<'_> {
    fn raw_next(&mut self) -> Option<Character> {
        let start = self.offset;
        let mut value = self.source.get(start..)?.chars().next()?;
        self.offset += value.len_utf8();
        if value == '\r' {
            if self.source.as_bytes().get(self.offset) == Some(&b'\n') {
                self.offset += 1;
            }
            value = '\n';
        }
        Some(Character {
            value,
            start,
            end: self.offset,
        })
    }

    fn next(&mut self) -> Option<Character> {
        while self.source.as_bytes().get(self.offset) == Some(&b'\\') {
            let mut end = self.offset + 1;
            while self
                .source
                .as_bytes()
                .get(end)
                .is_some_and(|&byte| matches!(byte, b' ' | b'\t' | b'\x0b' | b'\x0c'))
            {
                end += 1;
            }
            match self.source.as_bytes().get(end).copied() {
                Some(b'\n') => self.offset = end + 1,
                Some(b'\r') => {
                    self.offset = end + 1;
                    if self.source.as_bytes().get(self.offset) == Some(&b'\n') {
                        self.offset += 1;
                    }
                }
                _ => break,
            }
        }
        self.raw_next()
    }

    fn peek(&self) -> Option<Character> {
        self.clone().next()
    }

    fn starts_with(&self, expected: &str) -> bool {
        let mut cursor = self.clone();
        expected
            .chars()
            .all(|value| cursor.next().is_some_and(|next| next.value == value))
    }
}

struct Spelling {
    text: String,
    start: usize,
    end: usize,
}

impl Spelling {
    const fn new(start: usize) -> Self {
        Self {
            text: String::new(),
            start,
            end: start,
        }
    }

    fn push(&mut self, character: Character, budget: &mut Budget) -> Result<(), ParseError> {
        let span = Span::from(character.start..character.end);
        budget.scan(character.end - self.end, span)?;
        budget.own(character.value.len_utf8(), span)?;
        self.text.push(character.value);
        self.end = character.end;
        Ok(())
    }

    fn span(&self) -> Span {
        Span::from(self.start..self.end)
    }
}

pub(super) struct RawLexer<'a> {
    cursor: Cursor<'a>,
    separated: bool,
    line_start: bool,
}

impl<'a> RawLexer<'a> {
    pub(super) fn new(source: &'a str) -> Self {
        Self {
            cursor: Cursor {
                source,
                offset: if source.starts_with('\u{feff}') { 3 } else { 0 },
            },
            separated: true,
            line_start: true,
        }
    }

    pub(super) fn next(&mut self, budget: &mut Budget) -> Result<Option<RawToken>, ParseError> {
        loop {
            let Some(character) = self.cursor.peek() else {
                budget.scan(
                    self.cursor.source.len() - self.cursor.offset,
                    Span::from(self.cursor.offset..self.cursor.source.len()),
                )?;
                self.cursor.offset = self.cursor.source.len();
                return Ok(None);
            };
            budget.scan(
                character.start - self.cursor.offset,
                Span::from(self.cursor.offset..character.start),
            )?;
            self.cursor.offset = character.start;
            if character.value == '\n' {
                self.cursor.next();
                budget.scan(
                    character.end - character.start,
                    Span::from(character.start..character.end),
                )?;
                self.separated = true;
                self.line_start = true;
                return Ok(Some(RawToken {
                    kind: Kind::Newline,
                    spelling: String::new(),
                    span: Span::from(character.start..character.end),
                    separated: true,
                    line_start: true,
                }));
            }
            if character.value.is_ascii_whitespace() {
                self.cursor.next();
                budget.scan(
                    character.end - character.start,
                    Span::from(character.start..character.end),
                )?;
                self.separated = true;
                continue;
            }
            if self.cursor.starts_with("//") {
                while let Some(next) = self.cursor.peek() {
                    if next.value == '\n' {
                        break;
                    }
                    let previous = self.cursor.offset;
                    self.cursor.next();
                    budget.scan(next.end - previous, Span::from(previous..next.end))?;
                }
                self.separated = true;
                continue;
            }
            if self.cursor.starts_with("/*") {
                let start = character.start;
                self.cursor.next();
                self.cursor.next();
                budget.scan(
                    self.cursor.offset - start,
                    Span::from(start..self.cursor.offset),
                )?;
                loop {
                    if self.cursor.starts_with("*/") {
                        let previous = self.cursor.offset;
                        self.cursor.next();
                        self.cursor.next();
                        budget.scan(
                            self.cursor.offset - previous,
                            Span::from(previous..self.cursor.offset),
                        )?;
                        break;
                    }
                    let previous = self.cursor.offset;
                    let next = self.cursor.next().ok_or_else(|| {
                        error(
                            "unterminated CUDA preprocessing block comment",
                            Span::from(start..self.cursor.source.len()),
                        )
                    })?;
                    budget.scan(next.end - previous, Span::from(previous..next.end))?;
                }
                self.separated = true;
                continue;
            }
            break;
        }
        let Some(first) = self.cursor.peek() else {
            return Ok(None);
        };
        budget.step(Span::from(first.start..first.end))?;
        let mut spelling = Spelling::new(first.start);
        let kind = if let Some(prefix) = ["u8R\"", "uR\"", "UR\"", "LR\"", "R\""]
            .into_iter()
            .find(|prefix| self.cursor.starts_with(prefix))
        {
            self.take(prefix.chars().count(), &mut spelling, budget)?;
            self.raw_literal(&mut spelling, budget)?;
            self.suffix(&mut spelling, budget)?;
            Kind::Quoted
        } else if let Some(prefix) = [
            "u8\"", "u8'", "u\"", "u'", "U\"", "U'", "L\"", "L'", "\"", "'",
        ]
        .into_iter()
        .find(|prefix| self.cursor.starts_with(prefix))
        {
            let quote = prefix.chars().last().unwrap();
            self.take(prefix.chars().count(), &mut spelling, budget)?;
            self.quoted(quote, &mut spelling, budget)?;
            self.suffix(&mut spelling, budget)?;
            Kind::Quoted
        } else if first.value.is_ascii_alphabetic() || first.value == '_' {
            self.suffix(&mut spelling, budget)?;
            Kind::Word
        } else if first.value.is_ascii_digit()
            || (first.value == '.' && {
                let mut cursor = self.cursor.clone();
                cursor.next();
                cursor
                    .peek()
                    .is_some_and(|next| next.value.is_ascii_digit())
            })
        {
            self.number(&mut spelling, budget)?;
            Kind::Number
        } else {
            let separate_less_than = self.cursor.starts_with("<::") && {
                let mut following = self.cursor.clone();
                for _ in 0..3 {
                    following.next();
                }
                !following
                    .peek()
                    .is_some_and(|next| matches!(next.value, ':' | '>'))
            };
            if !separate_less_than
                && ["<:", ":>", "<%", "%>", "%:"]
                    .iter()
                    .any(|text| self.cursor.starts_with(text))
            {
                return Err(error(
                    "CUDA preprocessing digraphs are unsupported",
                    Span::from(first.start..first.end),
                ));
            }
            if self.cursor.starts_with("\\u") || self.cursor.starts_with("\\U") {
                return Err(error(
                    "CUDA preprocessing universal character names are unsupported",
                    Span::from(first.start..first.end),
                ));
            }
            let punctuation = [
                ">>=", "<<=", "<=>", "->*", "...", "##", "::", ".*", "->", "++", "--", "<<", ">>",
                "<=", ">=", "==", "!=", "&&", "||", "*=", "/=", "%=", "+=", "-=", "&=", "^=", "|=",
            ]
            .into_iter()
            .find(|text| self.cursor.starts_with(text));
            self.take(punctuation.map_or(1, str::len), &mut spelling, budget)?;
            Kind::Punctuation
        };
        let span = spelling.span();
        let token = RawToken {
            kind,
            spelling: spelling.text,
            span,
            separated: self.separated,
            line_start: self.line_start,
        };
        self.separated = false;
        self.line_start = false;
        Ok(Some(token))
    }

    fn take(
        &mut self,
        count: usize,
        spelling: &mut Spelling,
        budget: &mut Budget,
    ) -> Result<(), ParseError> {
        for _ in 0..count {
            let character = self.cursor.next().unwrap();
            spelling.push(character, budget)?;
        }
        Ok(())
    }

    fn suffix(&mut self, spelling: &mut Spelling, budget: &mut Budget) -> Result<(), ParseError> {
        if !self
            .cursor
            .peek()
            .is_some_and(|next| next.value.is_ascii_alphabetic() || next.value == '_')
        {
            return Ok(());
        }
        while self
            .cursor
            .peek()
            .is_some_and(|next| next.value.is_ascii_alphanumeric() || next.value == '_')
        {
            self.take(1, spelling, budget)?;
        }
        Ok(())
    }

    fn number(&mut self, spelling: &mut Spelling, budget: &mut Budget) -> Result<(), ParseError> {
        while let Some(next) = self.cursor.peek() {
            let ordinary = next.value.is_ascii_alphanumeric() || matches!(next.value, '_' | '.');
            let sign = matches!(next.value, '+' | '-')
                && spelling
                    .text
                    .strip_suffix(['e', 'E', 'p', 'P'])
                    .is_some_and(|prefix| !prefix.ends_with('\''));
            let separator = next.value == '\'' && {
                let mut cursor = self.cursor.clone();
                cursor.next();
                cursor
                    .peek()
                    .is_some_and(|after| after.value.is_ascii_alphanumeric() || after.value == '_')
            };
            if !ordinary && !sign && !separator {
                break;
            }
            self.take(1, spelling, budget)?;
        }
        Ok(())
    }

    fn quoted(
        &mut self,
        quote: char,
        spelling: &mut Spelling,
        budget: &mut Budget,
    ) -> Result<(), ParseError> {
        loop {
            let next = self
                .cursor
                .next()
                .ok_or_else(|| error("unterminated CUDA preprocessing literal", spelling.span()))?;
            if next.value == '\n' {
                return Err(error(
                    "unterminated CUDA preprocessing literal",
                    spelling.span(),
                ));
            }
            spelling.push(next, budget)?;
            if next.value == quote {
                return Ok(());
            }
            if next.value == '\\' {
                let escaped = self.cursor.next().ok_or_else(|| {
                    error("unterminated CUDA preprocessing escape", spelling.span())
                })?;
                if escaped.value == '\n' {
                    return Err(error(
                        "unterminated CUDA preprocessing literal",
                        spelling.span(),
                    ));
                }
                spelling.push(escaped, budget)?;
            }
        }
    }

    fn raw_literal(
        &mut self,
        spelling: &mut Spelling,
        budget: &mut Budget,
    ) -> Result<(), ParseError> {
        // Raw literal delimiters and bodies retain characters removed by phase 2.
        let mut delimiter = Vec::new();
        loop {
            let next = self
                .cursor
                .raw_next()
                .ok_or_else(|| error("unterminated CUDA raw literal delimiter", spelling.span()))?;
            if next.value == '(' {
                spelling.push(next, budget)?;
                break;
            }
            if delimiter.len() == 16
                || !next.value.is_ascii_graphic()
                || matches!(next.value, ')' | '\\')
            {
                return Err(error(
                    "invalid CUDA raw literal delimiter",
                    Span::from(next.start..next.end),
                ));
            }
            delimiter.push(next.value as u8);
            spelling.push(next, budget)?;
        }
        let mut closing = String::from(")");
        for byte in delimiter {
            closing.push(byte as char);
        }
        closing.push('"');
        loop {
            if self.cursor.source[self.cursor.offset..].starts_with(&closing) {
                for _ in 0..closing.len() {
                    let next = self.cursor.raw_next().unwrap();
                    spelling.push(next, budget)?;
                }
                return Ok(());
            }
            let next = self
                .cursor
                .raw_next()
                .ok_or_else(|| error("unterminated CUDA raw literal", spelling.span()))?;
            spelling.push(next, budget)?;
        }
    }
}
