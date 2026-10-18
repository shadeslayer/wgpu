use super::*;

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
}

#[derive(Clone, Copy, PartialEq)]
enum FunctionCandidate {
    Defined(Handle<Function>),
    Current,
    Builtin(MathBuiltin),
    Constructor { size: VectorSize, scalar: Scalar },
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
        self.expect_symbol('(')?;
        if !self.take_symbol(')')? {
            if self.take_ident("void")? {
                self.expect_symbol(')')?;
            } else {
                loop {
                    self.parameter()?;
                    if self.take_symbol(')')? {
                        break;
                    }
                    self.expect_symbol(',')?;
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
        let mut inline = false;
        let mut seen_static = false;
        let mut cuda_inline_span = None;
        let mut explicit_const = false;
        let mut seen_int = false;
        let mut seen_unsigned = false;
        loop {
            let token = self.lexer.peek()?;
            match token.kind {
                TokenKind::Ident("inline") => {
                    if inline {
                        return self.fail("duplicate inline specifier");
                    }
                    if cuda_inline_span.is_some() {
                        return self.fail("incompatible inlining specifiers");
                    }
                    inline = true;
                }
                TokenKind::Ident("static") => {
                    if extern_c {
                        return self.fail("static conflicts with an extern C declaration");
                    }
                    if seen_static {
                        return self.fail("duplicate static specifier");
                    }
                    seen_static = true;
                }
                TokenKind::Ident("__forceinline__" | "__noinline__") => {
                    if inline || cuda_inline_span.is_some() {
                        return self.fail("duplicate or incompatible inlining specifiers");
                    }
                    cuda_inline_span = Some(token.span);
                }
                TokenKind::Ident("__host__") => {
                    if host_span.is_some() || kind == Some(FunctionKind::Kernel) {
                        return self
                            .fail("duplicate or incompatible CUDA execution space specifier");
                    }
                    host_span = Some(token.span);
                }
                TokenKind::Ident("__device__" | "__global__") => {
                    if kind.is_some()
                        || (host_span.is_some() && token.kind == TokenKind::Ident("__global__"))
                    {
                        return self
                            .fail("duplicate or incompatible CUDA execution space specifier");
                    }
                    kind = Some(if token.kind == TokenKind::Ident("__device__") {
                        FunctionKind::Device
                    } else {
                        FunctionKind::Kernel
                    });
                }
                TokenKind::Ident("const") => {
                    if explicit_const {
                        return self.fail("duplicate const qualifier");
                    }
                    explicit_const = true;
                    result_span.get_or_insert(token.span);
                }
                TokenKind::Ident("int") => {
                    if seen_int || (result.is_some() && !seen_unsigned) {
                        return self.fail("incompatible or repeated return type specifier");
                    }
                    seen_int = true;
                    result = Some(ReturnType::Value(TypeSpecifier::Scalar(if seen_unsigned {
                        Scalar::U32
                    } else {
                        Scalar::I32
                    })));
                    result_span.get_or_insert(token.span);
                }
                TokenKind::Ident("unsigned") => {
                    if seen_unsigned || (result.is_some() && !seen_int) {
                        return self.fail("incompatible or repeated return type specifier");
                    }
                    seen_unsigned = true;
                    result = Some(ReturnType::Value(TypeSpecifier::Scalar(Scalar::U32)));
                    result_span.get_or_insert(token.span);
                }
                TokenKind::Ident("void" | "float" | "bool") => {
                    if result.is_some() {
                        return self.fail("incompatible or repeated return type specifier");
                    }
                    result = Some(match token.kind {
                        TokenKind::Ident("void") => ReturnType::Void,
                        TokenKind::Ident("float") => {
                            ReturnType::Value(TypeSpecifier::Scalar(Scalar::F32))
                        }
                        _ => ReturnType::Value(TypeSpecifier::Scalar(Scalar::BOOL)),
                    });
                    result_span.get_or_insert(token.span);
                }
                TokenKind::Ident(name) if result.is_none() => {
                    let Some(ty) = self.type_name(name) else {
                        break;
                    };
                    result = Some(ReturnType::Value(ty.ty));
                    result_span.get_or_insert(token.span);
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
        let (name, name_span) = self.identifier()?;
        self.check_function_name(name, name_span, kind)?;
        Ok(FunctionHeader {
            kind,
            result,
            result_span: result_span.unwrap(),
            name,
            name_span,
        })
    }

    pub(super) fn function_definition(&mut self) -> Result<FunctionKind, ParseError> {
        let extern_c = self.take_ident("extern")?;
        if extern_c {
            if self.lexer.peek()?.kind != TokenKind::String("C") {
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
        if !matches!(header.result, ReturnType::Void) {
            return Err(ParseError {
                message: String::from("CUDA kernels must return void"),
                span: header.result_span,
            });
        }
        self.parameters()?;
        self.expect_symbol('{')?;
        self.block_contents()?;
        self.function
            .ir
            .body
            .extend(self.function.emitter.finish(&self.function.ir.expressions));
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
            function: function.ir,
            mesh_info: None,
            task_payload: None,
            incoming_ray_payload: None,
        });
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
        if self
            .function_candidates(header.name)
            .filter(|candidate| {
                matches!(
                    candidate,
                    FunctionCandidate::Builtin(_) | FunctionCandidate::Constructor { .. }
                )
            })
            .any(|candidate| {
                self.function.ir.arguments.len() == self.candidate_arity(candidate)
                    && (0..self.candidate_arity(candidate)).all(|index| {
                        self.module.compare_types(
                            &self.candidate_type(FunctionCandidate::Current, index),
                            &self.candidate_type(candidate, index),
                        )
                    })
            })
        {
            return Err(ParseError {
                message: format!("duplicate builtin definition `{}`", header.name),
                span: header.name_span,
            });
        }
        if let Some(&GlobalDeclaration::Functions(ref functions)) =
            self.global_declarations.get(header.name)
        {
            let parameters = &self.function.ir.arguments;
            if functions.iter().any(|&function| {
                self.module.functions[function]
                    .arguments
                    .iter()
                    .map(|argument| argument.ty)
                    .eq(parameters.iter().map(|argument| argument.ty))
            }) {
                return Err(ParseError {
                    message: format!("duplicate device helper definition `{}`", header.name),
                    span: header.name_span,
                });
            }
        }
        self.expect_symbol('{')?;
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
        let handle = self.module.functions.append(function.ir, header.name_span);
        match *self
            .global_declarations
            .entry(header.name)
            .or_insert_with(|| GlobalDeclaration::Functions(Vec::new()))
        {
            GlobalDeclaration::Functions(ref mut functions) => functions.push(handle),
            _ => unreachable!(),
        }
        Ok(())
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
                message: format!("unknown or not previously defined device helper `{name}`"),
                span,
            });
        }
        self.expect_symbol('(')?;
        let mut arguments = Vec::new();
        if self.lexer.peek()?.kind != TokenKind::Symbol(')') {
            loop {
                let value = self.expression(0)?;
                arguments.push(self.rvalue(value)?);
                if self.lexer.peek()?.kind == TokenKind::Symbol(')') {
                    break;
                }
                self.expect_symbol(',')?;
            }
        }
        let end = self.expect_symbol(')')?;
        let candidate = self.resolve_overload(name, &arguments, span)?;
        if let FunctionCandidate::Constructor { size, scalar } = candidate {
            let components = arguments
                .into_iter()
                .map(|value| {
                    self.convert_scalar(value.into(), scalar)
                        .map(|value| value.handle)
                })
                .collect::<Result<Vec<_>, _>>()?;
            let span = span.until(&end);
            let ty = self.lower_type(TypeSpecifier::Vector { size, scalar }, span);
            let handle_ty = self.type_handle(&ty, span);
            let handle = self.append(
                Expression::Compose {
                    ty: handle_ty,
                    components,
                },
                span,
            )?;
            return Ok(Value { handle, ty, span }.into());
        }
        if let FunctionCandidate::Builtin(builtin) = candidate {
            let mut arguments = arguments.into_iter();
            let left = self.convert_scalar(arguments.next().unwrap().into(), builtin.result)?;
            let right = self.convert_scalar(arguments.next().unwrap().into(), builtin.result)?;
            let span = span.until(&end);
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
                handle,
                ty: scalar_resolution(builtin.result),
                span,
            }
            .into());
        }
        let FunctionCandidate::Defined(function) = candidate else {
            unreachable!()
        };
        let mut converted = Vec::with_capacity(arguments.len());
        for (index, value) in arguments.into_iter().enumerate() {
            let parameter = &self.module.functions[function].arguments[index];
            let ty = TypeResolution::Handle(parameter.ty);
            converted.push(self.convert_value(value.into(), &ty)?.handle);
        }
        let span = span.until(&end);
        let ty = self.module.functions[function]
            .result
            .as_ref()
            .map(|result| TypeResolution::Handle(result.ty));
        let value = if let Some(ty) = ty {
            let handle = self.append_call_result(function, span)?;
            Some(Value { handle, ty, span })
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
            .map(FunctionCandidate::Defined)
            .chain(current)
            .chain(
                math_builtins(name)
                    .filter(move |_| builtin_visible)
                    .map(FunctionCandidate::Builtin),
            )
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
        let mut viable_candidates = self.function_candidates(name).filter(|&candidate| {
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
            | FunctionCandidate::Constructor { .. } => Ok(best_candidate),
            FunctionCandidate::Current => Err(ParseError {
                message: String::from("recursive device helper calls are unsupported"),
                span,
            }),
        }
    }
}
