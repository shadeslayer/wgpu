use super::*;

struct BlockFlow {
    reaches_end: bool,
    breaks: bool,
}

fn block_flow(block: &crate::Block) -> BlockFlow {
    let mut flow = BlockFlow {
        reaches_end: true,
        breaks: false,
    };
    for statement in block {
        if !flow.reaches_end {
            break;
        }
        let statement_flow = match *statement {
            Statement::Break => BlockFlow {
                reaches_end: false,
                breaks: true,
            },
            Statement::Return { .. } | Statement::Continue => BlockFlow {
                reaches_end: false,
                breaks: false,
            },
            Statement::Block(ref body) => block_flow(body),
            Statement::If {
                ref accept,
                ref reject,
                ..
            } => {
                let accept = block_flow(accept);
                let reject = block_flow(reject);
                BlockFlow {
                    reaches_end: accept.reaches_end || reject.reaches_end,
                    breaks: accept.breaks || reject.breaks,
                }
            }
            Statement::Switch { ref cases, .. } => BlockFlow {
                reaches_end: switch_reaches_end(cases),
                breaks: false,
            },
            _ => BlockFlow {
                reaches_end: true,
                breaks: false,
            },
        };
        flow.reaches_end = statement_flow.reaches_end;
        flow.breaks |= statement_flow.breaks;
    }
    flow
}

fn switch_reaches_end(cases: &[crate::SwitchCase]) -> bool {
    let mut following_reaches_end = true;
    let mut reaches_end = false;
    for case in cases.iter().rev() {
        let body = block_flow(&case.body);
        let case_reaches_end =
            body.breaks || (body.reaches_end && (!case.fall_through || following_reaches_end));
        reaches_end |= case_reaches_end;
        following_reaches_end = case_reaches_end;
    }
    reaches_end
}

impl Parser<'_> {
    pub(super) fn switch_statement(&mut self, start: Span) -> Result<bool, ParseError> {
        self.lexer.expect_span(Token::Paren('('))?;
        let selector = self.comma_expression()?;
        let selector = self.integral_operand(selector)?;
        let scalar = self.require_scalar_type(&selector.ty, selector.span)?;
        self.lexer.expect_span(Token::Paren(')'))?;

        let initialization_start = self.function.push_scope();
        self.function.switch_depth += 1;
        let result = self.switch_body(scalar, initialization_start);
        self.function.switch_depth -= 1;
        self.function.pop_scope(initialization_start);
        let cases = result?;
        let terminated = !switch_reaches_end(&cases);
        self.push_statement(
            Statement::Switch {
                selector: selector.handle,
                cases,
            },
            start,
        );
        Ok(terminated)
    }

    fn switch_body(
        &mut self,
        selector: Scalar,
        initialization_start: usize,
    ) -> Result<Vec<crate::SwitchCase>, ParseError> {
        let compound = self.lexer.next_if(Token::Paren('{'))?;
        let mut values = crate::FastHashSet::default();
        let mut cases = Vec::new();
        let mut current_case: Option<crate::SwitchCase> = None;
        let mut terminated = false;
        loop {
            if compound && self.lexer.next_if(Token::Paren('}'))? {
                break;
            }
            if compound && self.lexer.peek()?.0 == Token::End {
                return self.fail("expected `}`");
            }
            let (token, span) = self.lexer.peek()?;
            if matches!(token, Token::Word("case" | "default")) {
                self.lexer.next()?;
                if let Some(&declaration) = self
                    .function
                    .initialized_declarations
                    .get(initialization_start)
                {
                    return Err(ParseError {
                        message: String::from("case label bypasses a local initializer"),
                        span: declaration,
                    });
                }
                let value = if token == Token::Word("case") {
                    self.switch_case_value(selector)?
                } else {
                    crate::SwitchValue::Default
                };
                self.lexer.expect_span(Token::Separator(':'))?;
                if !values.insert(value) {
                    return Err(ParseError {
                        message: String::from("duplicate switch case or default label"),
                        span,
                    });
                }
                if let Some(mut case) = current_case.take() {
                    case.fall_through = !terminated;
                    cases.push(case);
                }
                current_case = Some(crate::SwitchCase {
                    value,
                    body: crate::Block::new(),
                    fall_through: false,
                });
                terminated = false;
                continue;
            }

            let (statement_terminated, body) = self.capture_block(Self::statement)?;
            if let Some(case) = current_case.as_mut() {
                if !terminated {
                    case.body.extend_block(body);
                    terminated = statement_terminated;
                }
            }
            if !compound {
                break;
            }
        }
        if let Some(mut case) = current_case {
            case.fall_through = false;
            cases.push(case);
        }
        if !values.contains(&crate::SwitchValue::Default) {
            cases.push(crate::SwitchCase {
                value: crate::SwitchValue::Default,
                body: crate::Block::new(),
                fall_through: false,
            });
        }
        Ok(cases)
    }

    fn switch_case_value(&mut self, selector: Scalar) -> Result<crate::SwitchValue, ParseError> {
        let (value, _) = self.capture_block(|parser| {
            let expression = parser.expression(0)?;
            parser.rvalue(expression)
        })?;
        let literal = Self::required_constant(&value)?;
        let converted = match (selector, literal) {
            (Scalar::I32, Literal::Bool(boolean)) => {
                Some(crate::SwitchValue::I32(i32::from(boolean)))
            }
            (Scalar::I32, Literal::I32(integer)) => Some(crate::SwitchValue::I32(integer)),
            (Scalar::I32, Literal::U32(integer)) => {
                i32::try_from(integer).ok().map(crate::SwitchValue::I32)
            }
            (Scalar::U32, Literal::Bool(boolean)) => {
                Some(crate::SwitchValue::U32(u32::from(boolean)))
            }
            (Scalar::U32, Literal::I32(integer)) => {
                u32::try_from(integer).ok().map(crate::SwitchValue::U32)
            }
            (Scalar::U32, Literal::U32(integer)) => Some(crate::SwitchValue::U32(integer)),
            _ => None,
        };
        converted.ok_or_else(|| ParseError {
            message: String::from(
                "case label must be a representable integral constant expression",
            ),
            span: value.span,
        })
    }
}
