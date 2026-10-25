use alloc::{string::String, vec::Vec};
use core::ops::Range;

use crate::{FastHashMap, Span};

use super::ParseError;

mod conditions;
mod expansion;
mod lex;
mod literal;

use lex::{Kind, RawLexer, RawToken};

const MAX_DEPTH: usize = 64;
const MAX_DEFINITIONS: usize = 4096;
const MAX_TOKENS: usize = 1_048_576;
const MAX_STEPS: usize = 4_194_304;
const MAX_BYTES: usize = 67_108_864;

const PREDEFINED_NAMES: [&str; 11] = [
    "__LINE__",
    "__FILE__",
    "__DATE__",
    "__TIME__",
    "__cplusplus",
    "__STDC__",
    "__STDC_HOSTED__",
    "__STDCPP_DEFAULT_NEW_ALIGNMENT__",
    "__STDCPP_THREADS__",
    "__has_include",
    "__has_cpp_attribute",
];

pub(super) struct PreparedToken {
    pub(super) spelling: Range<usize>,
    pub(super) span: Span,
}

#[derive(Default)]
pub(super) struct PreparedSource {
    pub(super) spelling: String,
    pub(super) tokens: Vec<PreparedToken>,
}

#[derive(Default)]
struct Budget {
    steps: usize,
    owned_bytes: usize,
    visited_bytes: usize,
    replacement_tokens: usize,
    emitted_tokens: usize,
}

fn error(message: &str, span: Span) -> ParseError {
    ParseError {
        message: String::from(message),
        span,
    }
}

fn charge(count: &mut usize, amount: usize, limit: usize, span: Span) -> Result<(), ParseError> {
    *count = count
        .checked_add(amount)
        .filter(|&total| total <= limit)
        .ok_or_else(|| error("CUDA preprocessing resource limit exceeded", span))?;
    Ok(())
}

impl Budget {
    fn step(&mut self, span: Span) -> Result<(), ParseError> {
        charge(&mut self.steps, 1, MAX_STEPS, span)
    }

    fn own(&mut self, bytes: usize, span: Span) -> Result<(), ParseError> {
        charge(&mut self.owned_bytes, bytes, MAX_BYTES, span)
    }

    fn scan(&mut self, bytes: usize, span: Span) -> Result<(), ParseError> {
        charge(&mut self.steps, bytes, MAX_STEPS, span)
    }

    fn visit(&mut self, bytes: usize, span: Span) -> Result<(), ParseError> {
        charge(&mut self.visited_bytes, bytes, MAX_BYTES, span)
    }

    fn emit(&mut self, span: Span) -> Result<(), ParseError> {
        charge(&mut self.emitted_tokens, 1, MAX_TOKENS, span)
    }

    fn push<T>(&mut self, tokens: &mut Vec<T>, token: T, span: Span) -> Result<(), ParseError> {
        if tokens.len() == MAX_TOKENS {
            return Err(error("CUDA preprocessing token limit exceeded", span));
        }
        self.step(span)?;
        tokens.push(token);
        Ok(())
    }
}

#[derive(Clone, Copy)]
struct ExpandedToken<'a> {
    kind: Kind,
    spelling: &'a str,
    span: Span,
}

struct Definition {
    identifier: usize,
    parameters: Option<Vec<String>>,
    replacement: Vec<RawToken>,
    parameter_slots: Vec<Option<usize>>,
}

