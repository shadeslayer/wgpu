use alloc::{format, string::String, vec::Vec};

mod absolute_value;
mod barriers;
mod conditionals;
mod constant_evaluation;
mod constants;
mod context;
mod conversions;
mod declarations;
mod expressions;
mod floorf;
mod functions;
mod initializers;
mod loops;
mod operators;
mod references;
mod sqrtf;
mod statements;
mod structures;
mod switches;
mod trigonometry;
mod types;
mod updates;

use crate::proc::TypeResolution;
use context::{FunctionContext, FunctionKind};
use types::scalar_resolution;
use types::{GlobalDeclaration, QualifiedType, TypeSpecifier};

use crate::{
    front::SymbolTable, AddressSpace, Arena, ArraySize, BinaryOperator, Binding, BuiltIn,
    EntryPoint, Expression, FastHashMap, Function, FunctionArgument, GlobalVariable, Handle,
    Literal, LocalVariable, Module, ResourceBinding, Scalar, ScalarKind, ShaderStage, Span,
    Statement, StorageAccess, StructMember, Type, TypeInner, UnaryOperator, VectorSize,
};

use super::{
    lexer::{Lexer, Token},
    number, Options, ParseError,
};

const BUILTIN_INPUTS: [(&str, BuiltIn); 3] = [
    ("threadIdx", BuiltIn::LocalInvocationId),
    ("blockIdx", BuiltIn::WorkGroupId),
    ("gridDim", BuiltIn::NumWorkGroups),
];

#[derive(Clone, Debug)]
enum Symbol {
    Type(QualifiedType),
    Member,
    Array {
        pointer: Handle<Expression>,
        ty: TypeResolution,
        writable: bool,
    },
    Variable {
        pointer: Handle<Expression>,
        ty: TypeResolution,
        writable: bool,
    },
    Constant {
        handle: Option<Handle<crate::Constant>>,
        scalar: Scalar,
        literal: Option<Literal>,
    },
}

#[derive(Clone, Copy, Debug, Default)]
struct ConstantValue {
    literal: Option<Literal>,
    // Discarded glvalues can qualify without a constant rvalue.
    is_discardable: bool,
}

impl From<Literal> for ConstantValue {
    fn from(literal: Literal) -> Self {
        Self {
            literal: Some(literal),
            is_discardable: true,
        }
    }
}

impl ConstantValue {
    fn evaluated(literal: Option<Literal>) -> Self {
        literal.map(Self::from).unwrap_or_default()
    }

    fn after(self, earlier: Self) -> Self {
        if earlier.is_discardable {
            self
        } else {
            Self::default()
        }
    }
}

#[derive(Debug)]
struct ReferenceValue {
    reference: Reference,
    writable: bool,
    ty: TypeResolution,
    span: Span,
    constant: ConstantValue,
}

#[derive(Debug)]
enum Reference {
    Pointer(Handle<Expression>),
    Conditional(Handle<Option<ConditionalReference>>),
    Constant(Option<Handle<crate::Constant>>),
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

#[derive(Clone, Debug)]
struct Value {
    handle: Handle<Expression>,
    ty: TypeResolution,
    span: Span,
    constant: ConstantValue,
}

#[derive(Debug)]
enum ExpressionValue {
    Plain(Value),
    Reference(ReferenceValue),
}

impl ExpressionValue {
    const fn constant(&self) -> ConstantValue {
        match *self {
            Self::Plain(ref value) => value.constant,
            Self::Reference(ref reference) => reference.constant,
        }
    }

    const fn set_constant(&mut self, constant: ConstantValue) {
        match *self {
            Self::Plain(ref mut value) => value.constant = constant,
            Self::Reference(ref mut reference) => reference.constant = constant,
        }
    }

    const fn ty(&self) -> &TypeResolution {
        match *self {
            Self::Plain(ref value) => &value.ty,
            Self::Reference(ref reference) => &reference.ty,
        }
    }

