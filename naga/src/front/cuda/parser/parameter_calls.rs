use super::pointers::{PointerState, PointerValue};
use super::*;

#[derive(Clone, Copy, Eq, PartialEq)]
pub(super) enum ParameterMode {
    Value,
    Reference { is_const: bool },
    Pointer { is_const: bool },
}

impl ParameterMode {
    pub(super) const fn is_indirect(self) -> bool {
        matches!(self, Self::Reference { .. } | Self::Pointer { .. })
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub(super) struct SourceParameter {
    pub(super) ty: Handle<Type>,
    pub(super) mode: ParameterMode,
}

#[derive(Clone)]
pub(super) enum DeferredArgument {
    Value(Handle<Expression>),
    Reference(Handle<BoundReference>),
    Pointer(Box<PointerValue>),
}

#[derive(Clone)]
pub(super) enum MappedExpression {
    Value(Handle<Expression>),
    Reference {
        binding: Handle<BoundReference>,
        ty: TypeResolution,
    },
    Pointer(Box<PointerValue>),
}

#[derive(Clone)]
pub(super) struct DeferredCall {
    pub(super) function: Handle<Function>,
    pub(super) arguments: Vec<DeferredArgument>,
    pub(super) result_pointer: Option<Handle<Expression>>,
    pub(super) span: Span,
}

#[derive(Clone, Default)]
pub(super) struct FunctionParameterState {
    pub(super) expression_graph_depths: Vec<u32>,
    pub(super) deferred_calls: FastHashMap<Handle<Expression>, DeferredCall>,
    pub(super) bound_addresses: Arena<BoundAddress>,
    pub(super) bound_address_indices: FastHashMap<BoundAddress, Handle<BoundAddress>>,
    pub(super) bound_references: Arena<BoundReference>,
    pub(super) reference_binding_work: u32,
    pub(super) pointers: PointerState,
}

pub(super) struct CapturedArgument {
    pub(super) value: ExpressionValue,
    pub(super) body: crate::Block,
}

#[derive(Clone, Copy)]
pub(super) struct SourcePointerType {
    pub(super) pointee: Handle<Type>,
    pub(super) is_const: bool,
}

pub(super) trait CallArgument {
    fn ty(&self) -> &TypeResolution;
    fn span(&self) -> Span;
    fn reference_writable(&self) -> Option<bool>;
    fn pointer_type(&self, module: &Module) -> Option<SourcePointerType>;
}

impl CallArgument for Value {
    fn ty(&self) -> &TypeResolution {
        &self.ty
    }
    fn span(&self) -> Span {
        self.span
    }
    fn reference_writable(&self) -> Option<bool> {
        None
    }
    fn pointer_type(&self, _module: &Module) -> Option<SourcePointerType> {
        None
    }
}

impl CallArgument for CapturedArgument {
    fn ty(&self) -> &TypeResolution {
        self.value.ty()
    }
    fn span(&self) -> Span {
        self.value.span()
    }
    fn reference_writable(&self) -> Option<bool> {
        match self.value {
            ExpressionValue::Reference(ref reference) => Some(reference.writable),
            ExpressionValue::Plain(_) | ExpressionValue::Pointer(_) => None,
        }
    }
    fn pointer_type(&self, module: &Module) -> Option<SourcePointerType> {
        match self.value {
            ExpressionValue::Pointer(ref pointer) => match *pointer.ty.inner_with(&module.types) {
                TypeInner::Pointer { base, .. } => Some(SourcePointerType {
                    pointee: base,
                    is_const: pointer.is_const,
                }),
                _ => None,
            },
            ExpressionValue::Reference(ref reference) => {
                match *reference.ty.inner_with(&module.types) {
                    TypeInner::Array { base, .. } => Some(SourcePointerType {
                        pointee: base,
                        is_const: !reference.writable,
                    }),
                    _ => None,
                }
            }
            ExpressionValue::Plain(_) => None,
        }
    }
}

impl Parser<'_> {
    pub(super) fn activate_parameter_expansion(&mut self, span: Span) -> Result<(), ParseError> {
        if self.parameter_expansion_budget.activate() {
            for declaration in self.function_declarations.values() {
                self.parameter_expansion_budget
                    .charge_slice::<SourceParameter>(declaration.parameters.len(), span)?;
            }
            self.parameter_expansion_budget
                .charge_slice::<SourceParameter>(self.function.parameters.len(), span)?;
        }
        Ok(())
    }

    pub(super) fn prepare_parameter_bindings(&mut self, span: Span) -> Result<(), ParseError> {
        if self.function.parameter_state_charged {
            return Ok(());
        }
        let budget = &mut self.parameter_expansion_budget;
        budget.charge_bytes(size_of::<FunctionParameterState>(), span)?;
        budget.charge_slice::<BoundAddress>(self.function.bound_addresses.len(), span)?;
        for (_, address) in self.function.bound_addresses.iter() {
            budget.charge_slice::<reference_bindings::AddressProjection>(
                address.projections.len(),
                span,
            )?;
        }
        budget.charge_map_entry::<BoundAddress, Handle<BoundAddress>>(
            self.function.bound_address_indices.len(),
            span,
        )?;
        for address in self.function.bound_address_indices.keys() {
            budget.charge_slice::<reference_bindings::AddressProjection>(
                address.projections.len(),
                span,
            )?;
        }
        budget.charge_slice::<BoundReference>(self.function.bound_references.len(), span)?;
        for (_, reference) in self.function.bound_references.iter() {
            budget
                .charge_slice::<reference_bindings::BoundTarget>(reference.targets.len(), span)?;
        }
        self.function.parameter_state_charged = true;
        Ok(())
    }

    pub(super) fn parameter_state_needed(&self) -> bool {
        !self.function.deferred_calls.is_empty()
            || !self.function.pointers.is_empty()
            || self
                .function
                .parameters
                .iter()
                .any(|parameter| parameter.mode.is_indirect())
    }

    pub(super) fn capture_call_argument(&mut self) -> Result<CapturedArgument, ParseError> {
        let span = self.lexer.peek()?.1;
        self.parameter_expansion_budget.charge_nodes(1, span)?;
        self.parameter_expansion_budget
            .charge_slice::<CapturedArgument>(1, span)?;
        let (value, body) = self.capture_block(|parser| parser.expression(0))?;
        Ok(CapturedArgument { value, body })
    }

    pub(super) fn has_indirect_candidate(&self, name: &str) -> bool {
        self.function_candidates(name).any(|candidate| {
            (0..self.candidate_arity(candidate))
                .any(|index| self.candidate_mode(candidate, index).is_indirect())
        })
    }

    pub(super) fn call_with_captured_arguments(
        &mut self,
        name: &str,
        arguments: Vec<CapturedArgument>,
        name_span: Span,
        span: Span,
    ) -> Result<ExpressionResult, ParseError> {
        let candidate = self.resolve_overload(name, &arguments, name_span)?;
        let has_indirect_parameters =
            (0..arguments.len()).any(|index| self.candidate_mode(candidate, index).is_indirect());
        if !has_indirect_parameters {
            self.parameter_expansion_budget
                .charge_slice::<Value>(arguments.len(), span)?;
            let mut values = Vec::with_capacity(arguments.len());
            for argument in arguments {
                self.append_block(argument.body);
                let target = self.candidate_type(candidate, values.len());
                values.push(self.convert_value(argument.value, &target)?);
            }
            return self.lower_call(candidate, values, span);
        }
        let functions::FunctionCandidate::Defined(function) = candidate else {
            unreachable!()
        };
        self.prepare_parameter_bindings(span)?;
        self.parameter_expansion_budget
            .charge_slice::<DeferredArgument>(arguments.len(), span)?;
        self.parameter_expansion_budget
            .charge_map_entry::<Handle<Expression>, DeferredCall>(1, span)?;
        self.parameter_expansion_budget
            .charge_slice::<FunctionCall>(1, span)?;
        let mut bound_arguments = Vec::with_capacity(arguments.len());
        for (index, argument) in arguments.into_iter().enumerate() {
            self.append_block(argument.body);
            let target = self.candidate_type(candidate, index);
            let argument = match self.candidate_mode(candidate, index) {
                ParameterMode::Value => {
                    DeferredArgument::Value(self.convert_value(argument.value, &target)?.handle)
                }
                ParameterMode::Reference { is_const } => {
                    let reference = match argument.value {
                        ExpressionValue::Reference(reference)
                            if self.module.compare_types(&reference.ty, &target) =>
                        {
                            reference
                        }
                        value => {
                            let value = self.convert_value(value, &target)?;
                            let pointer = self.temporary_pointer(&target, span)?;
                            self.push_statement(
                                Statement::Store {
                                    pointer,
                                    value: value.handle,
                                },
                                value.span,
                            );
                            ReferenceValue {
                                reference: Reference::Pointer(pointer),
                                writable: false,
                                ty: target,
                                span: value.span,
                                constant: ConstantValue::default(),
                            }
                        }
                    };
                    if !is_const && !reference.writable {
                        return self.fail("mutable reference requires a writable lvalue");
                    }
                    DeferredArgument::Reference(self.freeze_reference(reference)?)
                }
                ParameterMode::Pointer { is_const } => {
                    let pointer = self.convert_pointer(argument.value, &target, is_const, span)?;
                    DeferredArgument::Pointer(self.boxed_pointer_value(pointer)?)
                }
            };
            bound_arguments.push(argument);
        }
        let result_ty = self.module.functions[function]
            .result
            .as_ref()
            .map(|result| TypeResolution::Handle(result.ty));
        let result_pointer = match result_ty.as_ref() {
            Some(ty) => Some(self.temporary_pointer(ty, span)?),
            None => None,
        };
        self.parameter_expansion_budget.charge_nodes(1, span)?;
        // Keep a unique emitted identity until the deferred call is expanded.
        let (marker, body) = self.capture_block(|parser| {
            let zero = parser.append(Expression::Literal(Literal::U32(0)), span)?;
            parser.append(
                Expression::Binary {
                    op: BinaryOperator::Add,
                    left: zero,
                    right: zero,
                },
                span,
            )
        })?;
        self.function.calls.push(FunctionCall { function, span });
        self.function.deferred_calls.insert(
            marker,
            DeferredCall {
                function,
                arguments: bound_arguments,
                result_pointer,
                span,
            },
        );
        self.push_statement(Statement::Block(body), span);
        match (result_pointer, result_ty) {
            (Some(pointer), Some(ty)) => Ok(self.load_pointer(pointer, &ty, span)?.into()),
            _ => Ok(ExpressionResult::Void(span)),
        }
    }
}

impl FunctionContext<'_> {
    pub(super) fn take_parameter_state(&mut self) -> FunctionParameterState {
        FunctionParameterState {
            expression_graph_depths: core::mem::take(&mut self.expression_graph_depths),
            deferred_calls: core::mem::take(&mut self.deferred_calls),
            bound_addresses: core::mem::take(&mut self.bound_addresses),
            bound_address_indices: core::mem::take(&mut self.bound_address_indices),
            bound_references: core::mem::take(&mut self.bound_references),
            reference_binding_work: core::mem::take(&mut self.reference_binding_work),
            pointers: core::mem::take(&mut self.pointers),
        }
    }

    pub(super) fn install_parameter_state(&mut self, state: FunctionParameterState) {
        self.expression_graph_depths = state.expression_graph_depths;
        self.deferred_calls = state.deferred_calls;
        self.bound_addresses = state.bound_addresses;
        self.bound_address_indices = state.bound_address_indices;
        self.bound_references = state.bound_references;
        self.reference_binding_work = state.reference_binding_work;
        self.pointers = state.pointers;
        self.parameter_state_charged = true;
    }
}
