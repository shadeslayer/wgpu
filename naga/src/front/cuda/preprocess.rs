use alloc::{string::String, vec::Vec};
use core::ops::Range;

use crate::{FastHashMap, Span};

use super::ParseError;

mod lex;

use lex::{Kind, RawLexer, RawToken};

const MAX_DEPTH: usize = 64;
const MAX_DEFINITIONS: usize = 4096;
const MAX_TOKENS: usize = 1_048_576;
const MAX_STEPS: usize = 4_194_304;
const MAX_BYTES: usize = 67_108_864;

const PREDEFINED_NAMES: [&str; 9] = [
    "__LINE__",
    "__FILE__",
    "__DATE__",
    "__TIME__",
    "__cplusplus",
    "__STDC__",
    "__STDC_HOSTED__",
    "__STDCPP_DEFAULT_NEW_ALIGNMENT__",
    "__STDCPP_THREADS__",
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
}

struct Definition {
    identifier: usize,
    replacement: Vec<RawToken>,
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
        budget: Budget::default(),
        output: PreparedSource::default(),
    };
    while let Some(token) = preprocessor.lexer.next(&mut preprocessor.budget)? {
        if token.kind == Kind::Newline {
            continue;
        }
        if token.line_start && token.spelling == "#" {
            preprocessor.directive(token.span)?;
        } else if preprocessor.active() {
            expand(
                &token,
                &preprocessor.definitions,
                &mut preprocessor.budget,
                &mut preprocessor.output,
            )?;
        }
    }
    if let Some(conditional) = preprocessor.conditionals.last() {
        return Err(error(
            "unterminated CUDA preprocessing conditional",
            conditional.span,
        ));
    }
    Ok(Some(preprocessor.output))
}

impl Preprocessor<'_> {
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
                    return Err(error(
                        "CUDA preprocessing #if expressions are unsupported",
                        directive.span,
                    ));
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
                    .last_mut()
                    .ok_or_else(|| error("unmatched CUDA preprocessing alternative", span))?;
                if conditional.seen_else {
                    return Err(error("CUDA preprocessing alternative after #else", span));
                }
                if conditional.parent_active && !conditional.branch_taken {
                    return Err(error(
                        "CUDA preprocessing #elif evaluation is unsupported",
                        directive.span,
                    ));
                }
                conditional.active = false;
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
            &[ref operand] if operand.kind == Kind::Word => Ok(operand),
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
        self.budget.visit(name.spelling.len(), name.span)?;
        check_predefined(&name.spelling, name.span)?;
        if let Some(first) = replacement.first() {
            if !first.separated {
                return Err(error(
                    if first.spelling == "(" {
                        "CUDA function-like macros are unsupported"
                    } else {
                        "expected whitespace before CUDA macro replacement"
                    },
                    first.span,
                ));
            }
        }
        if let Some(token) = replacement.iter().find(|token| token.spelling == "##") {
            return Err(error(
                "CUDA preprocessing token pasting is unsupported",
                token.span,
            ));
        }
        for token in replacement {
            self.budget.visit(token.spelling.len(), token.span)?;
        }
        if let Some(previous) = self.definitions.get(name.spelling.as_str()) {
            let compatible = previous.replacement.len() == replacement.len()
                && previous
                    .replacement
                    .iter()
                    .zip(replacement)
                    .enumerate()
                    .all(|(index, (left, right))| {
                        left.kind == right.kind
                            && left.spelling == right.spelling
                            && (index == 0 || left.separated == right.separated)
                    });
            if !compatible {
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
            replacement.len(),
            MAX_TOKENS,
            name.span,
        )?;
        self.budget.own(name.spelling.len(), name.span)?;
        for token in replacement {
            self.budget.own(token.spelling.len(), token.span)?;
        }
        self.definitions.insert(
            name.spelling.clone(),
            Definition {
                identifier: self.next_identifier,
                replacement: replacement.to_vec(),
            },
        );
        self.next_identifier += 1;
        Ok(())
    }
}

struct Expansion<'a> {
    tokens: &'a [RawToken],
    next: usize,
    disabled: Option<usize>,
}

fn expand(
    token: &RawToken,
    definitions: &FastHashMap<String, Definition>,
    budget: &mut Budget,
    output: &mut PreparedSource,
) -> Result<(), ParseError> {
    let mut pending = Vec::new();
    pending.push(Expansion {
        tokens: core::slice::from_ref(token),
        next: 0,
        disabled: None,
    });
    while let Some(expansion) = pending.last_mut() {
        budget.step(token.span)?;
        let tokens = expansion.tokens;
        let next = expansion.next;
        expansion.next += 1;
        let Some(current) = tokens.get(next) else {
            pending.pop();
            continue;
        };
        budget.visit(current.spelling.len(), token.span)?;
        if current.kind == Kind::Word {
            check_predefined(&current.spelling, token.span)?;
            if let Some(definition) = definitions.get(current.spelling.as_str()) {
                if !pending
                    .iter()
                    .any(|frame| frame.disabled == Some(definition.identifier))
                {
                    if pending.len() > MAX_DEPTH {
                        return Err(error(
                            "CUDA macro expansion depth limit exceeded",
                            token.span,
                        ));
                    }
                    pending.push(Expansion {
                        tokens: &definition.replacement,
                        next: 0,
                        disabled: Some(definition.identifier),
                    });
                    continue;
                }
            }
        }
        if output.tokens.len() == MAX_TOKENS {
            return Err(error(
                "CUDA preprocessing output token limit exceeded",
                token.span,
            ));
        }
        let start = output.spelling.len();
        let end = start
            .checked_add(current.spelling.len())
            .filter(|&end| end <= MAX_BYTES)
            .ok_or_else(|| {
                error(
                    "CUDA preprocessing output spelling limit exceeded",
                    token.span,
                )
            })?;
        output.spelling.push_str(&current.spelling);
        output.tokens.push(PreparedToken {
            spelling: start..end,
            span: token.span,
        });
    }
    Ok(())
}
