use super::*;

impl Parser<'_> {
    fn loop_condition(&mut self) -> Result<(), ParseError> {
        let value = self.comma_expression()?;
        let condition = self.truthy(value)?;
        let mut reject = crate::Block::new();
        reject.push(Statement::Break, condition.span);
        self.push_statement(
            Statement::If {
                condition: condition.handle,
                accept: crate::Block::new(),
                reject,
            },
            condition.span,
        );
        Ok(())
    }

    pub(super) fn while_loop(&mut self, start: Span) -> Result<(), ParseError> {
        self.lexer.expect_span(Token::Paren('('))?;
        let ((), body) = self.capture_block(|parser| {
            parser.loop_condition()?;
            parser.lexer.expect_span(Token::Paren(')'))?;
            parser.function.loop_depth += 1;
            let result = parser.scoped_statement();
            parser.function.loop_depth -= 1;
            let (body, _) = result?;
            parser.push_statement(Statement::Block(body), start);
            Ok(())
        })?;
        self.push_statement(
            Statement::Loop {
                body,
                continuing: crate::Block::new(),
                break_if: None,
            },
            start,
        );
        Ok(())
    }

    pub(super) fn for_loop(&mut self, start: Span) -> Result<(), ParseError> {
        self.lexer.expect_span(Token::Paren('('))?;
        let initialization_start = self.function.push_scope();
        let result = self.for_loop_body(start);
        self.function.pop_scope(initialization_start);
        self.push_statement(Statement::Block(result?), start);
        Ok(())
    }

    fn for_loop_body(&mut self, start: Span) -> Result<crate::Block, ParseError> {
        let ((), mut init) = self.capture_block(|parser| {
            if !parser.lexer.next_if(Token::Separator(';'))? {
                if parser.starts_declaration()? {
                    parser.local_declaration()?;
                } else {
                    parser.discard_expression()?;
                    parser.lexer.expect_span(Token::Separator(';'))?;
                }
            }
            Ok(())
        })?;
        let ((), mut body) = self.capture_block(|parser| {
            if !parser.lexer.next_if(Token::Separator(';'))? {
                parser.loop_condition()?;
                parser.lexer.expect_span(Token::Separator(';'))?;
            }
            Ok(())
        })?;
        let ((), continuing) = self.capture_block(|parser| {
            if !parser.lexer.next_if(Token::Paren(')'))? {
                parser.discard_expression()?;
                parser.lexer.expect_span(Token::Paren(')'))?;
            }
            Ok(())
        })?;
        self.function.loop_depth += 1;
        let result = self.capture_block(|parser| {
            if parser.lexer.next_if(Token::Paren('{'))? {
                parser.block_contents()
            } else {
                parser.statement()
            }
        });
        self.function.loop_depth -= 1;
        let (_, statements) = result?;
        body.extend_block(statements);
        init.push(
            Statement::Loop {
                body,
                continuing,
                break_if: None,
            },
            start,
        );
        Ok(init)
    }

    pub(super) fn do_loop(&mut self, start: Span) -> Result<(), ParseError> {
        self.function.loop_depth += 1;
        let result = self.scoped_statement();
        self.function.loop_depth -= 1;
        let (body, _) = result?;
        self.expect_ident("while")?;
        self.lexer.expect_span(Token::Paren('('))?;
        let ((break_if, end), continuing) = self.capture_block(|parser| {
            let value = parser.comma_expression()?;
            let condition = parser.truthy(value)?;
            let break_if = parser.append(
                Expression::Unary {
                    op: UnaryOperator::LogicalNot,
                    expr: condition.handle,
                },
                condition.span,
            )?;
            parser.lexer.expect_span(Token::Paren(')'))?;
            let end = parser.lexer.expect_span(Token::Separator(';'))?;
            Ok((break_if, end))
        })?;
        self.push_statement(
            Statement::Loop {
                body,
                continuing,
                break_if: Some(break_if),
            },
            start.until(&end),
        );
        Ok(())
    }
}
