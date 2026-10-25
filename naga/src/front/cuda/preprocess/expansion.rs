use alloc::{string::String, vec::Vec};

use crate::{FastHashMap, Span};

use super::{
    check_predefined, conditions, error,
    lex::{Kind, RawToken},
    Budget, Definition, ExpandedToken, ParseError, MAX_DEPTH,
};

#[derive(Clone, Copy, PartialEq)]
pub(super) enum Mode {
    Shader,
    Condition,
    Argument,
}

#[derive(Clone, Copy)]
struct Token<'a> {
    value: ExpandedToken<'a>,
    original: bool,
    unavailable: bool,
}

impl<'a> Token<'a> {
    const fn original(raw: &'a RawToken) -> Self {
        Self {
            value: ExpandedToken {
                kind: raw.kind,
                spelling: raw.spelling.as_str(),
                span: raw.span,
            },
            original: true,
            unavailable: false,
        }
    }
}

struct Frame<'a> {
    tokens: Vec<Token<'a>>,
    next: usize,
    disabled: Option<usize>,
    span: Span,
}

struct Context {
    inherited_disabled: Vec<usize>,
    pending_depth: usize,
    outer_span: Option<Span>,
    mode: Mode,
    directive: bool,
}

struct Expander<'a, 'budget> {
    definitions: &'a FastHashMap<String, Definition>,
    budget: &'budget mut Budget,
    frames: Vec<Frame<'a>>,
    inherited_disabled: Vec<usize>,
    pending_depth: usize,
    outer_span: Option<Span>,
    mode: Mode,
    directive: bool,
}

pub(super) fn expand<'a>(
    tokens: &'a [RawToken],
    definitions: &'a FastHashMap<String, Definition>,
    budget: &mut Budget,
    mode: Mode,
    directive: bool,
    emit: &mut impl FnMut(ExpandedToken<'a>) -> Result<(), ParseError>,
) -> Result<(), ParseError> {
    let mut input = Vec::new();
    for token in tokens {
        budget.push(&mut input, Token::original(token), token.span)?;
    }
    let mut expander = Expander::new(
        input,
        definitions,
        budget,
        Context {
            inherited_disabled: Vec::new(),
            pending_depth: 0,
            outer_span: None,
            mode,
            directive,
        },
    )?;
    expander.run(&mut |token| emit(token.value))
}

impl<'a, 'budget> Expander<'a, 'budget> {
    fn new(
        tokens: Vec<Token<'a>>,
        definitions: &'a FastHashMap<String, Definition>,
        budget: &'budget mut Budget,
        context: Context,
    ) -> Result<Self, ParseError> {
        let Context {
            inherited_disabled,
            pending_depth,
            outer_span,
            mode,
            directive,
        } = context;
        let span = outer_span.unwrap_or_else(|| {
            tokens
                .first()
                .map_or(Span::UNDEFINED, |token| token.value.span)
        });
        let mut frames = Vec::new();
        budget.push(
            &mut frames,
            Frame {
                tokens,
                next: 0,
                disabled: None,
                span,
            },
            span,
        )?;
        Ok(Self {
            definitions,
            budget,
            frames,
            inherited_disabled,
            pending_depth,
            outer_span,
            mode,
            directive,
        })
    }

    fn depth(&self) -> usize {
        self.pending_depth
            + self
                .frames
                .iter()
                .filter(|frame| frame.disabled.is_some())
                .count()
    }