    const fn span(&self) -> Span {
        match *self {
            Self::Plain(ref value) => value.span,
            Self::Reference(ref reference) => reference.span,
        }
    }
}

impl From<Value> for ExpressionValue {
    fn from(value: Value) -> Self {
        Self::Plain(value)
    }
}

#[derive(Debug)]
enum ExpressionResult {
    Value(ExpressionValue),
    Void(Span),
}

impl From<Value> for ExpressionResult {
    fn from(value: Value) -> Self {
        Self::Value(value.into())
    }
}

impl ExpressionResult {
    fn require_value(self) -> Result<ExpressionValue, ParseError> {
        match self {
            Self::Value(value) => Ok(value),
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
    layouter: crate::proc::Layouter,
    global_declarations: FastHashMap<&'a str, GlobalDeclaration>,
    function: FunctionContext<'a>,
    sqrtf_function: Option<Handle<Function>>,
    trigonometry_function: Option<Handle<Function>>,
    structures: FastHashMap<Handle<Type>, structures::Structure>,
    next_structure_id: u32,
    helper_builtin_globals: [Option<Handle<GlobalVariable>>; 3],
    workgroup_size: [u32; 3],
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>, options: &Options) -> Self {
        Self {
            lexer,
            module: Module::default(),
            layouter: crate::proc::Layouter::default(),
            global_declarations: functions::builtin_declarations(),
            function: FunctionContext::new(FunctionKind::Kernel),
            sqrtf_function: None,
            trigonometry_function: None,
            structures: FastHashMap::default(),
            next_structure_id: 0,
            helper_builtin_globals: [None; 3],
            workgroup_size: options.workgroup_size,
        }
    }

    pub fn parse(mut self) -> Result<Module, ParseError> {
        loop {
            if self.lexer.peek()?.0 == Token::Word("struct") {
                self.structure_declaration(true)?;
            } else if self.starts_alias()? {
                self.alias_declaration(true)?;
            } else if self.starts_constexpr_declaration()? {
                self.global_constant_declaration()?;
            } else if self.function_definition()? == FunctionKind::Kernel {
                break;
            }
        }
        loop {
            if self.lexer.peek()?.0 == Token::Word("struct") {
                self.structure_declaration(true)?;
            } else if self.starts_alias()? {
                self.alias_declaration(true)?;
            } else if self.starts_constexpr_declaration()? {
                self.global_constant_declaration()?;
            } else {
                break;
            }
        }
        if self.lexer.peek()?.0 != Token::End {
            return self.fail("only one CUDA kernel is supported");
        }
        // Remove call results from source statements discarded after a jump.
        crate::compact::compact(&mut self.module, crate::compact::KeepUnused::Yes);
        Ok(self.module)
    }

    fn parameter(&mut self, parameter_index: u32) -> Result<(), ParseError> {
        let type_span = self.lexer.peek()?.1;
        let QualifiedType {
            ty: specifier,
            is_const,
        } = self.parse_type()?;
        let ty = self.lower_type(specifier, type_span);
        if self.function.kind == FunctionKind::Kernel
            && self
                .module
                .compare_types(&ty, &scalar_resolution(Scalar::BOOL))
        {
            return self.fail("boolean kernel parameters are unsupported");
        }
        let pointer_span = self.lexer.peek()?.1;
        let is_pointer = self.lexer.next_if(Token::Operation('*'))?;
        if self.function.kind == FunctionKind::Kernel
            && !is_pointer
            && !matches!(specifier, TypeSpecifier::Scalar(_))
        {
            return Err(ParseError {
                message: String::from(if matches!(specifier, TypeSpecifier::Structure(_)) {
                    "structure kernel value parameters are unsupported"
                } else {
                    "vector kernel parameters are unsupported"
                }),
                span: type_span,
            });
        }
        if self.function.kind == FunctionKind::Device && is_pointer {
            return Err(ParseError {
                message: String::from("device helper pointer parameters are unsupported"),
                span: pointer_span,
            });
        }
        let (name, span) = self.identifier()?;
        self.check_name(name, span)?;
        if self.function.kind == FunctionKind::Device {
            return self.device_parameter(name, span, &ty, is_const);
        }
        if !is_pointer {
            return self.value_parameter(parameter_index, name, span, &ty, is_const);
        }
        let base = self.type_handle(&ty, span);
        let array_ty = self.array_type(base, ArraySize::Dynamic, type_span, span)?;
        let access = if is_const {
            StorageAccess::LOAD
        } else {
            StorageAccess::LOAD | StorageAccess::STORE
        };
        let global = self.module.global_variables.append(
            GlobalVariable {
                name: Some(String::from(name)),
                space: AddressSpace::Storage { access },
                binding: Some(ResourceBinding {
                    group: 0,
                    binding: parameter_index,
                }),
                ty: array_ty,
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
                ty,
                writable: !is_const,
            },
            span,
        )?;
        Ok(())
    }

    fn value_parameter(
        &mut self,
        parameter_index: u32,
        name: &'a str,
        span: Span,
        parameter_ty: &TypeResolution,
        is_const: bool,
    ) -> Result<(), ParseError> {
        let value_ty = self.type_handle(parameter_ty, span);
        let ty = self.module.types.insert(
            Type {
                name: None,
                inner: TypeInner::Struct {
                    members: alloc::vec![StructMember {
                        name: Some(String::from("value")),
                        ty: value_ty,
                        binding: None,
                        offset: 0,
                    }],
                    span: 16,
                },
            },
            span,
        );
        let global = self.module.global_variables.append(
            GlobalVariable {
                name: Some(String::from(name)),
                space: AddressSpace::Uniform,
                binding: Some(ResourceBinding {
                    group: 0,
                    binding: parameter_index,
                }),
                ty,
                init: None,
                memory_decorations: Default::default(),
            },
            span,
        );
        let base = self.append(Expression::GlobalVariable(global), span)?;
        let field = self.append(Expression::AccessIndex { base, index: 0 }, span)?;
        let value = self.append(Expression::Load { pointer: field }, span)?;
        self.initialize_parameter(name, span, parameter_ty, is_const, value)
    }

    fn initialize_parameter(
        &mut self,
        name: &'a str,
        span: Span,
        ty: &TypeResolution,
        is_const: bool,
        value: Handle<Expression>,
    ) -> Result<(), ParseError> {
        let value_ty = self.type_handle(ty, span);
        let local = self.function.ir.local_variables.append(
            LocalVariable {
                name: Some(String::from(name)),
                ty: value_ty,
                init: None,
            },
            span,
        );
        let pointer = self.append(Expression::LocalVariable(local), span)?;
        self.push_statement(Statement::Store { pointer, value }, span);
        self.insert_symbol(
            name,
            Symbol::Variable {
                pointer,
                ty: ty.clone(),
                writable: !is_const,
            },
            span,
        )?;
        Ok(())
    }

    fn array_element(
        &mut self,
        mut base: Handle<Expression>,
        mut element_ty: TypeResolution,
        span: Span,
    ) -> Result<(Handle<Expression>, TypeResolution, bool), ParseError> {
        let (array_ty, is_pending) = match self.function.ir.expressions[base] {
            Expression::LocalVariable(local) => (
                self.function.ir.local_variables[local].ty,
                self.function.pending_array == Some(local),
            ),
            Expression::GlobalVariable(global) => (self.module.global_variables[global].ty, false),
            _ => unreachable!(),
        };
        let mut count = if is_pending {
            None
        } else {
            match self.module.types[array_ty].inner {
                TypeInner::Array {
                    size: ArraySize::Constant(count),
                    ..
                } => Some(count),
                _ => None,
            }
        };
        let mut is_discardable = self.pointer_constant(base).is_discardable;
        loop {
            self.lexer.expect_span(Token::Paren('['))?;
            let index = self.expression(0)?;
            let index = self.rvalue(index)?;
            let scalar = self.require_scalar_type(&index.ty, index.span)?;
            if !matches!(scalar.kind, ScalarKind::Sint | ScalarKind::Uint) {
                return Err(ParseError {
                    message: String::from("array index must be an integer"),
                    span: index.span,
                });
            }
            let end = self.lexer.expect_span(Token::Paren(']'))?;
            is_discardable = Self::constant_array_address(is_discardable, count, index.constant);
            base = self.append(
                Expression::Access {
                    base,
                    index: index.handle,
                },
                span.until(&end),
            )?;
            match *element_ty.inner_with(&self.module.types) {
                TypeInner::Array {
                    base: nested_element,
                    size: ArraySize::Constant(nested_count),
                    ..
                } => {
                    if self.lexer.peek()?.0 != Token::Paren('[') {
                        return self.fail("array values require complete indexing");
                    }
                    element_ty = TypeResolution::Handle(nested_element);
                    count = Some(nested_count);
                }
                _ => return Ok((base, element_ty, is_discardable)),
            }
        }
    }

    fn builtin(&mut self, name: &str, start: Span) -> Result<Value, ParseError> {
        self.lexer.expect_span(Token::Separator('.'))?;
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
            let slot = match name {
                "threadIdx" => 0,
                "blockIdx" => 1,
                _ => 2,
            };
            let base = if self.function.kind == FunctionKind::Device {
                self.helper_builtin_value(slot, span)?
            } else {
                self.builtin_input(slot, span)?
            };
            self.append(Expression::AccessIndex { base, index }, span)?
        };
        Ok(Value {
            constant: ConstantValue::default(),
            handle,
            ty: scalar_resolution(Scalar::U32),
            span,
        })
    }

