use super::*;

impl<'a> Parser<'a> {
    pub(super) fn block_contents(&mut self) -> Result<bool, ParseError> {
        let mut terminated = false;
        while !self.take_symbol('}')? {
            if self.lexer.peek()?.kind == TokenKind::End {
                return self.fail("expected `}`");
            }
            if terminated {
                self.capture_block(Self::statement)?;
            } else {
                terminated = self.statement()?;
            }
        }
        Ok(terminated)
    }

    pub(super) fn scoped_statement(&mut self) -> Result<(crate::Block, bool), ParseError> {
        let compound = self.take_symbol('{')?;
        self.symbol_table.push_scope();
        let result = self.capture_block(|parser| {
            if compound {
                parser.block_contents()
            } else {
                parser.statement()
            }
        });
        self.symbol_table.pop_scope();
        result.map(|(terminated, body)| (body, terminated))
    }

    pub(super) fn statement(&mut self) -> Result<bool, ParseError> {
        self.statement_depth += 1;
        if self.statement_depth > 64 {
            return self.fail("CUDA statement nesting limit exceeded");
        }
        let result = self.statement_inner();
        self.statement_depth -= 1;
        result
    }

    fn statement_inner(&mut self) -> Result<bool, ParseError> {
        let token = self.lexer.peek()?;
        let start = token.span;
        if self.take_symbol(';')? {
            return Ok(false);
        }
        if self.lexer.peek()?.kind == TokenKind::Symbol('{') {
            let (body, terminated) = self.scoped_statement()?;
            self.push_statement(Statement::Block(body), start);
            return Ok(terminated);
        }
        if self.take_ident("if")? {
            self.expect_symbol('(')?;
            let value = self.expression(0)?;
            let condition = self.truthy(value)?;
            self.expect_symbol(')')?;
            let (accept, accept_terminated) = self.scoped_statement()?;
            let (reject, reject_terminated) = if self.take_ident("else")? {
                self.scoped_statement()?
            } else {
                (crate::Block::new(), false)
            };
            self.push_statement(
                Statement::If {
                    condition: condition.handle,
                    accept,
                    reject,
                },
                start,
            );
            return Ok(accept_terminated && reject_terminated);
        }
        if self.take_ident("return")? {
            let end = self.expect_symbol(';')?;
            self.push_statement(Statement::Return { value: None }, start.until(&end));
            return Ok(true);
        }
        if self.take_ident("for")? {
            self.for_loop(start)?;
            return Ok(false);
        }
        if self.take_ident("while")? {
            self.while_loop(start)?;
            return Ok(false);
        }
        if self.take_ident("do")? {
            self.do_loop(start)?;
            return Ok(false);
        }
        if matches!(token.kind, TokenKind::Ident("break" | "continue")) {
            self.lexer.next()?;
            if self.loop_depth == 0 {
                return Err(ParseError {
                    message: String::from("jump requires an enclosing loop"),
                    span: start,
                });
            }
            let statement = if token.kind == TokenKind::Ident("break") {
                Statement::Break
            } else {
                Statement::Continue
            };
            let end = self.expect_symbol(';')?;
            self.push_statement(statement, start.until(&end));
            return Ok(true);
        }
        if self.starts_declaration()? {
            self.local_declaration()?;
            return Ok(false);
        }
        self.expression(0)?;
        self.expect_symbol(';')?;
        Ok(false)
    }

    pub(super) fn starts_declaration(&mut self) -> Result<bool, ParseError> {
        if !matches!(
            self.lexer.peek()?.kind,
            TokenKind::Ident("const" | "bool" | "float" | "int" | "unsigned")
        ) {
            return Ok(false);
        }
        let mut lookahead = self.lexer.clone();
        lookahead.next()?;
        Ok(lookahead.peek()?.kind != TokenKind::Symbol('('))
    }

    pub(super) fn local_declaration(&mut self) -> Result<(), ParseError> {
        let is_const = self.take_ident("const")?;
        let scalar = self.scalar_type()?;
        let is_const = self.const_suffix(is_const)?;
        let (name, span) = self.identifier()?;
        self.check_name(name, span)?;
        self.expect_symbol('=')?;
        let value = self.expression(0)?;
        let value = self.convert_value(value, scalar)?;
        let end = self.expect_symbol(';')?;
        let ty = self.scalar_handle(scalar, span);
        let local = self.function.local_variables.append(
            LocalVariable {
                name: Some(String::from(name)),
                ty,
                init: None,
            },
            span,
        );
        let pointer = self.append(Expression::LocalVariable(local), span)?;
        self.push_statement(
            Statement::Store {
                pointer,
                value: value.handle,
            },
            span.until(&end),
        );
        self.insert_symbol(
            name,
            Symbol::Local {
                pointer,
                scalar,
                writable: !is_const,
            },
            span,
        )?;
        Ok(())
    }
}
