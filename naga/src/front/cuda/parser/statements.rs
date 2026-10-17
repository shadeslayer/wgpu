use super::*;

use core::num::NonZeroU32;

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
        self.function.symbol_table.push_scope();
        let result = self.capture_block(|parser| {
            if compound {
                parser.block_contents()
            } else {
                parser.statement()
            }
        });
        self.function.symbol_table.pop_scope();
        result.map(|(terminated, body)| (body, terminated))
    }

    pub(super) fn statement(&mut self) -> Result<bool, ParseError> {
        self.function.statement_depth += 1;
        if self.function.statement_depth > 64 {
            return self.fail("CUDA statement nesting limit exceeded");
        }
        let result = self.statement_inner();
        self.function.statement_depth -= 1;
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
            let value = if let Some(ref result) = self.function.ir.result {
                let TypeInner::Scalar(scalar) = self.module.types[result.ty].inner else {
                    unreachable!()
                };
                let value = self.expression(0)?;
                Some(self.convert_value(value, scalar)?.handle)
            } else {
                if self.lexer.peek()?.kind != TokenKind::Symbol(';') {
                    if let ExpressionResult::Scalar(value) = self.expression_result(0)? {
                        return Err(ParseError {
                            message: String::from("cannot return a value from a void function"),
                            span: value.span(),
                        });
                    }
                }
                None
            };
            let end = self.expect_symbol(';')?;
            self.push_statement(Statement::Return { value }, start.until(&end));
            return Ok(true);
        }
        if self.take_ident("__syncthreads")? {
            if self.function.kind == FunctionKind::Device {
                return Err(ParseError {
                    message: String::from("barriers are unsupported in device helpers"),
                    span: start,
                });
            }
            self.expect_symbol('(')?;
            self.expect_symbol(')')?;
            let end = self.expect_symbol(';')?;
            self.push_statement(
                Statement::ControlBarrier(crate::Barrier::WORK_GROUP | crate::Barrier::STORAGE),
                start.until(&end),
            );
            return Ok(false);
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
            if self.function.loop_depth == 0 {
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
        self.discard_expression()?;
        self.expect_symbol(';')?;
        Ok(false)
    }

    pub(super) fn starts_declaration(&mut self) -> Result<bool, ParseError> {
        if !matches!(
            self.lexer.peek()?.kind,
            TokenKind::Ident("__shared__" | "const" | "bool" | "float" | "int" | "unsigned")
        ) {
            return Ok(false);
        }
        let mut lookahead = self.lexer.clone();
        lookahead.next()?;
        let next = lookahead.peek()?;
        if next.kind != TokenKind::Symbol('(') {
            return Ok(true);
        }
        if declarations::parenthesized_declaration(lookahead)? {
            return Err(ParseError {
                message: String::from("parenthesized declarators are unsupported"),
                span: next.span,
            });
        }
        Ok(false)
    }

    pub(super) fn local_declaration(&mut self) -> Result<(), ParseError> {
        if self.take_ident("__shared__")? {
            return self.shared_declaration();
        }
        let is_const = self.take_ident("const")?;
        let scalar = self.scalar_type()?;
        let is_const = self.const_suffix(is_const)?;
        let (name, span) = self.identifier()?;
        self.check_name(name, span)?;
        let ty = self.scalar_handle(scalar, span);
        let local = self.function.ir.local_variables.append(
            LocalVariable {
                name: Some(String::from(name)),
                ty,
                init: None,
            },
            span,
        );
        let pointer = self.append(Expression::LocalVariable(local), span)?;
        self.insert_symbol(
            name,
            Symbol::Scalar {
                pointer,
                scalar,
                writable: !is_const,
            },
            span,
        )?;
        self.expect_symbol('=')?;
        let value = self.expression(0)?;
        let value = self.convert_value(value, scalar)?;
        let end = self.expect_symbol(';')?;
        self.push_statement(
            Statement::Store {
                pointer,
                value: value.handle,
            },
            span.until(&end),
        );
        Ok(())
    }

    fn shared_declaration(&mut self) -> Result<(), ParseError> {
        if self.function.kind == FunctionKind::Device {
            return self.fail("shared storage is unsupported in device helpers");
        }
        let type_span = self.lexer.peek()?.span;
        let scalar = self.scalar_type()?;
        if scalar == Scalar::BOOL {
            return Err(ParseError {
                message: String::from("shared storage requires a 32-bit numeric type"),
                span: type_span,
            });
        }
        let (name, span) = self.identifier()?;
        self.check_name(name, span)?;
        let base = self.scalar_handle(scalar, span);
        let is_array = self.take_symbol('[')?;
        let ty = if is_array {
            let (size, extent_span) = self.shared_array_size()?;
            self.expect_symbol(']')?;
            let inner = TypeInner::Array {
                base,
                size: ArraySize::Constant(size),
                stride: 4,
            };
            if inner.try_size(self.module.to_ctx()).is_none() {
                return Err(ParseError {
                    message: String::from("shared array exceeds Naga's type size limit"),
                    span: extent_span,
                });
            }
            self.module.types.insert(Type { name: None, inner }, span)
        } else {
            base
        };
        if self.lexer.peek()?.kind == TokenKind::Symbol('[') {
            return self.fail("multidimensional shared arrays are unsupported");
        }
        if self.lexer.peek()?.kind == TokenKind::Symbol('=') {
            return self.fail("shared variables cannot have initializers");
        }
        self.expect_symbol(';')?;
        let global = self.module.global_variables.append(
            GlobalVariable {
                name: Some(String::from(name)),
                space: AddressSpace::WorkGroup,
                binding: None,
                ty,
                init: None,
                memory_decorations: Default::default(),
            },
            span,
        );
        let pointer = self.append(Expression::GlobalVariable(global), span)?;
        let symbol = if is_array {
            Symbol::Array {
                pointer,
                scalar,
                writable: true,
            }
        } else {
            Symbol::Scalar {
                pointer,
                scalar,
                writable: true,
            }
        };
        self.insert_symbol(name, symbol, span)
    }

    fn shared_array_size(&mut self) -> Result<(NonZeroU32, Span), ParseError> {
        let token = self.lexer.next()?;
        let invalid = || ParseError {
            message: String::from("shared array extent must be a positive decimal integer literal"),
            span: token.span,
        };
        let TokenKind::Number(text) = token.kind else {
            return Err(invalid());
        };
        let digits = text.strip_suffix(['u', 'U']).unwrap_or(text);
        if digits.is_empty()
            || !digits.bytes().all(|byte| byte.is_ascii_digit())
            || (digits.len() > 1 && digits.starts_with('0'))
        {
            return Err(invalid());
        }
        let size = digits
            .parse::<u32>()
            .ok()
            .and_then(NonZeroU32::new)
            .ok_or_else(invalid)?;
        Ok((size, token.span))
    }
}
