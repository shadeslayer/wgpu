use super::parameter_calls::{DeferredArgument, DeferredCall, MappedExpression};
use super::pointers::{PointerRemapper, PointerValue};
use super::*;

mod budget;
mod expressions;
mod statements;
mod validation;

use budget::limit_error;
pub(super) use budget::ExpansionBudget;

struct Invocation<'a> {
    source: &'a Function,
    source_state: Option<&'a FunctionParameterState>,
    arguments: Vec<MappedExpression>,
    expressions: Vec<Option<MappedExpression>>,
    mapped_handles: Vec<Handle<Expression>>,
    locals: Vec<Option<Handle<LocalVariable>>>,
    result: Option<Handle<Expression>>,
    returned: Option<Handle<Expression>>,
    span: Span,
}

#[derive(Clone, Copy)]
enum ReturnMode {
    Preserve,
    Inline { tail_only: bool },
}

impl PointerRemapper for Invocation<'_> {
    fn expression(
        &mut self,
        parser: &mut Parser<'_>,
        source: Handle<Expression>,
    ) -> Result<MappedExpression, ParseError> {
        parser.invocation_expression(self, source)
    }

    fn parameter(&self, index: u32) -> Result<PointerValue, ParseError> {
        match self.arguments.get(index as usize) {
            Some(&MappedExpression::Pointer(ref pointer)) => Ok(pointer.as_ref().clone()),
            _ => Err(expansion_error(self.span)),
        }
    }
}

fn expansion_error(span: Span) -> ParseError {
    ParseError {
        message: String::from("invalid CUDA helper parameter expansion"),
        span,
    }
}

