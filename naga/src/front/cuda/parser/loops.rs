use super::*;

impl Parser<'_> {
    fn loop_condition(&mut self) -> Result<(), ParseError> {
        let value = self.expression(0)?;
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
        self.expect_symbol('(')?;
        let ((), body) = self.capture_block(|parser| {
            parser.loop_condition()?;
            parser.expect_symbol(')')?;
            parser.loop_depth += 1;
            let result = parser.scoped_statement();
            parser.loop_depth -= 1;
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
        self.expect_symbol('(')?;
        self.symbol_table.push_scope();
        let result = self.for_loop_body(start);
        self.symbol_table.pop_scope();
        self.push_statement(Statement::Block(result?), start);
        Ok(())
    }

    fn for_loop_body(&mut self, start: Span) -> Result<crate::Block, ParseError> {
        let ((), mut init) = self.capture_block(|parser| {
            if !parser.take_symbol(';')? {
                if parser.starts_declaration()? {
                    parser.local_declaration()?;
                } else {
                    parser.expression(0)?;
                    parser.expect_symbol(';')?;
                }
            }
            Ok(())
        })?;
        let ((), mut body) = self.capture_block(|parser| {
            if !parser.take_symbol(';')? {
                parser.loop_condition()?;
                parser.expect_symbol(';')?;
            }
            Ok(())
        })?;
        let ((), continuing) = self.capture_block(|parser| {
            if !parser.take_symbol(')')? {
                parser.expression(0)?;
                parser.expect_symbol(')')?;
            }
            Ok(())
        })?;
        self.loop_depth += 1;
        let result = self.capture_block(|parser| {
            if parser.take_symbol('{')? {
                parser.block_contents()
            } else {
                parser.statement()
            }
        });
        self.loop_depth -= 1;
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
        self.loop_depth += 1;
        let result = self.scoped_statement();
        self.loop_depth -= 1;
        let (body, _) = result?;
        self.expect_ident("while")?;
        self.expect_symbol('(')?;
        let ((break_if, end), continuing) = self.capture_block(|parser| {
            let value = parser.expression(0)?;
            let condition = parser.truthy(value)?;
            let break_if = parser.append(
                Expression::Unary {
                    op: UnaryOperator::LogicalNot,
                    expr: condition.handle,
                },
                condition.span,
            )?;
            parser.expect_symbol(')')?;
            let end = parser.expect_symbol(';')?;
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