    fn builtin_vector_type(&mut self, span: Span) -> Handle<Type> {
        self.module.types.insert(
            Type {
                name: None,
                inner: TypeInner::Vector {
                    size: VectorSize::Tri,
                    scalar: Scalar::U32,
                },
            },
            span,
        )
    }

    fn builtin_input(&mut self, slot: usize, span: Span) -> Result<Handle<Expression>, ParseError> {
        if let Some(argument) = self.function.builtin_arguments[slot] {
            return Ok(argument);
        }
        let (name, builtin) = BUILTIN_INPUTS[slot];
        let ty = self.builtin_vector_type(span);
        let argument_index = self.function.ir.arguments.len() as u32;
        self.function.ir.arguments.push(FunctionArgument {
            name: Some(String::from(name)),
            ty,
            binding: Some(Binding::BuiltIn(builtin)),
        });
        let argument = self.append(Expression::FunctionArgument(argument_index), span)?;
        self.function.builtin_arguments[slot] = Some(argument);
        Ok(argument)
    }

    fn helper_builtin_value(
        &mut self,
        slot: usize,
        span: Span,
    ) -> Result<Handle<Expression>, ParseError> {
        let global_handle = if let Some(global_handle) = self.helper_builtin_globals[slot] {
            global_handle
        } else {
            let ty = self.builtin_vector_type(span);
            let global_handle = self.module.global_variables.append(
                GlobalVariable {
                    name: Some(String::from(BUILTIN_INPUTS[slot].0)),
                    space: AddressSpace::Private,
                    binding: None,
                    ty,
                    init: None,
                    memory_decorations: Default::default(),
                },
                span,
            );
            self.helper_builtin_globals[slot] = Some(global_handle);
            global_handle
        };
        let pointer = self.append(Expression::GlobalVariable(global_handle), span)?;
        self.append(Expression::Load { pointer }, span)
    }

