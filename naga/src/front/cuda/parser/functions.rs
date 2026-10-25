use super::*;

use super::function_declarations::Inlining;
use super::types::VECTOR_TYPES;
use crate::MathFunction;

const MATH_BUILTINS: [(&str, MathFunction); 4] = [
    ("min", MathFunction::Min),
    ("max", MathFunction::Max),
    ("fminf", MathFunction::Min),
    ("fmaxf", MathFunction::Max),
];

const MATH_SIGNATURES: [([Scalar; 2], Scalar); 5] = [
    ([Scalar::I32, Scalar::I32], Scalar::I32),
    ([Scalar::U32, Scalar::U32], Scalar::U32),
    ([Scalar::I32, Scalar::U32], Scalar::U32),
    ([Scalar::U32, Scalar::I32], Scalar::U32),
    ([Scalar::F32, Scalar::F32], Scalar::F32),
];

const MULTIPLY24_BUILTINS: [(&str, Scalar); 2] =
    [("__mul24", Scalar::I32), ("__umul24", Scalar::U32)];

const BITCAST_BUILTINS: [(&str, Scalar, Scalar); 4] = [
    ("__float_as_int", Scalar::F32, Scalar::I32),
    ("__float_as_uint", Scalar::F32, Scalar::U32),
    ("__int_as_float", Scalar::I32, Scalar::F32),
    ("__uint_as_float", Scalar::U32, Scalar::F32),
];

const UNARY_BUILTINS: [(&str, MathFunction, Scalar, Scalar); 9] = [
    (
        "__clz",
        MathFunction::CountLeadingZeros,
        Scalar::I32,
        Scalar::I32,
    ),
    (
        "__ffs",
        MathFunction::FirstTrailingBit,
        Scalar::I32,
        Scalar::I32,
    ),
    (
        "__popc",
        MathFunction::CountOneBits,
        Scalar::U32,
        Scalar::I32,
    ),
    (
        "__brev",
        MathFunction::ReverseBits,
        Scalar::U32,
        Scalar::U32,
    ),
    ("floorf", MathFunction::Floor, Scalar::F32, Scalar::F32),
    ("fabsf", MathFunction::Abs, Scalar::F32, Scalar::F32),
    ("fabs", MathFunction::Abs, Scalar::F32, Scalar::F32),
    ("sinf", MathFunction::Sin, Scalar::F32, Scalar::F32),
    ("cosf", MathFunction::Cos, Scalar::F32, Scalar::F32),
];

#[derive(Clone, Copy, PartialEq)]
struct MathBuiltin {
    function: MathFunction,
    parameters: [Scalar; 2],
    result: Scalar,
}

fn math_builtins(name: &str) -> impl Iterator<Item = MathBuiltin> + Clone {
    let function = MATH_BUILTINS
        .into_iter()
        .find_map(|(candidate, function)| (candidate == name).then_some(function));
    let signatures = if matches!(name, "fminf" | "fmaxf") {
        &MATH_SIGNATURES[4..]
    } else {
        &MATH_SIGNATURES[..]
    };
    signatures
        .iter()
        .copied()
        .filter_map(move |(parameters, result)| {
            function.map(|function| MathBuiltin {
                function,
                parameters,
                result,
            })
        })
}

fn vector_constructor(name: &str) -> Option<(VectorSize, Scalar)> {
    VECTOR_TYPES
        .into_iter()
        .find_map(|(_, constructor, size, scalar)| (name == constructor).then_some((size, scalar)))
}

pub(super) fn builtin_declarations<'a>() -> FastHashMap<&'a str, GlobalDeclaration> {
    MATH_BUILTINS
        .into_iter()
        .map(|(name, _)| (name, GlobalDeclaration::Functions(Vec::new())))
        .chain(
            MULTIPLY24_BUILTINS
                .into_iter()
                .map(|(name, _)| (name, GlobalDeclaration::Functions(Vec::new()))),
        )
        .chain(
            BITCAST_BUILTINS
                .into_iter()
                .map(|(name, _, _)| (name, GlobalDeclaration::Functions(Vec::new()))),
        )
        .chain(
            UNARY_BUILTINS
                .into_iter()
                .map(|(name, _, _, _)| (name, GlobalDeclaration::Functions(Vec::new()))),
        )
        .chain(core::iter::once((
            "sqrtf",
            GlobalDeclaration::Functions(Vec::new()),
        )))
        .chain(
            VECTOR_TYPES
                .into_iter()
                .flat_map(|(name, constructor, size, scalar)| {
                    [
                        (
                            name,
                            GlobalDeclaration::Type(QualifiedType {
                                ty: TypeSpecifier::Vector { size, scalar },
                                is_const: false,
                            }),
                        ),
                        (constructor, GlobalDeclaration::Functions(Vec::new())),
                    ]
                }),
        )
        .collect()
}

#[derive(Clone)]
enum ReturnType {
    Void,
    Value(TypeSpecifier),
}

struct FunctionHeader<'a> {
    kind: FunctionKind,
    result: ReturnType,
    result_span: Span,
    name: &'a str,
    name_span: Span,
    has_host: bool,
    is_static: bool,
    inlining: Option<Inlining>,
    operator: Option<operators::Operator>,
}

