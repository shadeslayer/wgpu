use super::*;

#[derive(Clone, Debug, PartialEq)]
pub(super) struct QualifiedType {
    pub ty: TypeResolution,
    pub is_const: bool,
}

#[derive(Clone, Debug)]
pub(super) enum GlobalDeclaration {
    Type(QualifiedType),
    Functions(Vec<Handle<Function>>),
    Kernel,
}

fn type_name(
    name: &str,
    symbols: &SymbolTable<&str, Symbol>,
    globals: &FastHashMap<&str, GlobalDeclaration>,
) -> Option<QualifiedType> {
    let scalar = match name {
        "bool" => Scalar::BOOL,
        "float" => Scalar::F32,
        "int" => Scalar::I32,
        "unsigned" => Scalar::U32,
        _ => {
            return match symbols.lookup(name) {
                Some(&Symbol::Type(ref ty)) => Some(ty.clone()),
                Some(_) => None,
                None => match globals.get(name) {
                    Some(&GlobalDeclaration::Type(ref ty)) => Some(ty.clone()),
                    _ => None,
                },
            };
        }
    };
    Some(QualifiedType {
        ty: scalar_resolution(scalar),
        is_const: false,
    })
}

fn parse_type(
    lexer: &mut Lexer<'_>,
    symbols: &SymbolTable<&str, Symbol>,
    globals: &FastHashMap<&str, GlobalDeclaration>,
) -> Result<QualifiedType, ParseError> {
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
    pub(super) fn type_handle(&mut self, ty: &TypeResolution, span: Span) -> Handle<Type> {
        match ty.clone() {
            TypeResolution::Handle(handle) => handle,
            TypeResolution::Value(inner) => {
                self.module.types.insert(Type { name: None, inner }, span)
            }
        }
    }

    pub(super) fn require_scalar_type(
        &self,
        ty: &TypeResolution,
        span: Span,
    ) -> Result<Scalar, ParseError> {
        match *ty.inner_with(&self.module.types) {
            TypeInner::Scalar(scalar) => Ok(scalar),
            _ => Err(ParseError {
                message: String::from("expected a supported scalar expression"),
                span,
            }),
        }
    }

    pub(super) fn type_name(&self, name: &str) -> Option<QualifiedType> {
        type_name(name, &self.function.symbol_table, &self.global_declarations)
    }

    pub(super) fn parse_type(&mut self) -> Result<QualifiedType, ParseError> {
        parse_type(
            &mut self.lexer,
            &self.function.symbol_table,
            &self.global_declarations,
        )
    }

    pub(super) fn parse_type_from(
        &self,
        lexer: &mut Lexer<'_>,
    ) -> Result<QualifiedType, ParseError> {
        parse_type(
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
            let ty = self.parse_type()?;
            loop {
                let (name, span) = self.identifier()?;
                self.insert_type(name, ty.clone(), span, global)?;
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
        let ty = self.parse_type()?;
        self.expect_symbol(';')?;
        self.insert_type(name, ty, span, global)
    }

    fn insert_type(
        &mut self,
        name: &'a str,
        ty: QualifiedType,
        span: Span,
        global: bool,
    ) -> Result<(), ParseError> {
        self.check_name(name, span)?;
        let compatible = if global {
            match self
                .global_declarations
                .insert(name, GlobalDeclaration::Type(ty.clone()))
            {
                None => true,
                Some(GlobalDeclaration::Type(previous)) => {
                    ty.is_const == previous.is_const
                        && self.module.compare_types(&ty.ty, &previous.ty)
                }
                Some(_) => false,
            }
        } else {
            match self
                .function
                .symbol_table
                .add(name, Symbol::Type(ty.clone()))
            {
                None => true,
                Some(Symbol::Type(previous)) => {
                    ty.is_const == previous.is_const
                        && self.module.compare_types(&ty.ty, &previous.ty)
                }
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

pub(super) const fn scalar_resolution(scalar: Scalar) -> TypeResolution {
    TypeResolution::Value(TypeInner::Scalar(scalar))
}
