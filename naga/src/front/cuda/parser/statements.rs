use super::*;

use core::num::NonZeroU32;

pub(super) struct ArrayDeclaration {
    pub(super) ty: Handle<Type>,
    element_ty: Handle<Type>,
    count: Option<NonZeroU32>,
}

impl<'a> Parser<'a> {
    pub(super) fn block_contents(&mut self) -> Result<bool, ParseError> {
        let mut terminated = false;
        while !self.lexer.next_if(Token::Paren('}'))? {
            if self.lexer.peek()?.0 == Token::End {
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
        let compound = self.lexer.next_if(Token::Paren('{'))?;
        let initialization_start = self.function.push_scope();
        let result = self.capture_block(|parser| {
            if compound {
                parser.block_contents()
            } else {
                parser.statement()
            }
        });
        self.function.pop_scope(initialization_start);
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
        let start = token.1;
        if self.lexer.next_if(Token::Separator(';'))? {
            return Ok(false);
        }
        if self.lexer.peek()?.0 == Token::Paren('{') {
            let (body, terminated) = self.scoped_statement()?;
            self.push_statement(Statement::Block(body), start);
            return Ok(terminated);
        }
        if self.take_ident("if")? {
            self.lexer.expect_span(Token::Paren('('))?;
            let value = self.comma_expression()?;
            let condition = self.truthy(value)?;
            self.lexer.expect_span(Token::Paren(')'))?;
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
        if self.take_ident("switch")? {
            return self.switch_statement(start);
        }
        if matches!(token.0, Token::Word("case" | "default")) {
            return Err(ParseError {
                message: String::from("case and default labels require a direct switch body"),
                span: start,
            });
        }
        if self.take_ident("return")? {
            let value = if let Some(ref result) = self.function.ir.result {
                let ty = TypeResolution::Handle(result.ty);
                let value = self.comma_expression()?;
                Some(self.convert_value(value, &ty)?.handle)
            } else {
                if self.lexer.peek()?.0 != Token::Separator(';') {
                    if let ExpressionResult::Value(value) = self.comma_expression_result()? {
                        return Err(ParseError {
                            message: String::from("cannot return a value from a void function"),
                            span: value.span(),
                        });
                    }
                }
                None
            };
            let end = self.lexer.expect_span(Token::Separator(';'))?;
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
            self.lexer.expect_span(Token::Paren('('))?;
            self.lexer.expect_span(Token::Paren(')'))?;
            let end = self.lexer.expect_span(Token::Separator(';'))?;
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
        if matches!(token.0, Token::Word("break" | "continue")) {
            self.lexer.next()?;
            if self.function.loop_depth == 0
                && (token.0 == Token::Word("continue") || self.function.switch_depth == 0)
            {
                return Err(ParseError {
                    message: if token.0 == Token::Word("continue") {
                        String::from("continue requires an enclosing loop")
                    } else {
                        String::from("break requires an enclosing loop or switch")
                    },
                    span: start,
                });
            }
            let statement = if token.0 == Token::Word("break") {
                Statement::Break
            } else {
                Statement::Continue
            };
            let end = self.lexer.expect_span(Token::Separator(';'))?;
            self.push_statement(statement, start.until(&end));
            return Ok(true);
        }
        if self.starts_declaration()? {
            self.local_declaration()?;
            return Ok(false);
        }
        self.discard_expression()?;
        self.lexer.expect_span(Token::Separator(';'))?;
        Ok(false)
    }

    pub(super) fn starts_declaration(&mut self) -> Result<bool, ParseError> {
        if self.starts_alias()?
            || matches!(self.lexer.peek()?.0, Token::Word("__shared__" | "struct"))
        {
            return Ok(true);
        }
        if !self.starts_type()? {
            return Ok(false);
        }
        let mut lookahead = self.lexer.clone();
        lookahead.next()?;
        let next = lookahead.peek()?;
        if next.0 != Token::Paren('(') {
            return Ok(true);
        }
        if declarations::parenthesized_declaration(self, lookahead)? {
            return Err(ParseError {
                message: String::from("parenthesized declarators are unsupported"),
                span: next.1,
            });
        }
        Ok(false)
    }

    pub(super) fn local_declaration(&mut self) -> Result<(), ParseError> {
        if self.lexer.peek()?.0 == Token::Word("struct") {
            return self.structure_declaration(false);
        }
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
            let is_array = self.lexer.next_if(Token::Paren('['))?;
            if is_constexpr && is_array {
                return self.fail("constexpr arrays are unsupported");
            }
            let array = if is_array {
                Some(self.array_declaration(base, true, span)?)
            } else {
                None
            };
            let ty = array.as_ref().map_or(base, |array| array.ty);
            let array_count = array.as_ref().and_then(|array| array.count);
            let has_initializer = !matches!(self.lexer.peek()?.0, Token::Separator(',' | ';'));
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
                    ty: TypeResolution::Handle(array.as_ref().unwrap().element_ty),
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
                let element_ty = array.as_ref().unwrap().element_ty;
                let count = self.inferred_array_initializer(pointer, specifier, element_ty)?;
                let ty = self.array_type(element_ty, ArraySize::Constant(count), span, span)?;
                self.function.ir.local_variables[local].ty = ty;
                self.function.pending_array = None;
                None
            } else if is_constexpr {
                let literal = self.constant_initializer(&declared_ty)?;
                self.function.constant_locals.insert(local, literal);
                Some(self.append(Expression::Literal(literal), span)?)
            } else if !is_const && matches!(self.lexer.peek()?.0, Token::Separator(',' | ';')) {
                None
            } else if is_array
                || (matches!(
                    specifier,
                    TypeSpecifier::Vector { .. } | TypeSpecifier::Structure(_)
                ) && self.starts_list_initializer()?)
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
            if has_initializer {
                self.function.initialized_declarations.push(span);
            }
            let end = self.lexer.peek()?.1;
            let has_next_declarator = self.lexer.next_if(Token::Separator(','))?;
            if !has_next_declarator {
                self.lexer.expect_span(Token::Separator(';'))?;
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
        let type_span = self.lexer.peek()?.1;
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
        let is_array = self.lexer.next_if(Token::Paren('['))?;
        let array = if is_array {
            Some(self.array_declaration(base, false, span)?)
        } else {
            None
        };
        let ty = array.as_ref().map_or(base, |array| array.ty);
        if self.lexer.peek()?.0 == Token::Operation('=') {
            return self.fail("shared variables cannot have initializers");
        }
        self.lexer.expect_span(Token::Separator(';'))?;
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
                ty: TypeResolution::Handle(array.as_ref().unwrap().element_ty),
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

    pub(super) fn array_declaration(
        &mut self,
        base: Handle<Type>,
        allow_inferred: bool,
        span: Span,
    ) -> Result<ArrayDeclaration, ParseError> {
        let mut dimensions = Vec::new();
        loop {
            if dimensions.len() == 64 {
                return self.fail("CUDA array rank limit exceeded");
            }
            let extent = if self.lexer.peek()?.0 == Token::Paren(']') {
                if !allow_inferred || !dimensions.is_empty() {
                    return self.fail("only the outermost local array bound may be inferred");
                }
                (None, self.lexer.next()?.1)
            } else {
                let (count, extent_span) = self.array_size()?;
                self.lexer.expect_span(Token::Paren(']'))?;
                (Some(count), extent_span)
            };
            dimensions.push(extent);
            if !self.lexer.next_if(Token::Paren('['))? {
                break;
            }
        }
        let mut element_ty = base;
        for &(count, extent_span) in dimensions[1..].iter().rev() {
            element_ty = self.array_type(
                element_ty,
                ArraySize::Constant(count.unwrap()),
                extent_span,
                span,
            )?;
        }
        let (count, extent_span) = dimensions[0];
        let ty = match count {
            Some(count) => {
                self.array_type(element_ty, ArraySize::Constant(count), extent_span, span)?
            }
            None => element_ty,
        };
        Ok(ArrayDeclaration {
            ty,
            element_ty,
            count,
        })
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
