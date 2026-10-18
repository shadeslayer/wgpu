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
            let value = self.comma_expression()?;
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
                let ty = TypeResolution::Handle(result.ty);
                let value = self.comma_expression()?;
                Some(self.convert_value(value, &ty)?.handle)
            } else {
                if self.lexer.peek()?.kind != TokenKind::Symbol(';') {
                    if let ExpressionResult::Value(value) = self.comma_expression_result()? {
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
        if self.starts_alias()? || self.lexer.peek()?.kind == TokenKind::Ident("__shared__") {
            return Ok(true);
        }
        if !self.starts_type()? {
            return Ok(false);
        }
        let mut lookahead = self.lexer.clone();
        lookahead.next()?;
        let next = lookahead.peek()?;
        if next.kind != TokenKind::Symbol('(') {
            return Ok(true);
        }
        if declarations::parenthesized_declaration(self, lookahead)? {
            return Err(ParseError {
                message: String::from("parenthesized declarators are unsupported"),
                span: next.span,
            });
        }
        Ok(false)
    }

    pub(super) fn local_declaration(&mut self) -> Result<(), ParseError> {
        if self.starts_alias()? {
            return self.alias_declaration(false);
        }
        if self.take_ident("__shared__")? {
            return self.shared_declaration();
        }
        let (
            QualifiedType {
                ty: specifier,
                is_const,
            },
            is_constexpr,
        ) = self.declaration_type()?;
        if is_constexpr && !matches!(specifier, TypeSpecifier::Scalar(_)) {
            return self.fail("constexpr requires a scalar variable");
        }
        loop {
            let (name, span) = self.identifier()?;
            self.check_name(name, span)?;
            let declared_ty = self.lower_type(specifier, span);
            let base = self.type_handle(&declared_ty, span);
            let is_array = self.take_symbol('[')?;
            if is_constexpr && is_array {
                return self.fail("constexpr arrays are unsupported");
            }
            let (ty, array_count) = if is_array {
                let explicit_extent = if self.take_symbol(']')? {
                    None
                } else {
                    let extent = self.array_size()?;
                    self.expect_symbol(']')?;
                    Some(extent)
                };
                if self.lexer.peek()?.kind == TokenKind::Symbol('[') {
                    return self.fail("multidimensional local arrays are unsupported");
                }
                match explicit_extent {
                    Some((size, extent_span)) => (
                        self.array_type(base, ArraySize::Constant(size), extent_span, span)?,
                        Some(size),
                    ),
                    None => (base, None),
                }
            } else {
                (base, None)
            };
            let local = self.function.ir.local_variables.append(
                LocalVariable {
                    name: Some(String::from(name)),
                    ty,
                    init: None,
                },
                span,
            );
            let pointer = self.append(Expression::LocalVariable(local), span)?;
            let symbol = if is_array {
                Symbol::Array {
                    pointer,
                    ty: declared_ty.clone(),
                    writable: !is_const,
                }
            } else {
                Symbol::Variable {
                    pointer,
                    ty: declared_ty.clone(),
                    writable: !is_const,
                }
            };
            self.insert_symbol(name, symbol, span)?;
            let initializer = if is_array && array_count.is_none() {
                self.function.pending_array = Some(local);
                let count = self.inferred_array_initializer(pointer, specifier, base)?;
                let ty = self.array_type(base, ArraySize::Constant(count), span, span)?;
                self.function.ir.local_variables[local].ty = ty;
                self.function.pending_array = None;
                None
            } else if is_constexpr {
                let literal = self.constant_initializer(&declared_ty)?;
                self.function.constant_locals.insert(local, literal);
                Some(self.append(Expression::Literal(literal), span)?)
            } else if !is_const && matches!(self.lexer.peek()?.kind, TokenKind::Symbol(',' | ';')) {
                None
            } else if is_array
                || (matches!(specifier, TypeSpecifier::Vector { .. })
                    && self.starts_list_initializer()?)
            {
                self.aggregate_initializer(pointer, specifier, ty, array_count)?;
                None
            } else if is_const
                && matches!(
                    specifier,
                    TypeSpecifier::Scalar(Scalar::BOOL | Scalar::I32 | Scalar::U32)
                )
            {
                let (value, body) =
                    self.capture_block(|parser| parser.value_initializer(&declared_ty))?;
                if let Some(literal) = value.constant.literal {
                    self.function.constant_locals.insert(local, literal);
                    Some(self.append(Expression::Literal(literal), value.span)?)
                } else {
                    self.append_block(body);
                    Some(value.handle)
                }
            } else {
                Some(self.value_initializer(&declared_ty)?.handle)
            };
            let end = self.lexer.peek()?.span;
            let has_next_declarator = self.take_symbol(',')?;
            if !has_next_declarator {
                self.expect_symbol(';')?;
            }
            if let Some(initializer) = initializer {
                self.push_statement(
                    Statement::Store {
                        pointer,
                        value: initializer,
                    },
                    span.until(&end),
                );
            }
            if !has_next_declarator {
                return Ok(());
            }
        }
    }

    fn shared_declaration(&mut self) -> Result<(), ParseError> {
        if self.function.kind == FunctionKind::Device {
            return self.fail("shared storage is unsupported in device helpers");
        }
        let type_span = self.lexer.peek()?.span;
        let QualifiedType {
            ty: specifier,
            is_const,
        } = self.parse_type()?;
        if matches!(specifier, TypeSpecifier::Scalar(Scalar::BOOL)) || is_const {
            return Err(ParseError {
                message: String::from(
                    "shared storage requires a non-const numeric scalar or vector type",
                ),
                span: type_span,
            });
        }
        let (name, span) = self.identifier()?;
        self.check_name(name, span)?;
        let declared_ty = self.lower_type(specifier, span);
        let base = self.type_handle(&declared_ty, span);
        let is_array = self.take_symbol('[')?;
        let ty = if is_array {
            let (size, extent_span) = self.array_size()?;
            self.expect_symbol(']')?;
            self.array_type(base, ArraySize::Constant(size), extent_span, span)?
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
                ty: declared_ty,
                writable: true,
            }
        } else {
            Symbol::Variable {
                pointer,
                ty: declared_ty,
                writable: true,
            }
        };
        self.insert_symbol(name, symbol, span)
    }

    fn array_size(&mut self) -> Result<(NonZeroU32, Span), ParseError> {
        let (value, _) = self.capture_block(|parser| {
            let value = parser.expression(0)?;
            parser.rvalue(value)
        })?;
        let invalid = || ParseError {
            message: String::from("array extent must be a positive integral constant expression"),
            span: value.span,
        };
        let size = match value.constant.literal {
            Some(Literal::Bool(value)) => u32::from(value),
            Some(Literal::I32(value)) => u32::try_from(value).map_err(|_| invalid())?,
            Some(Literal::U32(value)) => value,
            _ => return Err(invalid()),
        };
        let size = NonZeroU32::new(size).ok_or_else(invalid)?;
        Ok((size, value.span))
    }
}
