use alloc::{format, string::String, vec::Vec};

use crate::{
    AddressSpace, ArraySize, BinaryOperator, Binding, BuiltIn, EntryPoint, Expression, FastHashMap,
    Function, FunctionArgument, GlobalVariable, Handle, Literal, LocalVariable, Module,
    ResourceBinding, Scalar, ScalarKind, ShaderStage, Span, Statement, StorageAccess, Type,
    TypeInner, UnaryOperator, VectorSize,
};

use super::{
    lexer::{Token, TokenKind},
    Options, ParseError,
};

#[derive(Clone, Copy, Debug)]
enum Symbol {
    Buffer {
        pointer: Handle<Expression>,
        scalar: Scalar,
        writable: bool,
    },
    Local {
        pointer: Handle<Expression>,
        scalar: Scalar,
        writable: bool,
    },
}

#[derive(Clone, Copy, Debug)]
struct Value {
    handle: Handle<Expression>,
    scalar: Scalar,
    span: Span,
}

pub(super) struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    position: usize,
    module: Module,
    function: Function,
    symbols: FastHashMap<&'a str, Symbol>,
    builtin_arguments: [Option<Handle<Expression>>; 3],
    workgroup_size: [u32; 3],
    expression_depth: u32,
    expression_graph_depths: Vec<u32>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>, options: &Options) -> Self {
        Self {
            tokens,
            position: 0,
            module: Module::default(),
            function: Function::default(),
            symbols: FastHashMap::default(),
            builtin_arguments: [None; 3],
            workgroup_size: options.workgroup_size,
            expression_depth: 0,
            expression_graph_depths: Vec::new(),
        }
    }

    pub fn parse(mut self) -> Result<Module, ParseError> {
        if self.take_ident("extern") {
            if self.current().kind != TokenKind::String("C") {
                return Err(self.error("expected `\"C\"` after `extern`"));
            }
            self.advance();
        }
        self.expect_ident("__global__")?;
        self.expect_ident("void")?;
        let (name, name_span) = self.identifier()?;
        self.check_name(name, name_span)?;
        self.expect_symbol('(')?;
        if !self.take_symbol(')') {
            if self.take_ident("void") {
                self.expect_symbol(')')?;
            } else {
                loop {
                    self.parameter()?;
                    if self.take_symbol(')') {
                        break;
                    }
                    self.expect_symbol(',')?;
                }
            }
        }
        self.expect_symbol('{')?;
        while !self.take_symbol('}') {
            if self.take_ident("return") {
                let span = self.expect_symbol(';')?;
                self.function
                    .body
                    .push(Statement::Return { value: None }, span);
                self.expect_symbol('}')?;
                break;
            }
            self.statement()?;
        }
        if self.current().kind != TokenKind::End {
            return Err(self.error("only one CUDA kernel is supported"));
        }
        self.module.entry_points.push(EntryPoint {
            name: String::from(name),
            stage: ShaderStage::Compute,
            early_depth_test: None,
            workgroup_size: self.workgroup_size,
            workgroup_size_overrides: None,
            function: self.function,
            mesh_info: None,
            task_payload: None,
            incoming_ray_payload: None,
        });
        Ok(self.module)
    }

    fn parameter(&mut self) -> Result<(), ParseError> {
        let is_const = self.take_ident("const");
        let scalar = self.scalar_type()?;
        let is_const = self.const_suffix(is_const)?;
        self.expect_symbol('*')?;
        let (name, span) = self.identifier()?;
        self.check_name(name, span)?;
        let base = self.scalar_handle(scalar, span);
        let ty = self.module.types.insert(
            Type {
                name: None,
                inner: TypeInner::Array {
                    base,
                    size: ArraySize::Dynamic,
                    stride: 4,
                },
            },
            span,
        );
        let binding = self.module.global_variables.len() as u32;
        let access = if is_const {
            StorageAccess::LOAD
        } else {
            StorageAccess::LOAD | StorageAccess::STORE
        };
        let global = self.module.global_variables.append(
            GlobalVariable {
                name: Some(String::from(name)),
                space: AddressSpace::Storage { access },
                binding: Some(ResourceBinding { group: 0, binding }),
                ty,
                init: None,
                memory_decorations: Default::default(),
            },
            span,
        );
        let pointer = self.append(Expression::GlobalVariable(global), span)?;
        self.symbols.insert(
            name,
            Symbol::Buffer {
                pointer,
                scalar,
                writable: !is_const,
            },
        );
        Ok(())
    }

    fn statement(&mut self) -> Result<(), ParseError> {
        if self.take_symbol(';') {
            return Ok(());
        }
        if matches!(
            self.current().kind,
            TokenKind::Ident("const" | "float" | "int" | "unsigned")
        ) {
            return self.local_declaration();
        }
        let (name, span) = self.identifier()?;
        let symbol = self.symbols.get(name).copied().ok_or_else(|| ParseError {
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
        if value.scalar != scalar {
            return Err(ParseError {
                message: String::from("assignment requires matching scalar types"),
                span: value.span,
            });
        }
        let end = self.expect_symbol(';')?;
        self.function.body.push(
            Statement::Store {
                pointer,
                value: value.handle,
            },
            span.until(&end),
        );
        Ok(())
    }

    fn local_declaration(&mut self) -> Result<(), ParseError> {
        let is_const = self.take_ident("const");
        let scalar = self.scalar_type()?;
        let is_const = self.const_suffix(is_const)?;
        let (name, span) = self.identifier()?;
        self.check_name(name, span)?;
        self.expect_symbol('=')?;
        let value = self.expression(0)?;
        if value.scalar != scalar {
            return Err(ParseError {
                message: String::from("initializer requires a matching scalar type"),
                span: value.span,
            });
        }
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
        self.symbols.insert(
            name,
            Symbol::Local {
                pointer,
                scalar,
                writable: !is_const,
            },
        );
        Ok(())
    }

    fn expression(&mut self, minimum_precedence: u8) -> Result<Value, ParseError> {
        self.expression_depth += 1;
        if self.expression_depth > 64 {
            return Err(self.error("CUDA expression nesting limit exceeded"));
        }
        let result = self.expression_inner(minimum_precedence);
        self.expression_depth -= 1;
        result
    }

    fn expression_inner(&mut self, minimum_precedence: u8) -> Result<Value, ParseError> {
        let mut left = self.primary()?;
        loop {
            let (operator, precedence) = match self.current().kind {
                TokenKind::Symbol('+') => (BinaryOperator::Add, 1),
                TokenKind::Symbol('-') => (BinaryOperator::Subtract, 1),
                TokenKind::Symbol('*') => (BinaryOperator::Multiply, 2),
                TokenKind::Symbol('/') => (BinaryOperator::Divide, 2),
                _ => break,
            };
            if precedence < minimum_precedence {
                break;
            }
            self.advance();
            let right = self.expression(precedence + 1)?;
            if left.scalar != right.scalar {
                return Err(ParseError {
                    message: String::from("arithmetic requires matching scalar types"),
                    span: left.span.until(&right.span),
                });
            }
            let span = left.span.until(&right.span);
            let handle = self.append(
                Expression::Binary {
                    op: operator,
                    left: left.handle,
                    right: right.handle,
                },
                span,
            )?;
            left = Value {
                handle,
                scalar: left.scalar,
                span,
            };
        }
        Ok(left)
    }

    fn primary(&mut self) -> Result<Value, ParseError> {
        let token = self.current();
        if self.take_symbol('(') {
            let value = self.expression(0)?;
            self.expect_symbol(')')?;
            return Ok(value);
        }
        if self.take_symbol('-') || self.take_symbol('+') {
            let value = self.expression(3)?;
            if token.kind == TokenKind::Symbol('+') {
                return Ok(value);
            }
            if value.scalar.kind == ScalarKind::Uint {
                return Err(ParseError {
                    message: String::from("unsigned negation is unsupported"),
                    span: token.span.until(&value.span),
                });
            }
            let span = token.span.until(&value.span);
            let handle = self.append(
                Expression::Unary {
                    op: UnaryOperator::Negate,
                    expr: value.handle,
                },
                span,
            )?;
            return Ok(Value {
                handle,
                scalar: value.scalar,
                span,
            });
        }
        match token.kind {
            TokenKind::Number(text) => {
                self.advance();
                self.number(text, token.span)
            }
            TokenKind::Ident(name) => {
                self.advance();
                if matches!(name, "threadIdx" | "blockIdx" | "blockDim" | "gridDim") {
                    return self.builtin(name, token.span);
                }
                let symbol = self.symbols.get(name).copied().ok_or_else(|| ParseError {
                    message: format!("unknown variable `{name}`"),
                    span: token.span,
                })?;
                let (pointer, scalar) = match symbol {
                    Symbol::Buffer {
                        pointer, scalar, ..
                    } => (self.buffer_element(pointer, token.span)?, scalar),
                    Symbol::Local {
                        pointer, scalar, ..
                    } => (pointer, scalar),
                };
                let handle = self.append(Expression::Load { pointer }, token.span)?;
                Ok(Value {
                    handle,
                    scalar,
                    span: token.span,
                })
            }
            _ => Err(self.error("expected a supported scalar expression")),
        }
    }

    fn buffer_element(
        &mut self,
        base: Handle<Expression>,
        span: Span,
    ) -> Result<Handle<Expression>, ParseError> {
        self.expect_symbol('[')?;
        let index = self.expression(0)?;
        if !matches!(index.scalar.kind, ScalarKind::Sint | ScalarKind::Uint) {
            return Err(ParseError {
                message: String::from("buffer index must be an integer"),
                span: index.span,
            });
        }
        let end = self.expect_symbol(']')?;
        self.append(
            Expression::Access {
                base,
                index: index.handle,
            },
            span.until(&end),
        )
    }

    fn builtin(&mut self, name: &str, start: Span) -> Result<Value, ParseError> {
        self.expect_symbol('.')?;
        let (component, span) = self.identifier()?;
        let index = match component {
            "x" => 0,
            "y" => 1,
            "z" => 2,
            _ => {
                return Err(ParseError {
                    message: String::from("CUDA dimension must be x, y, or z"),
                    span,
                })
            }
        };
        let span = start.until(&span);
        let handle = if name == "blockDim" {
            self.append(
                Expression::Literal(Literal::U32(self.workgroup_size[index as usize])),
                span,
            )?
        } else {
            let (slot, builtin) = match name {
                "threadIdx" => (0, BuiltIn::LocalInvocationId),
                "blockIdx" => (1, BuiltIn::WorkGroupId),
                _ => (2, BuiltIn::NumWorkGroups),
            };
            let base = if let Some(base) = self.builtin_arguments[slot] {
                base
            } else {
                let ty = self.module.types.insert(
                    Type {
                        name: None,
                        inner: TypeInner::Vector {
                            size: VectorSize::Tri,
                            scalar: Scalar::U32,
                        },
                    },
                    span,
                );
                let argument_index = self.function.arguments.len() as u32;
                self.function.arguments.push(FunctionArgument {
                    name: Some(String::from(name)),
                    ty,
                    binding: Some(Binding::BuiltIn(builtin)),
                });
                let base = self.append(Expression::FunctionArgument(argument_index), span)?;
                self.builtin_arguments[slot] = Some(base);
                base
            };
            self.append(Expression::AccessIndex { base, index }, span)?
        };
        Ok(Value {
            handle,
            scalar: Scalar::U32,
            span,
        })
    }

    fn number(&mut self, text: &str, span: Span) -> Result<Value, ParseError> {
        let invalid = || ParseError {
            message: String::from("unsupported or out-of-range numeric literal"),
            span,
        };
        let (literal, scalar) = if let Some(body) = text.strip_suffix(['f', 'F']) {
            if !body.contains(['.', 'e', 'E']) {
                return Err(invalid());
            }
            let value = body.parse::<f32>().map_err(|_| invalid())?;
            if !value.is_finite() {
                return Err(invalid());
            }
            (Literal::F32(value), Scalar::F32)
        } else {
            let (body, unsigned) = match text.strip_suffix(['u', 'U']) {
                Some(body) => (body, true),
                None => (text, false),
            };
            if body.is_empty()
                || !body.bytes().all(|byte| byte.is_ascii_digit())
                || (body.len() > 1 && body.starts_with('0'))
            {
                return Err(invalid());
            }
            if unsigned {
                (
                    Literal::U32(body.parse().map_err(|_| invalid())?),
                    Scalar::U32,
                )
            } else {
                (
                    Literal::I32(body.parse().map_err(|_| invalid())?),
                    Scalar::I32,
                )
            }
        };
        let handle = self.append(Expression::Literal(literal), span)?;
        Ok(Value {
            handle,
            scalar,
            span,
        })
    }

    fn scalar_type(&mut self) -> Result<Scalar, ParseError> {
        if self.take_ident("float") {
            Ok(Scalar::F32)
        } else if self.take_ident("int") {
            Ok(Scalar::I32)
        } else if self.take_ident("unsigned") {
            self.take_ident("int");
            Ok(Scalar::U32)
        } else {
            Err(self.error("expected `float`, `int`, or `unsigned int`"))
        }
    }

    fn const_suffix(&mut self, prefix: bool) -> Result<bool, ParseError> {
        let span = self.current().span;
        let suffix = self.take_ident("const");
        if prefix && suffix {
            Err(ParseError {
                message: String::from("duplicate const qualifier"),
                span,
            })
        } else {
            Ok(prefix || suffix)
        }
    }

    fn scalar_handle(&mut self, scalar: Scalar, span: Span) -> Handle<Type> {
        self.module.types.insert(
            Type {
                name: None,
                inner: TypeInner::Scalar(scalar),
            },
            span,
        )
    }

    fn append(
        &mut self,
        expression: Expression,
        span: Span,
    ) -> Result<Handle<Expression>, ParseError> {
        let depth = match expression {
            Expression::Unary { expr: operand, .. }
            | Expression::AccessIndex { base: operand, .. }
            | Expression::Load { pointer: operand } => {
                self.expression_graph_depths[operand.index()] + 1
            }
            Expression::Binary { left, right, .. }
            | Expression::Access {
                base: left,
                index: right,
            } => {
                self.expression_graph_depths[left.index()]
                    .max(self.expression_graph_depths[right.index()])
                    + 1
            }
            _ => 0,
        };
        if depth > 64 {
            return Err(ParseError {
                message: String::from("CUDA expression graph depth limit exceeded"),
                span,
            });
        }
        let pre_emitted = expression.needs_pre_emit();
        let handle = self.function.expressions.append(expression, span);
        self.expression_graph_depths.push(depth);
        if !pre_emitted {
            self.function.body.push(
                Statement::Emit(crate::Range::new_from_bounds(handle, handle)),
                span,
            );
        }
        Ok(handle)
    }

    fn check_name(&self, name: &str, span: Span) -> Result<(), ParseError> {
        if self.symbols.contains_key(name)
            || matches!(
                name,
                "threadIdx"
                    | "blockIdx"
                    | "gridDim"
                    | "blockDim"
                    | "const"
                    | "float"
                    | "int"
                    | "unsigned"
                    | "void"
                    | "return"
                    | "extern"
                    | "__global__"
            )
            || [
                "alignas",
                "alignof",
                "and",
                "and_eq",
                "asm",
                "auto",
                "bitand",
                "bitor",
                "bool",
                "break",
                "case",
                "catch",
                "char",
                "char8_t",
                "char16_t",
                "char32_t",
                "class",
                "compl",
                "concept",
                "consteval",
                "constexpr",
                "constinit",
                "const_cast",
                "continue",
                "co_await",
                "co_return",
                "co_yield",
                "decltype",
                "default",
                "delete",
                "do",
                "double",
                "dynamic_cast",
                "else",
                "enum",
                "explicit",
                "export",
                "false",
                "for",
                "friend",
                "goto",
                "if",
                "inline",
                "long",
                "mutable",
                "namespace",
                "new",
                "noexcept",
                "not",
                "not_eq",
                "nullptr",
                "operator",
                "or",
                "or_eq",
                "private",
                "protected",
                "public",
                "register",
                "reinterpret_cast",
                "requires",
                "short",
                "signed",
                "sizeof",
                "static",
                "static_assert",
                "static_cast",
                "struct",
                "switch",
                "template",
                "this",
                "thread_local",
                "throw",
                "true",
                "try",
                "typedef",
                "typeid",
                "typename",
                "union",
                "using",
                "virtual",
                "volatile",
                "wchar_t",
                "while",
                "xor",
                "xor_eq",
                "__device__",
                "__host__",
                "__shared__",
                "__constant__",
                "__managed__",
                "__restrict__",
                "__launch_bounds__",
            ]
            .contains(&name)
        {
            Err(ParseError {
                message: format!("duplicate or reserved name `{name}`"),
                span,
            })
        } else {
            Ok(())
        }
    }

    fn current(&self) -> Token<'a> {
        self.tokens[self.position]
    }

    fn advance(&mut self) {
        if self.current().kind != TokenKind::End {
            self.position += 1;
        }
    }

    fn take_symbol(&mut self, symbol: char) -> bool {
        if self.current().kind == TokenKind::Symbol(symbol) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expect_symbol(&mut self, symbol: char) -> Result<Span, ParseError> {
        let span = self.current().span;
        if self.take_symbol(symbol) {
            Ok(span)
        } else {
            Err(self.error(&format!("expected `{symbol}`")))
        }
    }

    fn take_ident(&mut self, identifier: &str) -> bool {
        if self.current().kind == TokenKind::Ident(identifier) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expect_ident(&mut self, identifier: &str) -> Result<(), ParseError> {
        if self.take_ident(identifier) {
            Ok(())
        } else {
            Err(self.error(&format!("expected `{identifier}`")))
        }
    }

    fn identifier(&mut self) -> Result<(&'a str, Span), ParseError> {
        let token = self.current();
        if let TokenKind::Ident(name) = token.kind {
            self.advance();
            Ok((name, token.span))
        } else {
            Err(self.error("expected an identifier"))
        }
    }

    fn error(&self, message: &str) -> ParseError {
        ParseError {
            message: String::from(message),
            span: self.current().span,
        }
    }
}