    fn initialize_helper_builtins(&mut self) -> Result<(), ParseError> {
        for (slot, global_handle) in self.helper_builtin_globals.into_iter().enumerate() {
            let Some(global_handle) = global_handle else {
                continue;
            };
            let span = self.module.global_variables.get_span(global_handle);
            let value = self.builtin_input(slot, span)?;
            let pointer = self.append(Expression::GlobalVariable(global_handle), span)?;
            self.push_statement(Statement::Store { pointer, value }, span);
        }
        Ok(())
    }

    fn number(&mut self, text: &str, span: Span) -> Result<Value, ParseError> {
        let invalid = || ParseError {
            message: String::from("unsupported or out-of-range numeric literal"),
            span,
        };
        let literal = number::parse(text).ok_or_else(invalid)?;
        let handle = self.append(Expression::Literal(literal), span)?;
        Ok(Value {
            constant: literal.into(),
            handle,
            ty: scalar_resolution(literal.scalar()),
            span,
        })
    }

    fn scalar_handle(&mut self, scalar: Scalar, span: Span) -> Handle<Type> {
        self.type_handle(&scalar_resolution(scalar), span)
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
            Expression::Compose { ref components, .. } => {
                components
                    .iter()
                    .map(|handle| self.function.expression_graph_depths[handle.index()])
                    .max()
                    .unwrap_or(0)
                    + 1
            }
            Expression::Math {
                arg,
                arg1,
                arg2,
                arg3,
                ..
            } => {
                let mut depth = self.function.expression_graph_depths[arg.index()];
                for operand in [arg1, arg2, arg3].into_iter().flatten() {
                    depth = depth.max(self.function.expression_graph_depths[operand.index()]);
                }
                depth + 1
            }
            Expression::Select {
                condition,
                accept,
                reject,
            } => {
                self.function.expression_graph_depths[condition.index()]
                    .max(self.function.expression_graph_depths[accept.index()])
                    .max(self.function.expression_graph_depths[reject.index()])
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
                | "__forceinline__"
                | "__noinline__"
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
        self.function
            .symbol_table
            .lookup(name)
            .cloned()
            .or_else(|| match self.global_declarations.get(name) {
                Some(&GlobalDeclaration::Constant {
                    handle,
                    scalar,
                    literal,
                }) => Some(Symbol::Constant {
                    handle,
                    scalar,
                    literal,
                }),
                _ => None,
            })
    }

    fn take_ident(&mut self, identifier: &str) -> Result<bool, ParseError> {
        self.lexer.next_if(Token::Word(identifier))
    }

    fn expect_ident(&mut self, identifier: &str) -> Result<(), ParseError> {
        self.lexer.expect_span(Token::Word(identifier))?;
        Ok(())
    }

    fn identifier(&mut self) -> Result<(&'a str, Span), ParseError> {
        let token = self.lexer.next()?;
        if let Token::Word(name) = token.0 {
            Ok((name, token.1))
        } else {
            Err(ParseError {
                message: String::from("expected an identifier"),
                span: token.1,
            })
        }
    }

    fn fail<T>(&mut self, message: &str) -> Result<T, ParseError> {
        Err(ParseError {
            message: String::from(message),
            span: self.lexer.peek()?.1,
        })
    }
}
