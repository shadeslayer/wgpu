use alloc::{format, string::String, vec::Vec};

mod barriers;
mod conditionals;
mod context;
mod conversions;
mod declarations;
mod expressions;
mod functions;
mod loops;
mod references;
mod statements;
mod types;
mod updates;

use context::{FunctionContext, FunctionKind};
use types::{GlobalDeclaration, QualifiedScalar};

use crate::{
    front::SymbolTable, AddressSpace, Arena, ArraySize, BinaryOperator, Binding, BuiltIn,
    EntryPoint, Expression, FastHashMap, Function, FunctionArgument, GlobalVariable, Handle,
    Literal, LocalVariable, Module, ResourceBinding, Scalar, ScalarKind, ShaderStage, Span,
    Statement, StorageAccess, StructMember, Type, TypeInner, UnaryOperator, VectorSize,
};

use super::{
    lexer::{Lexer, TokenKind},
    number, Options, ParseError,
};

#[derive(Clone, Copy, Debug)]
enum Symbol {
    Type(QualifiedScalar),
    Array {
        pointer: Handle<Expression>,
        scalar: Scalar,
        writable: bool,
    },
    Scalar {
        pointer: Handle<Expression>,
        scalar: Scalar,
        writable: bool,
    },
}

#[derive(Debug)]
struct ReferenceValue {
    reference: Reference,
    writable: bool,
    scalar: Scalar,
    span: Span,
}

#[derive(Debug)]
enum Reference {
    Pointer(Handle<Expression>),
    Conditional(Handle<Option<ConditionalReference>>),
}

#[derive(Debug)]
struct ReferenceBranch {
    body: crate::Block,
    reference: ReferenceValue,
}

#[derive(Debug)]
struct ConditionalReference {
    // Keep branch-local addresses in their blocks until the reference is consumed.
    condition: Handle<Expression>,
    accept: ReferenceBranch,
    reject: ReferenceBranch,
    depth: u32,
}

#[derive(Clone, Copy, Debug)]
struct Value {
    handle: Handle<Expression>,
    scalar: Scalar,
    span: Span,
}

#[derive(Debug)]
enum ScalarExpression {
    Value(Value),
    Reference(ReferenceValue),
}

impl ScalarExpression {
    const fn scalar(&self) -> Scalar {
        match *self {
            Self::Value(value) => value.scalar,
            Self::Reference(ref reference) => reference.scalar,
        }
    }

    const fn span(&self) -> Span {
        match *self {
            Self::Value(value) => value.span,
            Self::Reference(ref reference) => reference.span,
        }
    }
}

impl From<Value> for ScalarExpression {
    fn from(value: Value) -> Self {
        Self::Value(value)
    }
}

#[derive(Debug)]
enum ExpressionResult {
    Scalar(ScalarExpression),
    Void(Span),
}

impl From<Value> for ExpressionResult {
    fn from(value: Value) -> Self {
        Self::Scalar(value.into())
    }
}

impl ExpressionResult {
    fn require_scalar(self) -> Result<ScalarExpression, ParseError> {
        match self {
            Self::Scalar(value) => Ok(value),
            Self::Void(span) => Err(ParseError {
                message: String::from("void expression cannot be used as a value"),
                span,
            }),
        }
    }
}