#[derive(Clone, Copy, PartialEq)]
enum FunctionCandidate {
    Defined(Handle<Function>),
    Current,
    Builtin(MathBuiltin),
    Multiply24(Scalar),
    Bitcast {
        source: Scalar,
        result: Scalar,
    },
    Unary {
        function: MathFunction,
        source: Scalar,
        result: Scalar,
    },
    IntegralFabs(Scalar),
    Sqrt,
    Constructor {
        size: VectorSize,
        scalar: Scalar,
    },
}

fn conversion_rank(
    source: &TypeResolution,
    target: &TypeResolution,
    module: &Module,
) -> Option<u8> {
    if module.compare_types(source, target) {
        return Some(0);
    }
    match (
        source.inner_with(&module.types),
        target.inner_with(&module.types),
    ) {
        (&TypeInner::Scalar(source), &TypeInner::Scalar(target)) => {
            Some(if source == Scalar::BOOL && target == Scalar::I32 {
                1
            } else {
                2
            })
        }
        _ => None,
    }
}

impl<'a> Parser<'a> {
    pub(super) fn parameters(&mut self) -> Result<(), ParseError> {
        self.lexer.expect_span(Token::Paren('('))?;
        if !self.lexer.next_if(Token::Paren(')'))? {
            if self.take_ident("void")? {
                self.lexer.expect_span(Token::Paren(')'))?;
            } else {
                let mut parameter_index = 0u32;
                loop {
                    self.parameter(parameter_index)?;
                    if self.lexer.next_if(Token::Paren(')'))? {
                        break;
                    }
                    let separator_span = self.lexer.expect_span(Token::Separator(','))?;
                    parameter_index = parameter_index.checked_add(1).ok_or_else(|| ParseError {
                        message: String::from("too many CUDA function parameters"),
                        span: separator_span,
                    })?;
                }
            }
        }
        Ok(())
    }

    fn check_function_name(
        &self,
        name: &str,
        span: Span,
        kind: FunctionKind,
    ) -> Result<(), ParseError> {
        self.check_name(name, span)?;
        let compatible = match self.global_declarations.get(name) {
            None => true,
            Some(&GlobalDeclaration::Functions(_)) => kind == FunctionKind::Device,
            Some(_) => false,
        };
        if !compatible {
            return Err(ParseError {
                message: format!("incompatible function declaration `{name}`"),
                span,
            });
        }
        Ok(())
    }