    fn disabled(&mut self, identifier: usize, span: Span) -> Result<bool, ParseError> {
        for &disabled in &self.inherited_disabled {
            self.budget.step(span)?;
            if disabled == identifier {
                return Ok(true);
            }
        }
        for frame in &self.frames {
            self.budget.step(span)?;
            if frame.disabled == Some(identifier) {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn disabled_context(&mut self, span: Span) -> Result<Vec<usize>, ParseError> {
        let mut disabled = Vec::new();
        for &identifier in &self.inherited_disabled {
            self.budget.push(&mut disabled, identifier, span)?;
        }
        for frame in &self.frames {
            if let Some(identifier) = frame.disabled {
                self.budget.push(&mut disabled, identifier, span)?;
            }
        }
        Ok(disabled)
    }

    fn peek(&mut self) -> Result<Option<Token<'a>>, ParseError> {
        for frame in self.frames.iter().rev() {
            self.budget.step(frame.span)?;
            if let Some(token) = frame.tokens.get(frame.next) {
                return Ok(Some(*token));
            }
        }
        Ok(None)
    }

    fn next(&mut self) -> Result<Option<Token<'a>>, ParseError> {
        loop {
            let Some(frame) = self.frames.last_mut() else {
                return Ok(None);
            };
            self.budget.step(frame.span)?;
            let Some(mut token) = frame.tokens.get(frame.next).copied() else {
                self.frames.pop();
                continue;
            };
            frame.next += 1;
            if let Some(span) = self.outer_span {
                token.value.span = span;
            }
            self.budget
                .visit(token.value.spelling.len(), token.value.span)?;
            if token.value.kind == Kind::Word && !token.unavailable {
                // Preserve suppression before argument collection can pop this frame.
                if let Some(definition) = self.definitions.get(token.value.spelling) {
                    token.unavailable = self.disabled(definition.identifier, token.value.span)?;
                }
            }
            return Ok(Some(token));
        }
    }

    fn run(
        &mut self,
        emit: &mut dyn FnMut(Token<'a>) -> Result<(), ParseError>,
    ) -> Result<(), ParseError> {
        while let Some(mut token) = self.next()? {
            if token.value.kind == Kind::Word {
                if self.mode == Mode::Condition && token.value.spelling == "defined" {
                    if !token.original {
                        return Err(error(
                            "macro expansion producing defined is unsupported",
                            token.value.span,
                        ));
                    }
                    token = self.query(token)?;
                } else {
                    check_predefined(token.value.spelling, token.value.span)?;
                    if !token.unavailable {
                        self.budget
                            .visit(token.value.spelling.len(), token.value.span)?;
                        if let Some(definition) = self.definitions.get(token.value.spelling) {
                            let invoked = definition.parameters.is_none()
                                || self.peek()?.is_some_and(|next| next.value.spelling == "(");
                            if invoked {
                                self.invoke(token, definition)?;
                                continue;
                            }
                        }
                    }
                }
            }
            self.budget.emit(token.value.span)?;
            self.budget.step(token.value.span)?;
            self.budget
                .visit(token.value.spelling.len(), token.value.span)?;
            emit(token)?;
        }
        Ok(())
    }

    fn query(&mut self, operator: Token<'a>) -> Result<Token<'a>, ParseError> {
        let mut operand = self.next()?.ok_or_else(|| {
            error(
                "expected preprocessing macro name after defined",
                operator.value.span,
            )
        })?;
        let parenthesized = operand.value.spelling == "(";
        if parenthesized {
            operand = self.next()?.ok_or_else(|| {
                error(
                    "expected preprocessing macro name after defined",
                    operator.value.span,
                )
            })?;
        }
        if operand.value.kind != Kind::Word || conditions::is_alternative(operand.value.spelling) {
            return Err(error(
                "expected preprocessing macro name after defined",
                operand.value.span,
            ));
        }
        check_predefined(operand.value.spelling, operand.value.span)?;
        self.budget
            .visit(operand.value.spelling.len(), operand.value.span)?;
        let value = self.definitions.contains_key(operand.value.spelling);
        let mut span = operator.value.span;
        span.subsume(operand.value.span);
        if parenthesized {
            let closing = self
                .next()?
                .filter(|token| token.value.spelling == ")")
                .ok_or_else(|| {
                    error(
                        "expected closing parenthesis after defined",
                        operator.value.span,
                    )
                })?;
            span.subsume(closing.value.span);
        }
        Ok(Token {
            value: ExpandedToken {
                kind: Kind::Number,
                spelling: if value { "1" } else { "0" },
                span,
            },
            original: false,
            unavailable: false,
        })
    }

    fn arguments(
        &mut self,
        invocation: Span,
        parameter_count: usize,
    ) -> Result<(Vec<Vec<Token<'a>>>, Span), ParseError> {
        let opening = self
            .next()?
            .ok_or_else(|| error("expected CUDA macro opening parenthesis", invocation))?;
        let mut span = invocation;
        span.subsume(opening.value.span);
        let mut arguments = Vec::new();
        let mut argument = Vec::new();
        let mut parentheses = 0;
        loop {
            let token = self.next()?.ok_or_else(|| {
                error(
                    if self.directive {
                        "directive inside CUDA macro invocation is unsupported"
                    } else {
                        "unterminated CUDA macro invocation"
                    },
                    span,
                )
            })?;
            span.subsume(token.value.span);
            match token.value.spelling {
                "(" => {
                    if parentheses == MAX_DEPTH {
                        return Err(error(
                            "CUDA macro argument parenthesis limit exceeded",
                            span,
                        ));
                    }
                    parentheses += 1;
                }
                ")" if parentheses == 0 => {
                    self.budget.push(&mut arguments, argument, span)?;
                    if parameter_count == 0 && arguments.len() == 1 && arguments[0].is_empty() {
                        arguments.clear();
                    }
                    if arguments.len() != parameter_count {
                        return Err(error("incorrect CUDA macro argument count", span));
                    }
                    return Ok((arguments, span));
                }
                ")" => parentheses -= 1,
                "," if parentheses == 0 => {
                    self.budget
                        .push(&mut arguments, core::mem::take(&mut argument), span)?;
                    continue;
                }
                _ => {}
            }
            self.budget.push(&mut argument, token, span)?;
        }
    }

    fn invoke(&mut self, token: Token<'a>, definition: &'a Definition) -> Result<(), ParseError> {
        if self.depth() == MAX_DEPTH {
            return Err(error(
                "CUDA macro expansion depth limit exceeded",
                token.value.span,
            ));
        }
        let mut span = token.value.span;
        let mut expanded_arguments: Vec<Option<Vec<Token<'a>>>> = Vec::new();
        if let Some(ref parameters) = definition.parameters {
            let (mut arguments, invocation_span) = self.arguments(span, parameters.len())?;
            span = invocation_span;
            for _ in parameters {
                self.budget.push(&mut expanded_arguments, None, span)?;
            }
            for &slot in &definition.parameter_slots {
                self.budget.step(span)?;
                if let Some(index) = slot {
                    if expanded_arguments[index].is_none() {
                        let disabled = self.disabled_context(span)?;
                        let depth = self.depth() + 1;
                        let argument = core::mem::take(&mut arguments[index]);
                        let mut expanded = Vec::new();
                        let mut prescan = Expander::new(
                            argument,
                            self.definitions,
                            self.budget,
                            Context {
                                mode: Mode::Argument,
                                pending_depth: depth,
                                inherited_disabled: disabled,
                                outer_span: Some(span),
                                directive: false,
                            },
                        )?;
                        prescan.run(&mut |token| {
                            expanded.push(token);
                            Ok(())
                        })?;
                        expanded_arguments[index] = Some(expanded);
                    }
                }
            }
        }
        let mut replacement = Vec::new();
        for (raw, &slot) in definition
            .replacement
            .iter()
            .zip(&definition.parameter_slots)
        {
            if let Some(index) = slot {
                if let Some(ref argument) = expanded_arguments[index] {
                    for &argument_token in argument {
                        let mut substituted = argument_token;
                        substituted.original = false;
                        substituted.value.span = span;
                        self.budget.push(&mut replacement, substituted, span)?;
                    }
                }
            } else {
                let mut replacement_token = Token::original(raw);
                replacement_token.original = false;
                replacement_token.value.span = span;
                self.budget
                    .push(&mut replacement, replacement_token, span)?;
            }
        }
        self.budget.push(
            &mut self.frames,
            Frame {
                tokens: replacement,
                next: 0,
                disabled: Some(definition.identifier),
                span,
            },
            span,
        )?;
        Ok(())
    }
}
