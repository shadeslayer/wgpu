use super::*;

impl Parser<'_> {
    fn loop_condition(&mut self) -> Result<(), ParseError> {
        let value = self.expression(0)?;
        let condition = self.truthy(value)?;
        let mut reject = crate::Block::new();
        reject.push(Statement::Break, condition.span);
        self.function.body.push(
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
        let parent = core::mem::take(&mut self.function.body);
        self.loop_condition()?;
        self.expect_symbol(')')?;
        self.loop_depth += 1;
        let result = self.scoped_statement();
        self.loop_depth -= 1;
        let (body, _) = result?;
        self.function.body.push(Statement::Block(body), start);
        let body = core::mem::replace(&mut self.function.body, parent);
        self.function.body.push(
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
        let parent = core::mem::take(&mut self.function.body);
        if !self.take_symbol(';') {
            if self.starts_declaration() {
                self.local_declaration()?;
            } else {
                self.expression(0)?;
                self.expect_symbol(';')?;
            }
        }
        let mut init = core::mem::take(&mut self.function.body);
        if !self.take_symbol(';') {
            self.loop_condition()?;
            self.expect_symbol(';')?;
        }
        let mut body = core::mem::take(&mut self.function.body);
        if !self.take_symbol(')') {
            self.expression(0)?;
            self.expect_symbol(')')?;
        }
        let continuing = core::mem::take(&mut self.function.body);
        self.loop_depth += 1;
        let result = if self.take_symbol('{') {
            self.block_contents()
        } else {
            self.statement()
        };
        self.loop_depth -= 1;
        result?;
        body.append(&mut self.function.body);
        init.push(
            Statement::Loop {
                body,
                continuing,
                break_if: None,
            },
            start,
        );
        self.function.body = parent;
        self.symbol_table.pop_scope();
        self.function.body.push(Statement::Block(init), start);
        Ok(())
    }

    pub(super) fn do_loop(&mut self, start: Span) -> Result<(), ParseError> {
        let parent = core::mem::take(&mut self.function.body);
        self.loop_depth += 1;
        let result = self.scoped_statement();
        self.loop_depth -= 1;
        let (body, _) = result?;
        self.expect_ident("while")?;
        self.expect_symbol('(')?;
        let value = self.expression(0)?;
        let condition = self.truthy(value)?;
        let break_if = self.append(
            Expression::Unary {
                op: UnaryOperator::LogicalNot,
                expr: condition.handle,
            },
            condition.span,
        )?;
        self.expect_symbol(')')?;
        let end = self.expect_symbol(';')?;
        let continuing = core::mem::replace(&mut self.function.body, parent);
        self.function.body.push(
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