    fn function_header(&mut self, extern_c: bool) -> Result<FunctionHeader<'a>, ParseError> {
        let mut kind = None;
        let mut host_span = None;
        let mut result = None;
        let mut result_span = None;
        let mut inlining = None;
        let mut seen_static = false;
        let mut cuda_inline_span = None;
        let mut explicit_const = false;
        let mut seen_int = false;
        let mut seen_unsigned = false;
        loop {
            let token = self.lexer.peek()?;
            match token.0 {
                Token::Word("inline") => {
                    if inlining == Some(Inlining::Inline) {
                        return self.fail("duplicate inline specifier");
                    }
                    if cuda_inline_span.is_some() {
                        return self.fail("incompatible inlining specifiers");
                    }
                    inlining = Some(Inlining::Inline);
                }
                Token::Word("static") => {
                    if extern_c {
                        return self.fail("static conflicts with an extern C declaration");
                    }
                    if seen_static {
                        return self.fail("duplicate static specifier");
                    }
                    seen_static = true;
                }
                Token::Word("__forceinline__" | "__noinline__") => {
                    if inlining.is_some() {
                        return self.fail("duplicate or incompatible inlining specifiers");
                    }
                    cuda_inline_span = Some(token.1);
                    inlining = Some(if token.0 == Token::Word("__forceinline__") {
                        Inlining::ForceInline
                    } else {
                        Inlining::NoInline
                    });
                }
                Token::Word("__host__") => {
                    if host_span.is_some() || kind == Some(FunctionKind::Kernel) {
                        return self
                            .fail("duplicate or incompatible CUDA execution space specifier");
                    }
                    host_span = Some(token.1);
                }
                Token::Word("__device__" | "__global__") => {
                    if kind.is_some()
                        || (host_span.is_some() && token.0 == Token::Word("__global__"))
                    {
                        return self
                            .fail("duplicate or incompatible CUDA execution space specifier");
                    }
                    kind = Some(if token.0 == Token::Word("__device__") {
                        FunctionKind::Device
                    } else {
                        FunctionKind::Kernel
                    });
                }
                Token::Word("const") => {
                    if explicit_const {
                        return self.fail("duplicate const qualifier");
                    }
                    explicit_const = true;
                    result_span.get_or_insert(token.1);
                }
                Token::Word("int") => {
                    if seen_int || (result.is_some() && !seen_unsigned) {
                        return self.fail("incompatible or repeated return type specifier");
                    }
                    seen_int = true;
                    result = Some(ReturnType::Value(TypeSpecifier::Scalar(if seen_unsigned {
                        Scalar::U32
                    } else {
                        Scalar::I32
                    })));
                    result_span.get_or_insert(token.1);
                }
                Token::Word("unsigned") => {
                    if seen_unsigned || (result.is_some() && !seen_int) {
                        return self.fail("incompatible or repeated return type specifier");
                    }
                    seen_unsigned = true;
                    result = Some(ReturnType::Value(TypeSpecifier::Scalar(Scalar::U32)));
                    result_span.get_or_insert(token.1);
                }
                Token::Word("void" | "float" | "bool") => {
                    if result.is_some() {
                        return self.fail("incompatible or repeated return type specifier");
                    }
                    result = Some(match token.0 {
                        Token::Word("void") => ReturnType::Void,
                        Token::Word("float") => {
                            ReturnType::Value(TypeSpecifier::Scalar(Scalar::F32))
                        }
                        _ => ReturnType::Value(TypeSpecifier::Scalar(Scalar::BOOL)),
                    });
                    result_span.get_or_insert(token.1);
                }
                Token::Word(name) if result.is_none() => {
                    let Some(ty) = self.type_name(name) else {
                        break;
                    };
                    result = Some(ReturnType::Value(ty.ty));
                    result_span.get_or_insert(token.1);
                }
                _ => break,
            }
            self.lexer.next()?;
        }
        if let (Some(span), None) = (host_span, kind) {
            return Err(ParseError {
                message: String::from("host-only functions are unsupported"),
                span,
            });
        }
        let Some(result) = result else {
            return self.fail("expected a supported function return type");
        };
        let Some(kind) = kind else {
            return self.fail("expected a CUDA execution space specifier");
        };
        if let (FunctionKind::Kernel, Some(span)) = (kind, cuda_inline_span) {
            return Err(ParseError {
                message: String::from("CUDA inlining specifiers are unsupported for kernels"),
                span,
            });
        }
        let token = self.lexer.peek()?;
        let (name, name_span, operator) = if self.take_ident("operator")? {
            let (operator, span) = self.operator_name(token.1)?;
            (operator.name(), span, Some(operator))
        } else {
            let (name, span) = self.identifier()?;
            (name, span, None)
        };
        if operator.is_some() && kind != FunctionKind::Device {
            return Err(ParseError {
                message: String::from("operator functions must be device helpers"),
                span: name_span,
            });
        }
        self.check_function_name(name, name_span, kind)?;
        Ok(FunctionHeader {
            kind,
            result,
            result_span: result_span.unwrap(),
            name,
            name_span,
            has_host: host_span.is_some(),
            is_static: seen_static,
            inlining,
            operator,
        })
    }

    pub(super) fn function_definition(&mut self) -> Result<FunctionKind, ParseError> {
        let extern_c = self.take_ident("extern")?;
        if extern_c {
            if self.lexer.peek()?.0 != Token::String("C") {
                return self.fail("expected `\"C\"` after `extern`");
            }
            self.lexer.next()?;
        }
        let header = self.function_header(extern_c)?;
        self.function = FunctionContext::new(header.kind);
        match header.kind {
            FunctionKind::Device => {
                if extern_c {
                    return Err(ParseError {
                        message: String::from("extern C device helpers are unsupported"),
                        span: header.name_span,
                    });
                }
                self.device_function(header)?;
                Ok(FunctionKind::Device)
            }
            FunctionKind::Kernel => {
                self.kernel(header)?;
                Ok(FunctionKind::Kernel)
            }
        }
    }

    fn kernel(&mut self, header: FunctionHeader<'a>) -> Result<(), ParseError> {
        if self.kernel_context.is_some() {
            return Err(ParseError {
                message: String::from("only one CUDA kernel is supported"),
                span: header.name_span,
            });
        }
        if !matches!(header.result, ReturnType::Void) {
            return Err(ParseError {
                message: String::from("CUDA kernels must return void"),
                span: header.result_span,
            });
        }
        self.parameters()?;
        self.lexer.expect_span(Token::Paren('{'))?;
        self.initialize_helper_builtins()?;
        self.block_contents()?;
        self.flush_emitter();
        barriers::check_control_flow(&self.function.ir.body, false, false)?;
        let function = core::mem::replace(
            &mut self.function,
            FunctionContext::new(FunctionKind::Kernel),
        );
        self.module.entry_points.push(EntryPoint {
            name: String::from(header.name),
            stage: ShaderStage::Compute,
            early_depth_test: None,
            workgroup_size: self.workgroup_size,
            workgroup_size_overrides: None,
            function: Function::default(),
            mesh_info: None,
            task_payload: None,
            incoming_ray_payload: None,
        });
        self.kernel_context = Some(function);
        self.global_declarations
            .insert(header.name, GlobalDeclaration::Kernel);
        Ok(())
    }

    fn device_function(&mut self, header: FunctionHeader<'a>) -> Result<(), ParseError> {
        self.function.ir.name = Some(String::from(header.name));
        if let ReturnType::Value(ty) = header.result {
            let ty = self.lower_type(ty, header.result_span);
            self.function.ir.result = Some(crate::FunctionResult {
                ty: self.type_handle(&ty, header.result_span),
                binding: None,
            });
        }
        self.parameters()?;
        if let Some(operator) = header.operator {
            self.check_operator_parameters(operator, header.name_span)?;
        }
        let is_prototype = self.lexer.peek()?.0 == Token::Separator(';');
        let intrinsic = self.function_candidates(header.name).find(|&candidate| {
            matches!(
                candidate,
                FunctionCandidate::Builtin(_)
                    | FunctionCandidate::Multiply24(_)
                    | FunctionCandidate::Bitcast { .. }
                    | FunctionCandidate::Unary { .. }
                    | FunctionCandidate::Sqrt
                    | FunctionCandidate::Constructor { .. }
            ) && self.function.ir.arguments.len() == self.candidate_arity(candidate)
                && (0..self.candidate_arity(candidate)).all(|index| {
                    self.module.compare_types(
                        &self.candidate_type(FunctionCandidate::Current, index),
                        &self.candidate_type(candidate, index),
                    )
                })
        });
        if let Some(candidate) = intrinsic {
            if !is_prototype {
                return Err(ParseError {
                    message: format!("duplicate builtin definition `{}`", header.name),
                    span: header.name_span,
                });
            }
            let result = self.candidate_result(candidate, header.result_span);
            if self.function.ir.result.as_ref().map(|result| result.ty) != result {
                return Err(ParseError {
                    message: format!("incompatible builtin result type `{}`", header.name),
                    span: header.name_span,
                });
            }
            let parameters: Vec<_> = self
                .function
                .ir
                .arguments
                .iter()
                .map(|argument| argument.ty)
                .collect();
            if let Some(previous) = self
                .intrinsic_declarations
                .iter_mut()
                .find(|previous| previous.name == header.name && previous.parameters == parameters)
            {
                function_declarations::check_redeclaration(
                    previous.has_host,
                    previous.is_static,
                    previous.inlining,
                    header.has_host,
                    header.is_static,
                    header.inlining,
                    header.name_span,
                )?;
                previous.inlining = previous.inlining.or(header.inlining);
            } else {
                self.intrinsic_declarations.push(IntrinsicDeclaration {
                    name: header.name,
                    parameters,
                    has_host: header.has_host,
                    is_static: header.is_static,
                    inlining: header.inlining,
                });
            }
            self.lexer.expect_span(Token::Separator(';'))?;
            self.function = FunctionContext::new(FunctionKind::Kernel);
            return Ok(());
        }
        let previous = match self.global_declarations.get(header.name) {
            Some(&GlobalDeclaration::Functions(ref functions)) => {
                functions.iter().copied().find(|&function| {
                    self.module.functions[function]
                        .arguments
                        .iter()
                        .map(|argument| argument.ty)
                        .eq(self
                            .function
                            .ir
                            .arguments
                            .iter()
                            .map(|argument| argument.ty))
                })
            }
            _ => None,
        };
        if let Some(previous) = previous {
            let declaration = self.function_declarations.get_mut(&previous).unwrap();
            function_declarations::check_redeclaration(
                declaration.has_host,
                declaration.is_static,
                declaration.inlining,
                header.has_host,
                header.is_static,
                header.inlining,
                header.name_span,
            )?;
            if !is_prototype && declaration.is_defined {
                return Err(ParseError {
                    message: format!("duplicate device helper definition `{}`", header.name),
                    span: header.name_span,
                });
            }
            if self.module.functions[previous]
                .result
                .as_ref()
                .map(|result| result.ty)
                != self.function.ir.result.as_ref().map(|result| result.ty)
            {
                return Err(ParseError {
                    message: format!("incompatible device helper result type `{}`", header.name),
                    span: header.name_span,
                });
            }
            declaration.inlining = declaration.inlining.or(header.inlining);
        }
        if is_prototype {
            if previous.is_none() {
                let function = Function {
                    name: self.function.ir.name.clone(),
                    arguments: self.function.ir.arguments.clone(),
                    result: self.function.ir.result.clone(),
                    ..Function::default()
                };
                let handle = self.module.functions.append(function, header.name_span);
                self.register_function(header.name, handle);
                self.function_declarations.insert(
                    handle,
                    FunctionDeclaration {
                        has_host: header.has_host,
                        is_static: header.is_static,
                        inlining: header.inlining,
                        is_defined: false,
                        calls: Vec::new(),
                    },
                );
            }
            self.lexer.expect_span(Token::Separator(';'))?;
            self.function = FunctionContext::new(FunctionKind::Kernel);
            return Ok(());
        }
        self.function.current_declaration = previous;
        self.lexer.expect_span(Token::Paren('{'))?;
        let terminated = self.block_contents()?;
        if self.function.ir.result.is_some() && !terminated {
            return Err(ParseError {
                message: String::from("device helper must return a value on every supported path"),
                span: header.name_span,
            });
        }
        self.flush_emitter();
        if self.function.ir.result.is_none() {
            crate::proc::ensure_block_returns(&mut self.function.ir.body);
        }
        let function = core::mem::replace(
            &mut self.function,
            FunctionContext::new(FunctionKind::Kernel),
        );
        if let Some(previous) = previous {
            self.module.functions[previous] = function.ir;
            let declaration = self.function_declarations.get_mut(&previous).unwrap();
            declaration.is_defined = true;
            declaration.calls = function.calls;
        } else {
            let handle = self.module.functions.append(function.ir, header.name_span);
            self.register_function(header.name, handle);
            self.function_declarations.insert(
                handle,
                FunctionDeclaration {
                    has_host: header.has_host,
                    is_static: header.is_static,
                    inlining: header.inlining,
                    is_defined: true,
                    calls: function.calls,
                },
            );
        }
        Ok(())
    }

    fn register_function(&mut self, name: &'a str, handle: Handle<Function>) {
        match *self
            .global_declarations
            .entry(name)
            .or_insert_with(|| GlobalDeclaration::Functions(Vec::new()))
        {
            GlobalDeclaration::Functions(ref mut functions) => functions.push(handle),
            _ => unreachable!(),
        }
    }

    fn candidate_result(
        &mut self,
        candidate: FunctionCandidate,
        span: Span,
    ) -> Option<Handle<Type>> {
        let ty = match candidate {
            FunctionCandidate::Defined(function) => {
                return self.module.functions[function]
                    .result
                    .as_ref()
                    .map(|result| result.ty)
            }
            FunctionCandidate::Current => {
                return self.function.ir.result.as_ref().map(|result| result.ty)
            }
            FunctionCandidate::Builtin(builtin) => TypeSpecifier::Scalar(builtin.result),
            FunctionCandidate::Multiply24(result)
            | FunctionCandidate::Bitcast { result, .. }
            | FunctionCandidate::Unary { result, .. } => TypeSpecifier::Scalar(result),
            FunctionCandidate::IntegralFabs(_) => unreachable!(),
            FunctionCandidate::Sqrt => TypeSpecifier::Scalar(Scalar::F32),
            FunctionCandidate::Constructor { size, scalar } => {
                TypeSpecifier::Vector { size, scalar }
            }
        };
        let ty = self.lower_type(ty, span);
        Some(self.type_handle(&ty, span))
    }

    pub(super) fn device_parameter(
        &mut self,
        name: &'a str,
        span: Span,
        parameter_ty: &TypeResolution,
        is_const: bool,
    ) -> Result<(), ParseError> {
        let ty = self.type_handle(parameter_ty, span);
        let index = self.function.ir.arguments.len() as u32;
        self.function.ir.arguments.push(FunctionArgument {
            name: Some(String::from(name)),
            ty,
            binding: None,
        });
        let value = self.append(Expression::FunctionArgument(index), span)?;
        self.initialize_parameter(name, span, parameter_ty, is_const, value)
    }

    pub(super) fn call(&mut self, name: &str, span: Span) -> Result<ExpressionResult, ParseError> {
        if self.lookup_symbol(name).is_some() {
            return Err(ParseError {
                message: format!("local value `{name}` cannot be called"),
                span,
            });
        }
        if self.function_candidates(name).next().is_none() {
            return Err(ParseError {
                message: format!("unknown or not previously declared device helper `{name}`"),
                span,
            });
        }
        self.lexer.expect_span(Token::Paren('('))?;
        let mut arguments = Vec::new();
        if self.lexer.peek()?.0 != Token::Paren(')') {
            loop {
                let value = self.expression(0)?;
                arguments.push(self.rvalue(value)?);
                if self.lexer.peek()?.0 == Token::Paren(')') {
                    break;
                }
                self.lexer.expect_span(Token::Separator(','))?;
            }
        }
        let end = self.lexer.expect_span(Token::Paren(')'))?;
        self.call_with_arguments(name, arguments, span, span.until(&end))
    }

    pub(super) fn call_with_arguments(
        &mut self,
        name: &str,
        arguments: Vec<Value>,
        name_span: Span,
        span: Span,
    ) -> Result<ExpressionResult, ParseError> {
        let candidate = self.resolve_overload(name, &arguments, name_span)?;
        self.lower_call(candidate, arguments, span)
    }

    fn lower_call(
        &mut self,
        candidate: FunctionCandidate,
        arguments: Vec<Value>,
        span: Span,
    ) -> Result<ExpressionResult, ParseError> {
        if let FunctionCandidate::Constructor { size, scalar } = candidate {
            let components = arguments
                .into_iter()
                .map(|value| {
                    self.convert_scalar(value.into(), scalar)
                        .map(|value| value.handle)
                })
                .collect::<Result<Vec<_>, _>>()?;
            let ty = self.lower_type(TypeSpecifier::Vector { size, scalar }, span);
            let handle_ty = self.type_handle(&ty, span);
            let handle = self.append(
                Expression::Compose {
                    ty: handle_ty,
                    components,
                },
                span,
            )?;
            return Ok(Value {
                constant: ConstantValue::default(),
                handle,
                ty,
                span,
            }
            .into());
        }
        if let FunctionCandidate::Bitcast { source, result } = candidate {
            let argument =
                self.convert_scalar(arguments.into_iter().next().unwrap().into(), source)?;
            let mut expr = argument.handle;
            if result == Scalar::F32 {
                // Keep non-finite results out of WGSL constant evaluation.
                let pointer = self.temporary_pointer(&argument.ty, span)?;
                self.push_statement(
                    Statement::Store {
                        pointer,
                        value: expr,
                    },
                    span,
                );
                expr = self.append(Expression::Load { pointer }, span)?;
            }
            let handle = self.append(
                Expression::As {
                    expr,
                    kind: result.kind,
                    convert: None,
                },
                span,
            )?;
            return Ok(Value {
                constant: ConstantValue::default(),
                handle,
                ty: scalar_resolution(result),
                span,
            }
            .into());
        }
        if candidate == FunctionCandidate::Sqrt {
            let argument =
                self.convert_scalar(arguments.into_iter().next().unwrap().into(), Scalar::F32)?;
            let bits = self.append(
                Expression::As {
                    expr: argument.handle,
                    kind: ScalarKind::Uint,
                    convert: None,
                },
                span,
            )?;
            let function = self.sqrtf_function(span)?;
            let result = self.append_call_result(function, span)?;
            self.push_statement(
                Statement::Call {
                    function,
                    arguments: alloc::vec![bits],
                    result: Some(result),
                },
                span,
            );
            let handle = self.append(
                Expression::As {
                    expr: result,
                    kind: ScalarKind::Float,
                    convert: None,
                },
                span,
            )?;
            return Ok(Value {
                constant: ConstantValue::default(),
                handle,
                ty: scalar_resolution(Scalar::F32),
                span,
            }
            .into());
        }
        if let FunctionCandidate::Unary {
            function,
            source,
            result,
        } = candidate
        {
            let argument =
                self.convert_scalar(arguments.into_iter().next().unwrap().into(), source)?;
            let handle = match function {
                MathFunction::Floor => self.floorf(argument.handle, span)?,
                MathFunction::Abs => self.absolute_value(argument.handle, span)?,
                MathFunction::Sin | MathFunction::Cos => {
                    self.trigonometry(argument.handle, function == MathFunction::Cos, span)?
                }
                _ => self.append(
                    Expression::Math {
                        fun: function,
                        arg: argument.handle,
                        arg1: None,
                        arg2: None,
                        arg3: None,
                    },
                    span,
                )?,
            };
            let mut value = Value {
                constant: ConstantValue::default(),
                handle,
                ty: scalar_resolution(source),
                span,
            };
            if function == MathFunction::FirstTrailingBit {
                // CUDA numbers set bits from one and returns zero for no set bits.
                let one = self.append(Expression::Literal(Literal::I32(1)), span)?;
                value.handle = self.append(
                    Expression::Binary {
                        op: BinaryOperator::Add,
                        left: value.handle,
                        right: one,
                    },
                    span,
                )?;
            }
            return Ok(self.convert_scalar(value.into(), result)?.into());
        }
        if let FunctionCandidate::Multiply24(scalar) = candidate {
            let mut arguments = arguments.into_iter();
            let left = self.convert_scalar(arguments.next().unwrap().into(), scalar)?;
            let right = self.convert_scalar(arguments.next().unwrap().into(), scalar)?;
            let offset = self.append(Expression::Literal(Literal::U32(0)), span)?;
            let count = self.append(Expression::Literal(Literal::U32(24)), span)?;
            let mut operands = [left.handle, right.handle];
            for operand in &mut operands {
                let extracted = self.append(
                    Expression::Math {
                        fun: MathFunction::ExtractBits,
                        arg: *operand,
                        arg1: Some(offset),
                        arg2: Some(count),
                        arg3: None,
                    },
                    span,
                )?;
                *operand = if scalar == Scalar::I32 {
                    self.append(
                        Expression::As {
                            expr: extracted,
                            kind: ScalarKind::Uint,
                            convert: None,
                        },
                        span,
                    )?
                } else {
                    extracted
                };
            }
            let [left, right] = operands;
            let product = self.append(
                Expression::Binary {
                    op: BinaryOperator::Multiply,
                    left,
                    right,
                },
                span,
            )?;
            let handle = if scalar == Scalar::I32 {
                self.append(
                    Expression::As {
                        expr: product,
                        kind: ScalarKind::Sint,
                        convert: None,
                    },
                    span,
                )?
            } else {
                product
            };
            return Ok(Value {
                constant: ConstantValue::default(),
                handle,
                ty: scalar_resolution(scalar),
                span,
            }
            .into());
        }
        if let FunctionCandidate::Builtin(builtin) = candidate {
            let mut arguments = arguments.into_iter();
            let left = self.convert_scalar(arguments.next().unwrap().into(), builtin.result)?;
            let right = self.convert_scalar(arguments.next().unwrap().into(), builtin.result)?;
            let handle = if builtin.result == Scalar::F32 {
                self.float_minmax(builtin.function, left.handle, right.handle, span)?
            } else {
                self.append(
                    Expression::Math {
                        fun: builtin.function,
                        arg: left.handle,
                        arg1: Some(right.handle),
                        arg2: None,
                        arg3: None,
                    },
                    span,
                )?
            };
            return Ok(Value {
                constant: ConstantValue::default(),
                handle,
                ty: scalar_resolution(builtin.result),
                span,
            }
            .into());
        }
        let FunctionCandidate::Defined(function) = candidate else {
            unreachable!()
        };
        self.function.calls.push(FunctionCall { function, span });
        let mut converted = Vec::with_capacity(arguments.len());
        for (index, value) in arguments.into_iter().enumerate() {
            let parameter = &self.module.functions[function].arguments[index];
            let ty = TypeResolution::Handle(parameter.ty);
            converted.push(self.convert_value(value.into(), &ty)?.handle);
        }
        let ty = self.module.functions[function]
            .result
            .as_ref()
            .map(|result| TypeResolution::Handle(result.ty));
        let value = if let Some(ty) = ty {
            let handle = self.append_call_result(function, span)?;
            Some(Value {
                constant: ConstantValue::default(),
                handle,
                ty,
                span,
            })
        } else {
            None
        };
        self.push_statement(
            Statement::Call {
                function,
                arguments: converted,
                result: value.as_ref().map(|value| value.handle),
            },
            span,
        );
        Ok(match value {
            Some(value) => value.into(),
            None => ExpressionResult::Void(span),
        })
    }

    fn float_minmax(
        &mut self,
        function: MathFunction,
        left: Handle<Expression>,
        right: Handle<Expression>,
        span: Span,
    ) -> Result<Handle<Expression>, ParseError> {
        let sign_bit = self.append(Expression::Literal(Literal::U32(0x8000_0000)), span)?;
        let magnitude_mask = self.append(Expression::Literal(Literal::U32(0x7fff_ffff)), span)?;
        let infinity_bits = self.append(Expression::Literal(Literal::U32(0x7f80_0000)), span)?;
        let canonical_nan_bits =
            self.append(Expression::Literal(Literal::U32(0x7fc0_0000)), span)?;
        let operands = [left, right].map(|value| {
            let bits = self.append(
                Expression::As {
                    expr: value,
                    kind: ScalarKind::Uint,
                    convert: None,
                },
                span,
            )?;
            let is_negative = self.append(
                Expression::Binary {
                    op: BinaryOperator::GreaterEqual,
                    left: bits,
                    right: sign_bit,
                },
                span,
            )?;
            let reversed_bits = self.append(
                Expression::Unary {
                    op: UnaryOperator::BitwiseNot,
                    expr: bits,
                },
                span,
            )?;
            let positive_key = self.append(
                Expression::Binary {
                    op: BinaryOperator::ExclusiveOr,
                    left: bits,
                    right: sign_bit,
                },
                span,
            )?;
            let ordering_key = self.append(
                Expression::Select {
                    condition: is_negative,
                    accept: reversed_bits,
                    reject: positive_key,
                },
                span,
            )?;
            let absolute_bits = self.append(
                Expression::Binary {
                    op: BinaryOperator::And,
                    left: bits,
                    right: magnitude_mask,
                },
                span,
            )?;
            let is_nan = self.append(
                Expression::Binary {
                    op: BinaryOperator::Greater,
                    left: absolute_bits,
                    right: infinity_bits,
                },
                span,
            )?;
            Ok::<_, ParseError>((bits, ordering_key, is_nan))
        });
        let [left, right] = operands;
        let (left_bits, left_key, left_is_nan) = left?;
        let (right_bits, right_key, right_is_nan) = right?;
        let choose_left = self.append(
            Expression::Binary {
                op: if function == MathFunction::Min {
                    BinaryOperator::Less
                } else {
                    BinaryOperator::Greater
                },
                left: left_key,
                right: right_key,
            },
            span,
        )?;
        let selected_bits = self.append(
            Expression::Select {
                condition: choose_left,
                accept: left_bits,
                reject: right_bits,
            },
            span,
        )?;
        let selected_bits = self.append(
            Expression::Select {
                condition: left_is_nan,
                accept: right_bits,
                reject: selected_bits,
            },
            span,
        )?;
        let selected_bits = self.append(
            Expression::Select {
                condition: right_is_nan,
                accept: left_bits,
                reject: selected_bits,
            },
            span,
        )?;
        let both_are_nan = self.append(
            Expression::Binary {
                op: BinaryOperator::LogicalAnd,
                left: left_is_nan,
                right: right_is_nan,
            },
            span,
        )?;
        let bits = self.append(
            Expression::Select {
                condition: both_are_nan,
                accept: canonical_nan_bits,
                reject: selected_bits,
            },
            span,
        )?;
        self.append(
            Expression::As {
                expr: bits,
                kind: ScalarKind::Float,
                convert: None,
            },
            span,
        )
    }

    pub(super) fn has_function_candidate(&self, name: &str) -> bool {
        self.function_candidates(name).next().is_some()
    }

    fn function_candidates<'b>(
        &'b self,
        name: &str,
    ) -> impl Iterator<Item = FunctionCandidate> + Clone + 'b {
        let functions = match self.global_declarations.get(name) {
            Some(&GlobalDeclaration::Functions(ref functions)) => functions.as_slice(),
            _ => &[],
        };
        let current = (self.function.kind == FunctionKind::Device
            && self.function.ir.name.as_deref() == Some(name))
        .then_some(FunctionCandidate::Current);
        let builtin_visible = matches!(
            self.global_declarations.get(name),
            Some(&GlobalDeclaration::Functions(_))
        );
        functions
            .iter()
            .copied()
            .filter(move |&function| self.function.current_declaration != Some(function))
            .map(FunctionCandidate::Defined)
            .chain(current)
            .chain(
                math_builtins(name)
                    .filter(move |_| builtin_visible)
                    .map(FunctionCandidate::Builtin),
            )
            .chain(
                MULTIPLY24_BUILTINS
                    .into_iter()
                    .find(|&(candidate, _)| candidate == name)
                    .filter(move |_| builtin_visible)
                    .map(|(_, scalar)| FunctionCandidate::Multiply24(scalar)),
            )
            .chain(
                BITCAST_BUILTINS
                    .into_iter()
                    .find(|&(candidate, _, _)| candidate == name)
                    .filter(move |_| builtin_visible)
                    .map(|(_, source, result)| FunctionCandidate::Bitcast { source, result }),
            )
            .chain(
                UNARY_BUILTINS
                    .into_iter()
                    .find(|&(candidate, _, _, _)| candidate == name)
                    .filter(move |_| builtin_visible)
                    .map(|(_, function, source, result)| FunctionCandidate::Unary {
                        function,
                        source,
                        result,
                    }),
            )
            .chain((builtin_visible && name == "sqrtf").then_some(FunctionCandidate::Sqrt))
            .chain(
                vector_constructor(name)
                    .filter(move |_| builtin_visible)
                    .map(|(size, scalar)| FunctionCandidate::Constructor { size, scalar }),
            )
    }

    fn candidate_arity(&self, candidate: FunctionCandidate) -> usize {
        match candidate {
            FunctionCandidate::Defined(handle) => self.module.functions[handle].arguments.len(),
            FunctionCandidate::Current => self.function.ir.arguments.len(),
            FunctionCandidate::Builtin(builtin) => builtin.parameters.len(),
            FunctionCandidate::Multiply24(_) => 2,
            FunctionCandidate::Bitcast { .. }
            | FunctionCandidate::Unary { .. }
            | FunctionCandidate::IntegralFabs(_)
            | FunctionCandidate::Sqrt => 1,
            FunctionCandidate::Constructor { size, .. } => size as usize,
        }
    }

    fn candidate_type(&self, candidate: FunctionCandidate, index: usize) -> TypeResolution {
        let argument = match candidate {
            FunctionCandidate::Defined(handle) => &self.module.functions[handle].arguments[index],
            FunctionCandidate::Current => &self.function.ir.arguments[index],
            FunctionCandidate::Builtin(builtin) => {
                return scalar_resolution(builtin.parameters[index])
            }
            FunctionCandidate::Bitcast { source, .. }
            | FunctionCandidate::Unary { source, .. }
            | FunctionCandidate::Multiply24(source)
            | FunctionCandidate::IntegralFabs(source) => return scalar_resolution(source),
            FunctionCandidate::Sqrt => return scalar_resolution(Scalar::F32),
            FunctionCandidate::Constructor { scalar, .. } => return scalar_resolution(scalar),
        };
        TypeResolution::Handle(argument.ty)
    }

    fn better_candidate(
        &self,
        left_candidate: FunctionCandidate,
        right_candidate: FunctionCandidate,
        arguments: &[Value],
    ) -> bool {
        let mut has_better_conversion = false;
        for (index, argument) in arguments.iter().enumerate() {
            let left_rank = conversion_rank(
                &argument.ty,
                &self.candidate_type(left_candidate, index),
                &self.module,
            );
            let right_rank = conversion_rank(
                &argument.ty,
                &self.candidate_type(right_candidate, index),
                &self.module,
            );
            let (Some(left_rank), Some(right_rank)) = (left_rank, right_rank) else {
                return false;
            };
            if left_rank > right_rank {
                return false;
            }
            has_better_conversion |= left_rank < right_rank;
        }
        has_better_conversion
    }

    fn resolve_overload(
        &self,
        name: &str,
        arguments: &[Value],
        span: Span,
    ) -> Result<FunctionCandidate, ParseError> {
        let integral_fabs = self.integral_fabs_candidate(name, arguments);
        let mut viable_candidates =
            self.function_candidates(name)
                .chain(integral_fabs)
                .filter(|&candidate| {
                    self.candidate_arity(candidate) == arguments.len()
                        && arguments.iter().enumerate().all(|(index, argument)| {
                            conversion_rank(
                                &argument.ty,
                                &self.candidate_type(candidate, index),
                                &self.module,
                            )
                            .is_some()
                        })
                });
        let mut remaining_candidates = viable_candidates.clone();
        let Some(mut best_candidate) = remaining_candidates.next() else {
            return Err(ParseError {
                message: format!("no matching device helper overload `{name}`"),
                span,
            });
        };
        for candidate in remaining_candidates {
            if self.better_candidate(candidate, best_candidate, arguments) {
                best_candidate = candidate;
            }
        }
        if viable_candidates.any(|candidate| {
            candidate != best_candidate
                && !self.better_candidate(best_candidate, candidate, arguments)
        }) {
            return Err(ParseError {
                message: format!("ambiguous device helper overload `{name}`"),
                span,
            });
        }
        match best_candidate {
            FunctionCandidate::Defined(_)
            | FunctionCandidate::Builtin(_)
            | FunctionCandidate::Multiply24(_)
            | FunctionCandidate::Bitcast { .. }
            | FunctionCandidate::Unary { .. }
            | FunctionCandidate::Sqrt
            | FunctionCandidate::Constructor { .. } => Ok(best_candidate),
            FunctionCandidate::Current => Err(ParseError {
                message: String::from("recursive device helper calls are unsupported"),
                span,
            }),
            FunctionCandidate::IntegralFabs(_) => Err(ParseError {
                message: String::from("integral fabs requires unsupported double precision"),
                span,
            }),
        }
    }

    fn integral_fabs_candidate(
        &self,
        name: &str,
        arguments: &[Value],
    ) -> Option<FunctionCandidate> {
        if name != "fabs"
            || arguments.len() != 1
            || !matches!(
                self.global_declarations.get(name),
                Some(&GlobalDeclaration::Functions(_))
            )
        {
            return None;
        }
        let &TypeInner::Scalar(source) = arguments[0].ty.inner_with(&self.module.types) else {
            return None;
        };
        if ![Scalar::BOOL, Scalar::I32, Scalar::U32].contains(&source) {
            return None;
        }
        // An exact ordinary overload wins its tie with the arithmetic template.
        let exact_ordinary = self.function_candidates(name).any(|candidate| {
            matches!(
                candidate,
                FunctionCandidate::Defined(_) | FunctionCandidate::Current
            ) && self.candidate_arity(candidate) == 1
                && self
                    .module
                    .compare_types(&self.candidate_type(candidate, 0), &arguments[0].ty)
        });
        (!exact_ordinary).then_some(FunctionCandidate::IntegralFabs(source))
    }
}