impl Parser<'_> {
    pub(super) fn lower_indirect_parameters(
        &mut self,
        order: &[Handle<Function>],
    ) -> Result<Vec<bool>, ParseError> {
        let span = order.first().map_or(Span::default(), |&function| {
            self.module.functions.get_span(function)
        });
        self.parameter_expansion_budget
            .charge_slice::<bool>(self.module.functions.len(), span)?;
        self.parameter_expansion_budget
            .charge_work(self.module.functions.len(), span)?;
        let mut templates = Vec::with_capacity(self.module.functions.len());
        for (handle, _) in self.module.functions.iter() {
            let parameters = self
                .function_declarations
                .get(&handle)
                .map(|declaration| &declaration.parameters);
            self.parameter_expansion_budget
                .charge_work(parameters.map_or(0, Vec::len), span)?;
            templates.push(parameters.is_some_and(|parameters| {
                parameters
                    .iter()
                    .any(|parameter| parameter.mode.is_indirect())
            }));
        }
        self.validate_parameter_templates(order)?;
        for &handle in order {
            let Some(state) = self.parameter_states.remove(&handle) else {
                continue;
            };
            let parameters = &self.function_declarations[&handle].parameters;
            self.parameter_expansion_budget
                .charge_slice::<SourceParameter>(
                    parameters.len(),
                    self.module.functions.get_span(handle),
                )?;
            let parameters = parameters.clone();
            let span = self.module.functions.get_span(handle);
            let function = core::mem::take(&mut self.module.functions[handle]);
            self.expand_parameter_function(function, state, parameters, span)?;
            let function = core::mem::take(&mut self.function.ir);
            let state = self.function.take_parameter_state();
            self.validate_pointer_state(&state, &function, span)?;
            if !templates[handle.index()] {
                self.check_no_live_pointer_accesses(&function, &state, span)?;
            }
            if !state.pointers.is_empty() {
                self.parameter_states.insert(handle, state);
            }
            self.module.functions[handle] = function;
        }
        if let Some(state) = self.kernel_parameter_state.take() {
            let span = state
                .deferred_calls
                .values()
                .next()
                .map_or(span, |call| call.span);
            let function = core::mem::take(&mut self.module.entry_points[0].function);
            self.expand_parameter_function(function, state, Vec::new(), span)?;
            let function = core::mem::take(&mut self.function.ir);
            let state = self.function.take_parameter_state();
            self.validate_pointer_state(&state, &function, span)?;
            self.check_no_live_pointer_accesses(&function, &state, span)?;
            self.module.entry_points[0].function = function;
        }
        self.function = FunctionContext::new(FunctionKind::Kernel);
        Ok(templates)
    }

    fn expand_parameter_function(
        &mut self,
        mut function: Function,
        state: FunctionParameterState,
        parameters: Vec<SourceParameter>,
        span: Span,
    ) -> Result<(), ParseError> {
        if !state.pointers.is_empty() {
            return self.clone_parameter_function(function, state, parameters, span);
        }
        let body = core::mem::take(&mut function.body);
        self.function = FunctionContext::new(FunctionKind::Device);
        self.function.ir = function;
        self.function.parameters = parameters;
        self.function.install_parameter_state(state);
        self.function.emitter = crate::proc::Emitter::default();
        self.function.emitter.start(&self.function.ir.expressions);
        self.expand_parameter_block(body, 1)?;
        self.flush_emitter();
        self.parameter_expansion_budget
            .check_control_depth(&self.function.ir.body, 1, span)?;
        self.check_remaining_parameter_markers(span)?;
        self.function.deferred_calls.clear();
        Ok(())
    }

    fn clone_parameter_function(
        &mut self,
        source: Function,
        state: FunctionParameterState,
        parameters: Vec<SourceParameter>,
        span: Span,
    ) -> Result<(), ParseError> {
        self.parameter_expansion_budget
            .charge_bytes(source.name.as_ref().map_or(0, String::len), span)?;
        self.parameter_expansion_budget
            .charge_nodes(source.arguments.len(), span)?;
        self.parameter_expansion_budget
            .charge_slice::<FunctionArgument>(source.arguments.len(), span)?;
        self.parameter_expansion_budget
            .charge_work(source.arguments.len(), span)?;
        for argument in &source.arguments {
            self.parameter_expansion_budget
                .charge_bytes(argument.name.as_ref().map_or(0, String::len), span)?;
        }
        self.function = FunctionContext::new(FunctionKind::Device);
        self.function.ir.name = source.name.clone();
        self.function.ir.arguments = source.arguments.clone();
        self.function.ir.result = source.result.clone();
        self.function.ir.diagnostic_filter_leaf = source.diagnostic_filter_leaf;
        self.function.parameters = parameters;
        self.prepare_parameter_bindings(span)?;
        self.parameter_expansion_budget
            .charge_slice::<MappedExpression>(source.arguments.len(), span)?;
        let mut arguments = Vec::with_capacity(source.arguments.len());
        for (index, argument) in source.arguments.iter().enumerate() {
            let parameter = self.function.parameters.get(index).copied();
            let mapped = match parameter.map(|parameter| parameter.mode) {
                Some(ParameterMode::Pointer { is_const }) => {
                    let ty = TypeResolution::Handle(argument.ty);
                    let pointer = self.parameter_pointer(index as u32, &ty, is_const, span)?;
                    MappedExpression::Pointer(self.boxed_pointer_value(pointer)?)
                }
                _ => MappedExpression::Value(
                    self.append(Expression::FunctionArgument(index as u32), span)?,
                ),
            };
            arguments.push(mapped);
        }
        let locals = self.clone_invocation_locals(&source, Some(&state), span)?;
        self.parameter_expansion_budget
            .charge_slice::<Option<MappedExpression>>(source.expressions.len(), span)?;
        let mut invocation = Invocation {
            source: &source,
            source_state: Some(&state),
            arguments,
            expressions: alloc::vec![None; source.expressions.len()],
            mapped_handles: Vec::new(),
            locals,
            result: None,
            returned: None,
            span,
        };
        for (handle, local) in source.local_variables.iter() {
            if let (Some(mapped), Some(initial)) = (invocation.locals[handle.index()], local.init) {
                let initial = self.invocation_value(&mut invocation, initial)?;
                self.function.ir.local_variables[mapped].init = Some(initial);
            }
        }
        self.clone_parameter_block(&source.body, &mut invocation, 1, ReturnMode::Preserve)?;
        self.flush_emitter();
        self.parameter_expansion_budget
            .check_control_depth(&self.function.ir.body, 1, span)?;
        self.check_remaining_parameter_markers(span)?;
        self.function.deferred_calls.clear();
        Ok(())
    }

    fn clone_invocation_locals(
        &mut self,
        source: &Function,
        state: Option<&FunctionParameterState>,
        span: Span,
    ) -> Result<Vec<Option<Handle<LocalVariable>>>, ParseError> {
        self.parameter_expansion_budget
            .charge_slice::<Option<Handle<LocalVariable>>>(source.local_variables.len(), span)?;
        let mut locals = Vec::with_capacity(source.local_variables.len());
        for (handle, local) in source.local_variables.iter() {
            if state.is_some_and(|state| state.pointers.validation_addresses.contains_key(&handle))
            {
                locals.push(None);
                continue;
            }
            self.parameter_expansion_budget.charge_nodes(1, span)?;
            self.parameter_expansion_budget
                .charge_slice::<(LocalVariable, Span)>(1, span)?;
            self.parameter_expansion_budget
                .charge_bytes(local.name.as_ref().map_or(0, String::len), span)?;
            locals.push(Some(self.function.ir.local_variables.append(
                LocalVariable {
                    name: local.name.clone(),
                    ty: local.ty,
                    init: None,
                },
                source.local_variables.get_span(handle),
            )));
        }
        Ok(locals)
    }

    fn generated_statement(&mut self, statement: Statement, span: Span) -> Result<(), ParseError> {
        self.parameter_expansion_budget.charge_nodes(1, span)?;
        self.parameter_expansion_budget
            .charge_slice::<(Statement, Span)>(1, span)?;
        self.push_statement(statement, span);
        Ok(())
    }

    fn inline_parameter_call(&mut self, call: DeferredCall, depth: u32) -> Result<(), ParseError> {
        let source = &self.module.functions[call.function];
        self.parameter_expansion_budget
            .inventory_function(source, call.span, depth)?;
        let source = source.clone();
        let source_state = match self.parameter_states.remove(&call.function) {
            Some(state) => {
                self.inventory_parameter_state(&state, call.span)?;
                let snapshot = state.clone();
                self.parameter_states.insert(call.function, state);
                Some(snapshot)
            }
            None => None,
        };
        self.parameter_expansion_budget
            .charge_slice::<Option<MappedExpression>>(source.expressions.len(), call.span)?;
        self.parameter_expansion_budget
            .charge_slice::<MappedExpression>(call.arguments.len(), call.span)?;
        let mut arguments = Vec::with_capacity(call.arguments.len());
        for (argument, parameter) in call.arguments.into_iter().zip(&source.arguments) {
            arguments.push(match argument {
                DeferredArgument::Value(value) => MappedExpression::Value(value),
                DeferredArgument::Reference(binding) => {
                    let TypeInner::Pointer { base, .. } = self.module.types[parameter.ty].inner
                    else {
                        return Err(expansion_error(call.span));
                    };
                    MappedExpression::Reference {
                        binding,
                        ty: TypeResolution::Handle(base),
                    }
                }
                DeferredArgument::Pointer(pointer) => MappedExpression::Pointer(pointer),
            });
        }
        let locals = self.clone_invocation_locals(&source, source_state.as_ref(), call.span)?;
        let mut invocation = Invocation {
            source: &source,
            source_state: source_state.as_ref(),
            arguments,
            expressions: alloc::vec![None; source.expressions.len()],
            mapped_handles: Vec::new(),
            locals,
            result: call.result_pointer,
            returned: None,
            span: call.span,
        };
        for (handle, local) in source.local_variables.iter() {
            let Some(local_handle) = invocation.locals[handle.index()] else {
                continue;
            };
            let value = match local.init {
                Some(initial) => self.invocation_value(&mut invocation, initial)?,
                None => self.append(Expression::ZeroValue(local.ty), call.span)?,
            };
            let pointer = self.append(Expression::LocalVariable(local_handle), call.span)?;
            self.generated_statement(Statement::Store { pointer, value }, call.span)?;
        }
        let tail_returns_only = statements::has_only_tail_returns(
            &source.body,
            &mut self.parameter_expansion_budget,
            call.span,
        )?;
        if tail_returns_only {
            self.clone_parameter_block(
                &source.body,
                &mut invocation,
                depth,
                ReturnMode::Inline { tail_only: true },
            )?;
        } else {
            if depth + 1 > 127 {
                return Err(limit_error(call.span));
            }
            let flag = self.temporary_pointer(&scalar_resolution(Scalar::BOOL), call.span)?;
            let value = self.append(Expression::Literal(Literal::Bool(false)), call.span)?;
            self.generated_statement(
                Statement::Store {
                    pointer: flag,
                    value,
                },
                call.span,
            )?;
            invocation.returned = Some(flag);
            let (_, mut body) = self.capture_block(|parser| {
                parser.clone_parameter_block(
                    &source.body,
                    &mut invocation,
                    depth + 1,
                    ReturnMode::Inline { tail_only: false },
                )
            })?;
            self.parameter_expansion_budget.charge_nodes(1, call.span)?;
            self.parameter_expansion_budget
                .charge_slice::<(Statement, Span)>(1, call.span)?;
            body.push(Statement::Break, call.span);
            self.generated_statement(
                Statement::Loop {
                    body,
                    continuing: crate::Block::new(),
                    break_if: None,
                },
                call.span,
            )?;
        }
        Ok(())
    }
}