impl Definition {
    fn compatible(
        &self,
        parameters: &Option<Vec<&str>>,
        replacement: &[RawToken],
        budget: &mut Budget,
        span: Span,
    ) -> Result<bool, ParseError> {
        match (&self.parameters, parameters) {
            (&None, &None) => {}
            (&Some(ref previous), &Some(ref current)) => {
                if previous.len() != current.len() {
                    return Ok(false);
                }
                for (left, right) in previous.iter().zip(current) {
                    budget.step(span)?;
                    budget.visit(left.len(), span)?;
                    budget.visit(right.len(), span)?;
                    if left.as_str() != *right {
                        return Ok(false);
                    }
                }
            }
            (&None, &Some(_)) | (&Some(_), &None) => return Ok(false),
        }
        if self.replacement.len() != replacement.len() {
            return Ok(false);
        }
        for (index, (left, right)) in self.replacement.iter().zip(replacement).enumerate() {
            budget.step(span)?;
            budget.visit(left.spelling.len(), span)?;
            budget.visit(right.spelling.len(), span)?;
            if left.kind != right.kind
                || left.spelling != right.spelling
                || (index != 0 && left.separated != right.separated)
            {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

struct Conditional {
    parent_active: bool,
    active: bool,
    branch_taken: bool,
    seen_else: bool,
    span: Span,
}

struct Preprocessor<'a> {
    lexer: RawLexer<'a>,
    definitions: FastHashMap<String, Definition>,
    next_identifier: usize,
    conditionals: Vec<Conditional>,
    text: Vec<RawToken>,
    budget: Budget,
    output: PreparedSource,
}

fn needs_preparation(source: &str) -> bool {
    source.starts_with('\u{feff}')
        || source
            .bytes()
            .any(|byte| matches!(byte, b'#' | b'\\' | b'\r'))
        || ["<:", ":>", "<%", "%>", "%:"]
            .iter()
            .any(|&text| source.contains(text))
        || PREDEFINED_NAMES.iter().any(|&name| source.contains(name))
}

fn check_predefined(name: &str, span: Span) -> Result<(), ParseError> {
    if PREDEFINED_NAMES.contains(&name) {
        return Err(error(
            "CUDA predefined macro providers are unsupported",
            span,
        ));
    }
    Ok(())
}

pub(super) fn prepare(source: &str) -> Result<Option<PreparedSource>, ParseError> {
    if !needs_preparation(source) {
        return Ok(None);
    }
    if source.len() > MAX_BYTES {
        return Err(error(
            "CUDA preprocessing source is too large",
            Span::UNDEFINED,
        ));
    }
    if let Some(offset) = source
        .as_bytes()
        .windows(3)
        .position(|bytes| bytes == b"??/")
    {
        return Err(error(
            "CUDA trigraphs are unsupported",
            Span::from(offset..offset + 3),
        ));
    }
    let mut preprocessor = Preprocessor {
        lexer: RawLexer::new(source),
        definitions: FastHashMap::default(),
        next_identifier: 0,
        conditionals: Vec::new(),
        text: Vec::new(),
        budget: Budget::default(),
        output: PreparedSource::default(),
    };
    while let Some(token) = preprocessor.lexer.next(&mut preprocessor.budget)? {
        if token.kind == Kind::Newline {
            continue;
        }
        if token.line_start && token.spelling == "#" {
            preprocessor.flush(true)?;
            preprocessor.directive(token.span)?;
        } else if preprocessor.active() {
            let span = token.span;
            preprocessor
                .budget
                .push(&mut preprocessor.text, token, span)?;
        }
    }
    preprocessor.flush(false)?;
    if let Some(conditional) = preprocessor.conditionals.last() {
        return Err(error(
            "unterminated CUDA preprocessing conditional",
            conditional.span,
        ));
    }
    Ok(Some(preprocessor.output))
}

impl Preprocessor<'_> {
    fn flush(&mut self, directive: bool) -> Result<(), ParseError> {
        let output = &mut self.output;
        expansion::expand(
            &self.text,
            &self.definitions,
            &mut self.budget,
            expansion::Mode::Shader,
            directive,
            &mut |token| output.push(token),
        )?;
        self.text.clear();
        Ok(())
    }

    fn active(&self) -> bool {
        self.conditionals
            .last()
            .is_none_or(|conditional| conditional.active)
    }

    fn directive(&mut self, span: Span) -> Result<(), ParseError> {
        let mut tokens = Vec::new();
        while let Some(token) = self.lexer.next(&mut self.budget)? {
            if token.kind == Kind::Newline {
                break;
            }
            if tokens.len() == MAX_TOKENS + 2 {
                return Err(error("CUDA preprocessing token limit exceeded", token.span));
            }
            tokens.push(token);
        }
        let Some((directive, payload)) = tokens.split_first() else {
            return Ok(());
        };
        let name = directive.spelling.as_str();
        match name {
            "ifdef" | "ifndef" | "if" => {
                if self.conditionals.len() == MAX_DEPTH {
                    return Err(error(
                        "CUDA preprocessing conditional depth limit exceeded",
                        span,
                    ));
                }
                let parent_active = self.active();
                let selected = if !parent_active {
                    false
                } else if name == "if" {
                    conditions::evaluate(
                        payload,
                        &self.definitions,
                        &mut self.budget,
                        directive.span,
                    )?
                } else {
                    let operand = Self::operand(payload, directive.span)?;
                    self.budget.visit(operand.spelling.len(), operand.span)?;
                    check_predefined(&operand.spelling, operand.span)?;
                    self.definitions.contains_key(operand.spelling.as_str()) == (name == "ifdef")
                };
                self.conditionals.push(Conditional {
                    parent_active,
                    active: parent_active && selected,
                    branch_taken: selected,
                    seen_else: false,
                    span,
                });
            }
            "elif" | "elifdef" | "elifndef" => {
                let conditional = self
                    .conditionals
                    .last()
                    .ok_or_else(|| error("unmatched CUDA preprocessing alternative", span))?;
                if conditional.seen_else {
                    return Err(error("CUDA preprocessing alternative after #else", span));
                }
                let needed = conditional.parent_active && !conditional.branch_taken;
                let selected = if !needed {
                    false
                } else if name == "elif" {
                    conditions::evaluate(
                        payload,
                        &self.definitions,
                        &mut self.budget,
                        directive.span,
                    )?
                } else {
                    let operand = Self::operand(payload, directive.span)?;
                    self.budget.visit(operand.spelling.len(), operand.span)?;
                    check_predefined(&operand.spelling, operand.span)?;
                    self.definitions.contains_key(operand.spelling.as_str()) == (name == "elifdef")
                };
                let conditional = self.conditionals.last_mut().unwrap();
                conditional.active = selected;
                conditional.branch_taken |= selected;
            }
            "else" => {
                let conditional = self
                    .conditionals
                    .last_mut()
                    .ok_or_else(|| error("unmatched CUDA preprocessing #else", span))?;
                if conditional.seen_else {
                    return Err(error("duplicate CUDA preprocessing #else", span));
                }
                if conditional.parent_active && !payload.is_empty() {
                    return Err(error(
                        "unexpected tokens after CUDA preprocessing #else",
                        payload[0].span,
                    ));
                }
                conditional.seen_else = true;
                conditional.active = conditional.parent_active && !conditional.branch_taken;
                conditional.branch_taken |= conditional.active;
            }
            "endif" => {
                let conditional = self
                    .conditionals
                    .last()
                    .ok_or_else(|| error("unmatched CUDA preprocessing #endif", span))?;
                if conditional.parent_active && !payload.is_empty() {
                    return Err(error(
                        "unexpected tokens after CUDA preprocessing #endif",
                        payload[0].span,
                    ));
                }
                self.conditionals.pop();
            }
            _ if !self.active() => {}
            "define" => self.define(payload, directive.span)?,
            "undef" => {
                let operand = Self::operand(payload, directive.span)?;
                self.budget.visit(operand.spelling.len(), operand.span)?;
                check_predefined(&operand.spelling, operand.span)?;
                if operand.spelling == "defined" {
                    return Err(error("defined cannot be a CUDA macro name", operand.span));
                }
                self.definitions.remove(operand.spelling.as_str());
            }
            _ => {
                return Err(error(
                    "CUDA preprocessing directive is unsupported",
                    directive.span,
                ))
            }
        }
        Ok(())
    }

    fn operand(payload: &[RawToken], span: Span) -> Result<&RawToken, ParseError> {
        match payload {
            &[ref operand]
                if operand.kind == Kind::Word && !conditions::is_alternative(&operand.spelling) =>
            {
                Ok(operand)
            }
            _ => Err(error("expected one CUDA preprocessing macro name", span)),
        }
    }

    fn define(&mut self, payload: &[RawToken], span: Span) -> Result<(), ParseError> {
        let Some((name, replacement)) = payload.split_first() else {
            return Err(error("expected CUDA preprocessing macro name", span));
        };
        if name.kind != Kind::Word {
            return Err(error(
                "expected ASCII CUDA preprocessing macro name",
                name.span,
            ));
        }
        if name.spelling == "defined" || conditions::is_alternative(&name.spelling) {
            return Err(error(
                "reserved word cannot be a CUDA macro name",
                name.span,
            ));
        }
        self.budget.visit(name.spelling.len(), name.span)?;
        check_predefined(&name.spelling, name.span)?;
        let mut parameters = None;
        let mut parameter_indices = FastHashMap::default();
        let mut replacement = replacement;
        if let Some(first) = replacement.first().filter(|first| !first.separated) {
            if first.spelling != "(" {
                return Err(error(
                    "expected whitespace before CUDA macro replacement",
                    first.span,
                ));
            }
            let mut names = Vec::new();
            let mut index = 1;
            if replacement
                .get(index)
                .is_some_and(|token| token.spelling == ")")
            {
                index += 1;
            } else {
                loop {
                    let parameter = replacement.get(index).ok_or_else(|| {
                        error("unterminated CUDA macro parameter list", name.span)
                    })?;
                    self.budget.step(parameter.span)?;
                    self.budget
                        .visit(parameter.spelling.len(), parameter.span)?;
                    if parameter.kind != Kind::Word
                        || conditions::is_alternative(&parameter.spelling)
                        || matches!(parameter.spelling.as_str(), "__VA_ARGS__" | "__VA_OPT__")
                    {
                        return Err(error("unsupported CUDA macro parameter", parameter.span));
                    }
                    if parameter_indices
                        .insert(parameter.spelling.as_str(), names.len())
                        .is_some()
                    {
                        return Err(error("duplicate CUDA macro parameter", parameter.span));
                    }
                    self.budget
                        .push(&mut names, parameter.spelling.as_str(), parameter.span)?;
                    index += 1;
                    match replacement.get(index) {
                        Some(token) if token.spelling == ")" => {
                            index += 1;
                            break;
                        }
                        Some(token) if token.spelling == "," => index += 1,
                        Some(token) => {
                            return Err(error(
                                "expected comma or closing CUDA macro parenthesis",
                                token.span,
                            ))
                        }
                        None => {
                            return Err(error("unterminated CUDA macro parameter list", name.span))
                        }
                    }
                }
            }
            parameters = Some(names);
            replacement = &replacement[index..];
        }
        if let Some(token) = replacement.iter().find(|token| token.spelling == "##") {
            return Err(error(
                "CUDA preprocessing token pasting is unsupported",
                token.span,
            ));
        }
        if parameters.is_some() {
            if let Some(token) = replacement.iter().find(|token| {
                token.spelling == "#"
                    || (token.kind == Kind::Word
                        && matches!(token.spelling.as_str(), "__VA_ARGS__" | "__VA_OPT__"))
            }) {
                return Err(error(
                    "CUDA macro stringization and variadics are unsupported",
                    token.span,
                ));
            }
        }
        let mut parameter_slots = Vec::new();
        for token in replacement {
            self.budget.visit(token.spelling.len(), token.span)?;
            let slot = if token.kind == Kind::Word {
                parameter_indices.get(token.spelling.as_str()).copied()
            } else {
                None
            };
            self.budget.push(&mut parameter_slots, slot, token.span)?;
        }
        if let Some(previous) = self.definitions.get(name.spelling.as_str()) {
            if !previous.compatible(&parameters, replacement, &mut self.budget, name.span)? {
                return Err(error("conflicting CUDA macro redefinition", name.span));
            }
            return Ok(());
        }
        if self.definitions.len() == MAX_DEFINITIONS {
            return Err(error(
                "CUDA preprocessing macro definition limit exceeded",
                name.span,
            ));
        }
        charge(
            &mut self.budget.replacement_tokens,
            replacement.len() + parameters.as_ref().map_or(0, Vec::len),
            MAX_TOKENS,
            name.span,
        )?;
        self.budget.own(name.spelling.len(), name.span)?;
        for token in replacement {
            self.budget.visit(token.spelling.len(), token.span)?;
            self.budget.own(token.spelling.len(), token.span)?;
        }
        let parameters = if let Some(names) = parameters {
            let mut owned = Vec::new();
            for parameter in names {
                self.budget.visit(parameter.len(), name.span)?;
                self.budget.own(parameter.len(), name.span)?;
                self.budget
                    .push(&mut owned, String::from(parameter), name.span)?;
            }
            Some(owned)
        } else {
            None
        };
        self.budget.visit(name.spelling.len(), name.span)?;
        self.definitions.insert(
            name.spelling.clone(),
            Definition {
                identifier: self.next_identifier,
                parameters,
                replacement: replacement.to_vec(),
                parameter_slots,
            },
        );
        self.next_identifier += 1;
        Ok(())
    }
}

impl PreparedSource {
    fn push(&mut self, token: ExpandedToken<'_>) -> Result<(), ParseError> {
        let start = self.spelling.len();
        let end = start
            .checked_add(token.spelling.len())
            .filter(|&end| end <= MAX_BYTES)
            .ok_or_else(|| {
                error(
                    "CUDA preprocessing output spelling limit exceeded",
                    token.span,
                )
            })?;
        self.spelling.push_str(token.spelling);
        self.tokens.push(PreparedToken {
            spelling: start..end,
            span: token.span,
        });
        Ok(())
    }
}
