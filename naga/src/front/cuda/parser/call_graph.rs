use super::parameter_expansion::ExpansionBudget;
use super::*;

#[derive(Clone, Copy, PartialEq)]
enum VisitState {
    Unvisited,
    Visiting,
    Complete,
}

fn push_work<T>(
    pending: &mut Vec<T>,
    item: T,
    budget: &mut ExpansionBudget,
    span: Span,
) -> Result<(), ParseError> {
    budget.charge_slice::<T>(1, span)?;
    pending.push(item);
    Ok(())
}

fn emitted_calls(
    function: &Function,
    calls: &mut Vec<FunctionCall>,
    budget: &mut ExpansionBudget,
    span: Span,
) -> Result<(), ParseError> {
    let mut blocks = Vec::new();
    push_work(&mut blocks, &function.body, budget, span)?;
    while let Some(block) = blocks.pop() {
        for (statement, span) in block.span_iter() {
            budget.charge_work(1, *span)?;
            match *statement {
                Statement::Call { function, .. } => push_work(
                    calls,
                    FunctionCall {
                        function,
                        span: *span,
                    },
                    budget,
                    *span,
                )?,
                Statement::Block(ref body) => push_work(&mut blocks, body, budget, *span)?,
                Statement::If {
                    ref accept,
                    ref reject,
                    ..
                } => {
                    push_work(&mut blocks, reject, budget, *span)?;
                    push_work(&mut blocks, accept, budget, *span)?;
                }
                Statement::Switch { ref cases, .. } => {
                    for case in cases.iter().rev() {
                        push_work(&mut blocks, &case.body, budget, *span)?;
                    }
                }
                Statement::Loop {
                    ref body,
                    ref continuing,
                    ..
                } => {
                    push_work(&mut blocks, continuing, budget, *span)?;
                    push_work(&mut blocks, body, budget, *span)?;
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn remap_calls(
    function: &mut Function,
    handles: &[Option<Handle<Function>>],
    budget: &mut ExpansionBudget,
    span: Span,
) -> Result<(), ParseError> {
    budget.charge_work(function.expressions.len(), span)?;
    for (_, expression) in function.expressions.iter_mut() {
        if let Expression::CallResult(ref mut callee) = *expression {
            *callee = handles[callee.index()].unwrap();
        }
    }
    let mut blocks: Vec<&mut [Statement]> = Vec::new();
    push_work(&mut blocks, &mut function.body, budget, span)?;
    while let Some(block) = blocks.pop() {
        for statement in block {
            budget.charge_work(1, span)?;
            match *statement {
                Statement::Call {
                    ref mut function, ..
                } => {
                    *function = handles[function.index()].unwrap();
                }
                Statement::Block(ref mut body) => push_work(&mut blocks, body, budget, span)?,
                Statement::If {
                    ref mut accept,
                    ref mut reject,
                    ..
                } => {
                    push_work(&mut blocks, reject, budget, span)?;
                    push_work(&mut blocks, accept, budget, span)?;
                }
                Statement::Switch { ref mut cases, .. } => {
                    for case in cases {
                        push_work(&mut blocks, &mut case.body, budget, span)?;
                    }
                }
                Statement::Loop {
                    ref mut body,
                    ref mut continuing,
                    ..
                } => {
                    push_work(&mut blocks, continuing, budget, span)?;
                    push_work(&mut blocks, body, budget, span)?;
                }
                _ => {}
            }
        }
    }
    Ok(())
}

pub(super) fn reorder_functions(
    module: &mut Module,
    order: &[Handle<Function>],
    budget: &mut ExpansionBudget,
    span: Span,
) -> Result<(), ParseError> {
    budget.charge_slice::<Option<Handle<Function>>>(module.functions.len(), span)?;
    budget.charge_slice::<Option<(Function, Span)>>(module.functions.len(), span)?;
    budget.charge_slice::<(Function, Span)>(order.len(), span)?;
    budget.charge_work(module.functions.len() + order.len(), span)?;
    let mut handles = alloc::vec![None; module.functions.len()];
    let mut functions: Vec<_> = module
        .functions
        .drain()
        .map(|(_, function, span)| Some((function, span)))
        .collect();
    for &previous in order {
        let (function, span) = functions[previous.index()].take().unwrap();
        let handle = module.functions.append(function, span);
        handles[previous.index()] = Some(handle);
    }
    for (_, function) in module.functions.iter_mut() {
        remap_calls(function, &handles, budget, span)?;
    }
    for entry in &mut module.entry_points {
        remap_calls(&mut entry.function, &handles, budget, span)?;
    }
    Ok(())
}

impl Parser<'_> {
    pub(super) fn finish_functions(&mut self) -> Result<(), ParseError> {
        let span = self
            .module
            .functions
            .iter()
            .next()
            .map_or(Span::default(), |(handle, _)| {
                self.module.functions.get_span(handle)
            });
        self.parameter_expansion_budget
            .charge_slice::<bool>(self.module.functions.len(), span)?;
        self.parameter_expansion_budget
            .charge_slice::<Vec<FunctionCall>>(self.module.functions.len(), span)?;
        self.parameter_expansion_budget
            .charge_work(self.module.functions.len(), span)?;
        let mut retained = alloc::vec![true; self.module.functions.len()];
        let mut dependencies = Vec::with_capacity(self.module.functions.len());
        for (handle, function) in self.module.functions.iter() {
            let mut calls = Vec::new();
            if let Some(declaration) = self.function_declarations.get(&handle) {
                retained[handle.index()] = declaration.is_defined;
                self.parameter_expansion_budget
                    .charge_slice::<FunctionCall>(declaration.calls.len(), span)?;
                self.parameter_expansion_budget
                    .charge_work(declaration.calls.len(), span)?;
                calls.extend_from_slice(&declaration.calls);
            }
            emitted_calls(
                function,
                &mut calls,
                &mut self.parameter_expansion_budget,
                span,
            )?;
            dependencies.push(calls);
        }
        let missing_definition = |module: &Module,
                                  budget: &mut ExpansionBudget,
                                  call: FunctionCall|
         -> Result<ParseError, ParseError> {
            let name = module.functions[call.function]
                .name
                .as_deref()
                .unwrap_or_default();
            budget.charge_bytes(
                name.len().checked_add(40).ok_or_else(|| ParseError {
                    message: String::from("CUDA helper parameter expansion limit exceeded"),
                    span: call.span,
                })?,
                call.span,
            )?;
            Ok(ParseError {
                message: format!("device helper `{name}` has no definition"),
                span: call.span,
            })
        };
        self.parameter_expansion_budget
            .charge_work(self.kernel_calls.len(), span)?;
        for &call in &self.kernel_calls {
            if !retained[call.function.index()] {
                return Err(missing_definition(
                    &self.module,
                    &mut self.parameter_expansion_budget,
                    call,
                )?);
            }
        }
        self.parameter_expansion_budget
            .charge_slice::<VisitState>(self.module.functions.len(), span)?;
        self.parameter_expansion_budget
            .charge_slice::<Handle<Function>>(self.module.functions.len(), span)?;
        self.parameter_expansion_budget
            .charge_slice::<(Handle<Function>, usize)>(self.module.functions.len(), span)?;
        let mut states = alloc::vec![VisitState::Unvisited; self.module.functions.len()];
        let mut order = Vec::with_capacity(self.module.functions.len());
        let mut pending = Vec::with_capacity(self.module.functions.len());
        for (root, _) in self.module.functions.iter() {
            self.parameter_expansion_budget.charge_work(1, span)?;
            if !retained[root.index()] || states[root.index()] != VisitState::Unvisited {
                continue;
            }
            states[root.index()] = VisitState::Visiting;
            pending.push((root, 0));
            while let Some(&mut (caller, ref mut next_call)) = pending.last_mut() {
                self.parameter_expansion_budget.charge_work(1, span)?;
                if *next_call == dependencies[caller.index()].len() {
                    let (caller, _) = pending.pop().unwrap();
                    states[caller.index()] = VisitState::Complete;
                    order.push(caller);
                    continue;
                }
                let call = dependencies[caller.index()][*next_call];
                *next_call += 1;
                if !retained[call.function.index()] {
                    return Err(missing_definition(
                        &self.module,
                        &mut self.parameter_expansion_budget,
                        call,
                    )?);
                }
                match states[call.function.index()] {
                    VisitState::Complete => {}
                    VisitState::Visiting => {
                        return Err(ParseError {
                            message: String::from("recursive device helper calls are unsupported"),
                            span: call.span,
                        });
                    }
                    VisitState::Unvisited => {
                        states[call.function.index()] = VisitState::Visiting;
                        pending.push((call.function, 0));
                    }
                }
            }
        }
        if self.function_declarations.values().any(|declaration| {
            declaration
                .parameters
                .iter()
                .any(|parameter| parameter.mode.is_indirect())
        }) || !self.parameter_states.is_empty()
            || self.kernel_parameter_state.is_some()
        {
            let templates = self.lower_indirect_parameters(&order)?;
            order.retain(|function| !templates[function.index()]);
        }
        let span = order.first().map_or(Span::default(), |&function| {
            self.module.functions.get_span(function)
        });
        reorder_functions(
            &mut self.module,
            &order,
            &mut self.parameter_expansion_budget,
            span,
        )?;
        Ok(())
    }
}
