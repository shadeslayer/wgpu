use super::*;

pub(super) struct Structure {
    pub(super) depth: u32,
}

impl<'a> Parser<'a> {
    pub(super) fn structure_declaration(&mut self, global: bool) -> Result<(), ParseError> {
        let start = self.lexer.expect_span(Token::Word("struct"))?;
        let (name, name_span) = self.identifier()?;
        self.check_name(name, name_span)?;
        if self.lexer.peek()?.0 != Token::Paren('{') {
            return self.fail("incomplete and elaborated struct declarations are unsupported");
        }
        if global && self.global_declarations.contains_key(name) {
            return Err(ParseError {
                message: format!("duplicate structure definition `{name}`"),
                span: name_span,
            });
        }
        self.lexer.next()?;
        let initialization_start = self.function.push_scope();
        self.function.symbol_table.add(name, Symbol::Member);
        let definition = self.structure_members();
        self.function.pop_scope(initialization_start);
        let (members, size, depth) = definition?;
        self.lexer.expect_span(Token::Separator(';'))?;
        let declaration_id = self.next_structure_id;
        self.next_structure_id = declaration_id.checked_add(1).ok_or_else(|| ParseError {
            message: String::from("too many structure declarations"),
            span: name_span,
        })?;
        // Equal names and layouts must still have distinct nominal identities.
        let handle = self.module.types.insert(
            Type {
                name: Some(format!("{name}_cuda_record_{declaration_id}")),
                inner: TypeInner::Struct {
                    members,
                    span: size,
                },
            },
            self.lexer.span_from(start.to_range().unwrap().start),
        );
        self.structures.insert(handle, Structure { depth });
        self.insert_type(
            name,
            QualifiedType {
                ty: TypeSpecifier::Structure(handle),
                is_const: false,
            },
            name_span,
            global,
        )
    }

    fn structure_members(&mut self) -> Result<(Vec<StructMember>, u32, u32), ParseError> {
        let mut members = Vec::new();
        let mut names = crate::FastHashSet::default();
        let mut offset = 0;
        let mut alignment = crate::proc::Alignment::ONE;
        let mut depth = 1u32;
        while !self.lexer.next_if(Token::Paren('}'))? {
            let type_span = self.lexer.peek()?.1;
            let QualifiedType {
                ty: specifier,
                is_const,
            } = self.parse_type()?;
            if is_const || matches!(specifier, TypeSpecifier::Scalar(Scalar::BOOL)) {
                return Err(ParseError {
                    message: String::from(
                        "structure members require non-const numeric value types",
                    ),
                    span: type_span,
                });
            }
            let resolution = self.lower_type(specifier, type_span);
            let base = self.type_handle(&resolution, type_span);
            loop {
                let (name, span) = self.identifier()?;
                if !matches!(name, "threadIdx" | "blockIdx" | "gridDim" | "blockDim") {
                    self.check_name(name, span)?;
                }
                if !names.insert(name) {
                    return Err(ParseError {
                        message: format!("duplicate structure member `{name}`"),
                        span,
                    });
                }
                let ty = if self.lexer.next_if(Token::Paren('['))? {
                    self.array_declaration(base, false, span)?.ty
                } else {
                    base
                };
                let member_depth = self.structure_field_depth(ty);
                depth = depth.max(1 + member_depth);
                if depth > 64 {
                    return Err(ParseError {
                        message: String::from("CUDA structure nesting limit exceeded"),
                        span,
                    });
                }
                self.layouter
                    .update(self.module.to_ctx())
                    .map_err(|error| ParseError {
                        message: format!("invalid structure layout: {error}"),
                        span,
                    })?;
                let layout = self.layouter[ty];
                offset = Self::structure_aligned_offset(offset, layout.alignment, span)?;
                members.push(StructMember {
                    name: Some(String::from(name)),
                    ty,
                    binding: None,
                    offset,
                });
                offset = offset
                    .checked_add(layout.size)
                    .filter(|&size| size <= crate::valid::MAX_TYPE_SIZE)
                    .ok_or_else(|| ParseError {
                        message: String::from("structure exceeds Naga's type size limit"),
                        span,
                    })?;
                alignment = alignment.max(layout.alignment);
                self.function.symbol_table.add(name, Symbol::Member);
                if !self.lexer.next_if(Token::Separator(','))? {
                    self.lexer.expect_span(Token::Separator(';'))?;
                    break;
                }
            }
        }
        if members.is_empty() {
            return self.fail("empty structures are unsupported");
        }
        let span = self.lexer.peek()?.1;
        let size = Self::structure_aligned_offset(offset, alignment, span)?;
        Ok((members, size, depth))
    }

    fn structure_field_depth(&self, mut ty: Handle<Type>) -> u32 {
        let mut depth = 0;
        while let TypeInner::Array { base, .. } = self.module.types[ty].inner {
            depth += 1;
            ty = base;
        }
        depth
            + self
                .structures
                .get(&ty)
                .map_or(0, |structure| structure.depth)
    }

    fn structure_aligned_offset(
        offset: u32,
        alignment: crate::proc::Alignment,
        span: Span,
    ) -> Result<u32, ParseError> {
        offset
            .checked_add(alignment * 1 - 1)
            .filter(|&size| size <= crate::valid::MAX_TYPE_SIZE)
            .map(|_| alignment.round_up(offset))
            .ok_or_else(|| ParseError {
                message: String::from("structure exceeds Naga's type size limit"),
                span,
            })
    }

    pub(super) fn structure_value(
        &mut self,
        ty: Handle<Type>,
        span: Span,
    ) -> Result<Value, ParseError> {
        let resolution = TypeResolution::Handle(ty);
        let pointer = self.temporary_pointer(&resolution, span)?;
        self.aggregate_initializer(pointer, TypeSpecifier::Structure(ty), ty, None)?;
        let span = self.lexer.span_from(span.to_range().unwrap().start);
        self.load_pointer(pointer, &resolution, span)
    }
}
