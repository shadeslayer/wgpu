use super::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct QualifiedScalar {
    pub scalar: Scalar,
    pub is_const: bool,
}

#[derive(Clone, Debug)]
pub(super) enum GlobalDeclaration {
    Type(QualifiedScalar),
    Functions(Vec<Handle<Function>>),
    Kernel,
}

fn type_name(
    name: &str,
    symbols: &SymbolTable<&str, Symbol>,
    globals: &FastHashMap<&str, GlobalDeclaration>,
) -> Option<QualifiedScalar> {
    let scalar = match name {
        "bool" => Scalar::BOOL,
        "float" => Scalar::F32,
        "int" => Scalar::I32,
        "unsigned" => Scalar::U32,
        _ => {
            return match symbols.lookup(name) {
                Some(&Symbol::Type(ty)) => Some(ty),
                Some(_) => None,
                None => match globals.get(name) {
                    Some(&GlobalDeclaration::Type(ty)) => Some(ty),
                    _ => None,
                },
            };
        }
    };
    Some(QualifiedScalar {
        scalar,
        is_const: false,
    })
}

fn scalar_type(
    lexer: &mut Lexer<'_>,
    symbols: &SymbolTable<&str, Symbol>,
    globals: &FastHashMap<&str, GlobalDeclaration>,
) -> Result<QualifiedScalar, ParseError> {
    let prefix = lexer.next_if(TokenKind::Ident("const"))?;
    let token = lexer.next()?;
    let mut ty = match token.kind {
        TokenKind::Ident(name) => {
            let ty = type_name(name, symbols, globals).ok_or_else(|| ParseError {
                message: format!("unknown or unsupported scalar type `{name}`"),
                span: token.span,
            })?;
            if name == "unsigned" {
                lexer.next_if(TokenKind::Ident("int"))?;
            }
            ty
        }
        _ => {
            return Err(ParseError {
                message: String::from("expected a scalar type"),
                span: token.span,
            });
        }
    };
    let suffix_span = lexer.peek()?.span;
    let suffix = lexer.next_if(TokenKind::Ident("const"))?;
    if prefix && suffix {
        return Err(ParseError {
            message: String::from("duplicate const qualifier"),
            span: suffix_span,
        });
    }
    ty.is_const |= prefix || suffix;
    Ok(ty)
}

impl<'a> Parser<'a> {
    pub(super) fn type_name(&self, name: &str) -> Option<QualifiedScalar> {
        type_name(name, &self.function.symbol_table, &self.global_declarations)
    }

    pub(super) fn scalar_type(&mut self) -> Result<QualifiedScalar, ParseError> {
        scalar_type(
            &mut self.lexer,
            &self.function.symbol_table,
            &self.global_declarations,
        )
    }

    pub(super) fn scalar_type_from(
        &self,
        lexer: &mut Lexer<'_>,
    ) -> Result<QualifiedScalar, ParseError> {
        scalar_type(
            lexer,
            &self.function.symbol_table,
            &self.global_declarations,
        )
    }

    pub(super) fn starts_type(&mut self) -> Result<bool, ParseError> {
        Ok(match self.lexer.peek()?.kind {
            TokenKind::Ident("const") => true,
            TokenKind::Ident(name) => self.type_name(name).is_some(),
            _ => false,
        })
    }

    pub(super) fn starts_alias(&mut self) -> Result<bool, ParseError> {
        Ok(matches!(
            self.lexer.peek()?.kind,
            TokenKind::Ident("typedef" | "using")
        ))
    }

    pub(super) fn alias_declaration(&mut self, global: bool) -> Result<(), ParseError> {
        if self.take_ident("typedef")? {
            let ty = self.scalar_type()?;
            loop {
                let (name, span) = self.identifier()?;
                self.insert_type(name, ty, span, global)?;
                if self.take_symbol(';')? {
                    return Ok(());
                }
                self.expect_symbol(',')?;
            }
        }
        self.expect_ident("using")?;
        let (name, span) = self.identifier()?;
        self.check_name(name, span)?;
        self.expect_symbol('=')?;
        let ty = self.scalar_type()?;
        self.expect_symbol(';')?;
        self.insert_type(name, ty, span, global)
    }

    fn insert_type(
        &mut self,
        name: &'a str,
        ty: QualifiedScalar,
        span: Span,
        global: bool,
    ) -> Result<(), ParseError> {
        self.check_name(name, span)?;
        let compatible = if global {
            match self
                .global_declarations
                .insert(name, GlobalDeclaration::Type(ty))
            {
                None => true,
                Some(GlobalDeclaration::Type(previous)) => ty == previous,
                Some(_) => false,
            }
        } else {
            match self.function.symbol_table.add(name, Symbol::Type(ty)) {
                None => true,
                Some(Symbol::Type(previous)) => ty == previous,
                Some(_) => false,
            }
        };
        if compatible {
            Ok(())
        } else {
            Err(ParseError {
                message: format!("incompatible redeclaration of `{name}`"),
                span,
            })
        }
    }
}
