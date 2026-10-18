use super::*;

impl<'a> Parser<'a> {
    pub(super) fn starts_constexpr_declaration(&mut self) -> Result<bool, ParseError> {
        let mut lexer = self.lexer.clone();
        loop {
            match lexer.peek()?.kind {
                TokenKind::Ident("constexpr") => return Ok(true),
                TokenKind::Ident("const" | "int" | "unsigned") => {}
                TokenKind::Ident(name) if self.type_name(name).is_some() => {}
                _ => return Ok(false),
            }
            lexer.next()?;
        }
    }

    pub(super) fn global_constant_declaration(&mut self) -> Result<(), ParseError> {
        let (ty, _) = self.declaration_type()?;
        let TypeSpecifier::Scalar(scalar) = ty.ty else {
            return self.fail("constexpr requires a scalar variable");
        };
        loop {
            let (name, span) = self.identifier()?;
            self.check_name(name, span)?;
            if self.global_declarations.contains_key(name) {
                return Err(ParseError {
                    message: format!("incompatible redeclaration of `{name}`"),
                    span,
                });
            }
            self.global_declarations.insert(
                name,
                GlobalDeclaration::Constant {
                    handle: None,
                    scalar,
                    literal: None,
                },
            );
            let literal = self.constant_initializer(&scalar_resolution(scalar))?;
            let ty = self.scalar_handle(scalar, span);
            let init = self
                .module
                .global_expressions
                .append(Expression::Literal(literal), span);
            let constant_handle = self.module.constants.append(
                crate::Constant {
                    name: Some(String::from(name)),
                    ty,
                    init,
                },
                span,
            );
            self.global_declarations.insert(
                name,
                GlobalDeclaration::Constant {
                    handle: Some(constant_handle),
                    scalar,
                    literal: Some(literal),
                },
            );
            self.function = FunctionContext::new(FunctionKind::Kernel);
            if !self.take_symbol(',')? {
                self.expect_symbol(';')?;
                return Ok(());
            }
        }
    }

    pub(super) fn constant_initializer(
        &mut self,
        ty: &TypeResolution,
    ) -> Result<Literal, ParseError> {
        let (value, _) = self.capture_block(|parser| parser.value_initializer(ty))?;
        Self::required_constant(&value)
    }

    pub(super) fn required_constant(value: &Value) -> Result<Literal, ParseError> {
        value.constant.literal.ok_or_else(|| ParseError {
            message: String::from("not a supported scalar constant expression"),
            span: value.span,
        })
    }

    pub(super) fn pointer_constant(&self, pointer: Handle<Expression>) -> ConstantValue {
        let (literal, is_discardable) = match self.function.ir.expressions[pointer] {
            Expression::LocalVariable(local) => {
                (self.function.constant_locals.get(&local).copied(), true)
            }
            Expression::GlobalVariable(global) => (
                None,
                self.module.global_variables[global].space == AddressSpace::WorkGroup,
            ),
            _ => (None, false),
        };
        ConstantValue {
            literal,
            is_discardable,
        }
    }

    pub(super) fn constant_array_address(
        &self,
        base: Handle<Expression>,
        index: Option<Literal>,
    ) -> bool {
        if !self.pointer_constant(base).is_discardable {
            return false;
        }
        let index = match index {
            Some(Literal::I32(index)) if index >= 0 => index as u32,
            Some(Literal::U32(index)) => index,
            _ => return false,
        };
        let ty = match self.function.ir.expressions[base] {
            Expression::LocalVariable(local) => self.function.ir.local_variables[local].ty,
            Expression::GlobalVariable(global) => self.module.global_variables[global].ty,
            _ => return false,
        };
        matches!(self.module.types[ty].inner, TypeInner::Array { size: ArraySize::Constant(count), .. } if index < count.get())
    }
}
