use super::*;

impl<'a> Parser<'a> {
    pub(super) fn block_contents(&mut self) -> Result<bool, ParseError> {
        let mut terminated = false;
        while !self.take_symbol('}') {
            if self.current().kind == TokenKind::End {
                return Err(self.error("expected `}`"));
            }
            if terminated {
                let reachable = core::mem::take(&mut self.function.body);
                let result = self.statement();
                self.function.body = reachable;
                result?;
            } else {
                terminated = self.statement()?;
            }
        }
        Ok(terminated)
    }

    fn scoped_statement(&mut self) -> Result<(crate::Block, bool), ParseError> {
        let compound = self.take_symbol('{');
        self.scopes.push(FastHashMap::default());
        let parent = core::mem::take(&mut self.function.body);
        let result = if compound {
            self.block_contents()
        } else {
            self.statement()
        };
        let body = core::mem::replace(&mut self.function.body, parent);
        self.scopes.pop();
        result.map(|terminated| (body, terminated))
    }

    fn statement(&mut self) -> Result<bool, ParseError> {
        self.statement_depth += 1;
        if self.statement_depth > 64 {
            return Err(self.error("CUDA statement nesting limit exceeded"));
        }
        let result = self.statement_inner();
        self.statement_depth -= 1;
        result
    }

    fn statement_inner(&mut self) -> Result<bool, ParseError> {
        let start = self.current().span;
        if self.take_symbol(';') {
            return Ok(false);
        }
        if self.current().kind == TokenKind::Symbol('{') {
            let (body, terminated) = self.scoped_statement()?;
            self.function.body.push(Statement::Block(body), start);
            return Ok(terminated);
        }
        if self.take_ident("if") {
            self.expect_symbol('(')?;
            let value = self.expression(0)?;
            let condition = self.truthy(value)?;
            self.expect_symbol(')')?;
            let (accept, accept_terminated) = self.scoped_statement()?;
            let (reject, reject_terminated) = if self.take_ident("else") {
                self.scoped_statement()?
            } else {
                (crate::Block::new(), false)
            };
            self.function.body.push(
                Statement::If {
                    condition: condition.handle,
                    accept,
                    reject,
                },
                start,
            );
            return Ok(accept_terminated && reject_terminated);
        }
        if self.take_ident("return") {
            let end = self.expect_symbol(';')?;
            self.function
                .body
                .push(Statement::Return { value: None }, start.until(&end));
            return Ok(true);
        }
        if matches!(
            self.current().kind,
            TokenKind::Ident("const" | "bool" | "float" | "int" | "unsigned")
        ) {
            self.local_declaration()?;
            return Ok(false);
        }
        let (name, span) = self.identifier()?;
        let symbol = self.lookup_symbol(name).ok_or_else(|| ParseError {
            message: format!("unknown variable `{name}`"),
            span,
        })?;
        let (pointer, scalar, writable) = match symbol {
            Symbol::Buffer {
                pointer,
                scalar,
                writable,
            } => {
                let pointer = self.buffer_element(pointer, span)?;
                (pointer, scalar, writable)
            }
            Symbol::Local {
                pointer,
                scalar,
                writable,
            } => (pointer, scalar, writable),
        };
        if !writable {
            return Err(ParseError {
                message: format!("cannot write to const variable `{name}`"),
                span,
            });
        }
        self.expect_symbol('=')?;
        let value = self.expression(0)?;
        let value = self.convert_value(value, scalar)?;
        let end = self.expect_symbol(';')?;
        self.function.body.push(
            Statement::Store {
                pointer,
                value: value.handle,
            },
            span.until(&end),
        );
        Ok(false)
    }

    fn local_declaration(&mut self) -> Result<(), ParseError> {
        let is_const = self.take_ident("const");
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
        self.function.body.push(
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
        );
        Ok(())
    }
}