pub(super) struct Parser<'a> {
    lexer: Lexer<'a>,
    module: Module,
    global_declarations: FastHashMap<&'a str, GlobalDeclaration>,
    function: FunctionContext<'a>,
    workgroup_size: [u32; 3],
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>, options: &Options) -> Self {
        Self {
            lexer,
            module: Module::default(),
            global_declarations: FastHashMap::default(),
            function: FunctionContext::new(FunctionKind::Kernel),
            workgroup_size: options.workgroup_size,
        }
    }

    pub fn parse(mut self) -> Result<Module, ParseError> {
        loop {
            if self.starts_alias()? {
                self.alias_declaration(true)?;
            } else if self.function_definition()? == FunctionKind::Kernel {
                break;
            }
        }
        while self.starts_alias()? {
            self.alias_declaration(true)?;
        }
        if self.lexer.peek()?.kind != TokenKind::End {
            return self.fail("only one CUDA kernel is supported");
        }
        // Remove call results from source statements discarded after a jump.
        crate::compact::compact(&mut self.module, crate::compact::KeepUnused::Yes);
        Ok(self.module)
    }

    fn parameter(&mut self) -> Result<(), ParseError> {
        let QualifiedScalar { scalar, is_const } = self.scalar_type()?;
        if self.function.kind == FunctionKind::Kernel && scalar == Scalar::BOOL {
            return self.fail("boolean kernel parameters are unsupported");
        }
        let pointer_span = self.lexer.peek()?.span;
        let is_pointer = self.take_symbol('*')?;
        if self.function.kind == FunctionKind::Device && is_pointer {
            return Err(ParseError {
                message: String::from("device helper pointer parameters are unsupported"),
                span: pointer_span,
            });
        }
        let (name, span) = self.identifier()?;
        self.check_name(name, span)?;
        if self.function.kind == FunctionKind::Device {
            return self.device_parameter(name, span, scalar, is_const);
        }
        if !is_pointer {
            return self.scalar_parameter(name, span, scalar, is_const);
        }
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
        self.insert_symbol(
            name,
            Symbol::Array {
                pointer,
                scalar,
                writable: !is_const,
            },
            span,
        )?;
        Ok(())
    }

    fn scalar_parameter(
        &mut self,
        name: &'a str,
        span: Span,
        scalar: Scalar,
        is_const: bool,
    ) -> Result<(), ParseError> {
        let scalar_ty = self.scalar_handle(scalar, span);
        let ty = self.module.types.insert(
            Type {
                name: None,
                inner: TypeInner::Struct {
                    members: alloc::vec![StructMember {
                        name: Some(String::from("value")),
                        ty: scalar_ty,
                        binding: None,
                        offset: 0,
                    }],
                    span: 16,
                },
            },
            span,
        );
        let binding = self.module.global_variables.len() as u32;
        let global = self.module.global_variables.append(
            GlobalVariable {
                name: Some(String::from(name)),
                space: AddressSpace::Uniform,
                binding: Some(ResourceBinding { group: 0, binding }),
                ty,
                init: None,
                memory_decorations: Default::default(),
            },
            span,
        );
        let base = self.append(Expression::GlobalVariable(global), span)?;
        let field = self.append(Expression::AccessIndex { base, index: 0 }, span)?;
        let value = self.append(Expression::Load { pointer: field }, span)?;
        self.initialize_parameter(name, span, scalar, is_const, value)
    }

    fn initialize_parameter(
        &mut self,
        name: &'a str,
        span: Span,
        scalar: Scalar,
        is_const: bool,
        value: Handle<Expression>,
    ) -> Result<(), ParseError> {
        let scalar_ty = self.scalar_handle(scalar, span);
        let local = self.function.ir.local_variables.append(
            LocalVariable {
                name: Some(String::from(name)),
                ty: scalar_ty,
                init: None,
            },
            span,
        );
        let pointer = self.append(Expression::LocalVariable(local), span)?;
        self.push_statement(Statement::Store { pointer, value }, span);
        self.insert_symbol(
            name,
            Symbol::Scalar {
                pointer,
                scalar,
                writable: !is_const,
            },
            span,
        )?;
        Ok(())
    }

    fn array_element(
        &mut self,
        base: Handle<Expression>,
        span: Span,
    ) -> Result<Handle<Expression>, ParseError> {
        self.expect_symbol('[')?;
        let index = self.expression(0)?;
        let index = self.rvalue(index)?;
        if !matches!(index.scalar.kind, ScalarKind::Sint | ScalarKind::Uint) {
            return Err(ParseError {
                message: String::from("array index must be an integer"),
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
        if self.function.kind == FunctionKind::Device {
            return Err(ParseError {
                message: String::from("compute builtins are unsupported in device helpers"),
                span: start,
            });
        }
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
            let base = if let Some(base) = self.function.builtin_arguments[slot] {
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
                let argument_index = self.function.ir.arguments.len() as u32;
                self.function.ir.arguments.push(FunctionArgument {
                    name: Some(String::from(name)),
                    ty,
                    binding: Some(Binding::BuiltIn(builtin)),
                });
                let base = self.append(Expression::FunctionArgument(argument_index), span)?;
                self.function.builtin_arguments[slot] = Some(base);
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
        let literal = number::parse(text).ok_or_else(invalid)?;
        let handle = self.append(Expression::Literal(literal), span)?;
        Ok(Value {
            handle,
            scalar: literal.scalar(),
            span,
        })
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
            | Expression::As { expr: operand, .. }
            | Expression::AccessIndex { base: operand, .. }
            | Expression::Load { pointer: operand } => {
                self.function.expression_graph_depths[operand.index()] + 1
            }
            Expression::Binary { left, right, .. }
            | Expression::Access {
                base: left,
                index: right,
            } => {
                self.function.expression_graph_depths[left.index()]
                    .max(self.function.expression_graph_depths[right.index()])
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
        if pre_emitted {
            self.function
                .ir
                .body
                .extend(self.function.emitter.finish(&self.function.ir.expressions));
        }
        let handle = self.function.ir.expressions.append(expression, span);
        self.function.expression_graph_depths.push(depth);
        if pre_emitted {
            self.function.emitter.start(&self.function.ir.expressions);
        }
        Ok(handle)
    }

    fn check_name(&self, name: &str, span: Span) -> Result<(), ParseError> {
        if Self::reserved_name(name) {
            Err(ParseError {
                message: format!("duplicate or reserved name `{name}`"),
                span,
            })
        } else {
            Ok(())
        }
    }

    fn reserved_name(name: &str) -> bool {
        matches!(
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
                | "__device__"
                | "__syncthreads"
        ) || [
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
    }

    fn insert_symbol(
        &mut self,
        name: &'a str,
        symbol: Symbol,
        span: Span,
    ) -> Result<(), ParseError> {
        if self.function.symbol_table.add(name, symbol).is_some() {
            Err(ParseError {
                message: format!("duplicate or reserved name `{name}`"),
                span,
            })
        } else {
            Ok(())
        }
    }

    fn lookup_symbol(&self, name: &str) -> Option<Symbol> {
        self.function.symbol_table.lookup(name).copied()
    }

    fn take_symbol(&mut self, symbol: char) -> Result<bool, ParseError> {
        self.lexer.next_if(TokenKind::Symbol(symbol))
    }

    fn expect_symbol(&mut self, symbol: char) -> Result<Span, ParseError> {
        self.lexer.expect_span(TokenKind::Symbol(symbol))
    }

    fn take_ident(&mut self, identifier: &str) -> Result<bool, ParseError> {
        self.lexer.next_if(TokenKind::Ident(identifier))
    }

    fn expect_ident(&mut self, identifier: &str) -> Result<(), ParseError> {
        self.lexer.expect_span(TokenKind::Ident(identifier))?;
        Ok(())
    }

    fn identifier(&mut self) -> Result<(&'a str, Span), ParseError> {
        let token = self.lexer.next()?;
        if let TokenKind::Ident(name) = token.kind {
            Ok((name, token.span))
        } else {
            Err(ParseError {
                message: String::from("expected an identifier"),
                span: token.span,
            })
        }
    }

    fn fail<T>(&mut self, message: &str) -> Result<T, ParseError> {
        Err(ParseError {
            message: String::from(message),
            span: self.lexer.peek()?.span,
        })
    }
}
