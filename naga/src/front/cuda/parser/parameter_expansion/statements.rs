use super::*;

fn deferred_call_marker(body: &crate::Block) -> Option<Handle<Expression>> {
    match body[..] {
        [Statement::Emit(ref range)]
        | [Statement::Emit(ref range), Statement::Return { value: None }] => {
            let mut handles = range.clone();
            let marker = handles.next();
            if handles.next().is_none() {
                marker
            } else {
                None
            }
        }
        _ => None,
    }
}

pub(super) fn has_only_tail_returns(
    body: &crate::Block,
    budget: &mut ExpansionBudget,
    span: Span,
) -> Result<bool, ParseError> {
    budget.charge_slice::<(&crate::Block, bool)>(1, span)?;
    let mut pending = alloc::vec![(body, true)];
    while let Some((block, tail)) = pending.pop() {
        for (index, statement) in block.iter().enumerate() {
            budget.charge_work(1, span)?;
            let statement_is_tail = tail && index + 1 == block.len();
            let mut child = |body, tail| -> Result<(), ParseError> {
                budget.charge_slice::<(&crate::Block, bool)>(1, span)?;
                pending.push((body, tail));
                Ok(())
            };
            match *statement {
                Statement::Return { .. } if statement_is_tail => {}
                Statement::Return { .. } => return Ok(false),
                Statement::Block(ref body) => child(body, statement_is_tail)?,
                Statement::If {
                    ref accept,
                    ref reject,
                    ..
                } => {
                    child(accept, statement_is_tail)?;
                    child(reject, statement_is_tail)?;
                }
                Statement::Switch { ref cases, .. } => {
                    for case in cases {
                        child(&case.body, statement_is_tail && !case.fall_through)?;
                    }
                }
                Statement::Loop {
                    ref body,
                    ref continuing,
                    ..
                } => {
                    child(body, false)?;
                    child(continuing, false)?;
                }
                _ => {}
            }
        }
    }
    Ok(true)
}

impl Parser<'_> {
    pub(super) fn expand_parameter_block(
        &mut self,
        body: crate::Block,
        depth: u32,
    ) -> Result<(), ParseError> {
        if depth > 127 {
            return Err(limit_error(Span::default()));
        }
        for (statement, span) in body.span_into_iter() {
            self.parameter_expansion_budget.charge_work(1, span)?;
            match statement {
                Statement::Block(body) => {
                    let marker = match body[..] {
                        [Statement::Emit(ref range)]
                        | [Statement::Emit(ref range), Statement::Return { value: None }] => {
                            let mut handles = range.clone();
                            let first = handles.next();
                            if handles.next().is_none() {
                                first
                            } else {
                                None
                            }
                        }
                        _ => None,
                    };
                    if let Some(call) =
                        marker.and_then(|marker| self.function.deferred_calls.remove(&marker))
                    {
                        self.inline_parameter_call(call, depth)?;
                        if let Some((&Statement::Return { value: None }, return_span)) =
                            body.span_iter().nth(1)
                        {
                            self.generated_statement(
                                Statement::Return { value: None },
                                *return_span,
                            )?;
                        }
                    } else {
                        let ((), body) = self.capture_block(|parser| {
                            parser.expand_parameter_block(body, depth + 1)
                        })?;
                        self.generated_statement(Statement::Block(body), span)?;
                    }
                }
                Statement::If {
                    condition,
                    accept,
                    reject,
                } => {
                    let ((), accept) = self
                        .capture_block(|parser| parser.expand_parameter_block(accept, depth + 1))?;
                    let ((), reject) = self
                        .capture_block(|parser| parser.expand_parameter_block(reject, depth + 1))?;
                    self.generated_statement(
                        Statement::If {
                            condition,
                            accept,
                            reject,
                        },
                        span,
                    )?;
                }
                Statement::Switch { selector, cases } => {
                    self.parameter_expansion_budget
                        .charge_slice::<crate::SwitchCase>(cases.len(), span)?;
                    let mut mapped = Vec::with_capacity(cases.len());
                    for case in cases {
                        let ((), body) = self.capture_block(|parser| {
                            parser.expand_parameter_block(case.body, depth + 3)
                        })?;
                        mapped.push(crate::SwitchCase {
                            value: case.value,
                            body,
                            fall_through: case.fall_through,
                        });
                    }
                    self.generated_statement(
                        Statement::Switch {
                            selector,
                            cases: mapped,
                        },
                        span,
                    )?;
                }
                Statement::Loop {
                    body,
                    continuing,
                    break_if,
                } => {
                    let ((), body) = self
                        .capture_block(|parser| parser.expand_parameter_block(body, depth + 1))?;
                    let ((), continuing) = self.capture_block(|parser| {
                        parser.expand_parameter_block(continuing, depth + 2)
                    })?;
                    self.generated_statement(
                        Statement::Loop {
                            body,
                            continuing,
                            break_if,
                        },
                        span,
                    )?;
                }
                statement => self.generated_statement(statement, span)?,
            }
        }
        Ok(())
    }

    pub(super) fn check_remaining_parameter_markers(
        &mut self,
        span: Span,
    ) -> Result<(), ParseError> {
        self.parameter_expansion_budget
            .charge_slice::<&crate::Block>(1, span)?;
        let mut pending = alloc::vec![&self.function.ir.body];
        while let Some(block) = pending.pop() {
            for statement in block {
                self.parameter_expansion_budget.charge_work(1, span)?;
                let mut child = |body| -> Result<(), ParseError> {
                    self.parameter_expansion_budget
                        .charge_slice::<&crate::Block>(1, span)?;
                    pending.push(body);
                    Ok(())
                };
                match *statement {
                    Statement::Emit(ref range) => {
                        for expression in range.clone() {
                            self.parameter_expansion_budget.charge_work(1, span)?;
                            if let Some(call) = self.function.deferred_calls.get(&expression) {
                                return Err(expansion_error(call.span));
                            }
                        }
                    }
                    Statement::Block(ref body) => child(body)?,
                    Statement::If {
                        ref accept,
                        ref reject,
                        ..
                    } => {
                        child(accept)?;
                        child(reject)?;
                    }
                    Statement::Switch { ref cases, .. } => {
                        for case in cases {
                            child(&case.body)?;
                        }
                    }
                    Statement::Loop {
                        ref body,
                        ref continuing,
                        ..
                    } => {
                        child(body)?;
                        child(continuing)?;
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn propagate_parameter_return(
        &mut self,
        invocation: &Invocation<'_>,
        depth: u32,
    ) -> Result<(), ParseError> {
        if let Some(pointer) = invocation.returned {
            if depth + 1 > 127 {
                return Err(limit_error(invocation.span));
            }
            let condition = self.append(Expression::Load { pointer }, invocation.span)?;
            self.parameter_expansion_budget
                .charge_nodes(1, invocation.span)?;
            self.parameter_expansion_budget
                .charge_slice::<(Statement, Span)>(1, invocation.span)?;
            let mut accept = crate::Block::new();
            accept.push(Statement::Break, invocation.span);
            self.generated_statement(
                Statement::If {
                    condition,
                    accept,
                    reject: crate::Block::new(),
                },
                invocation.span,
            )?;
        }
        Ok(())
    }

    pub(super) fn clone_parameter_block(
        &mut self,
        body: &crate::Block,
        invocation: &mut Invocation<'_>,
        depth: u32,
        return_mode: ReturnMode,
    ) -> Result<bool, ParseError> {
        if depth > 127 {
            return Err(limit_error(invocation.span));
        }
        let mut has_return = false;
        for (statement, span) in body.span_iter() {
            let span = *span;
            self.parameter_expansion_budget.charge_work(1, span)?;
            match *statement {
                Statement::Emit(ref range) => {
                    for expression in range.clone() {
                        self.invocation_expression(invocation, expression)?;
                    }
                    self.flush_emitter();
                }
                Statement::Block(ref body) => {
                    let source_call = invocation.source_state.and_then(|state| {
                        deferred_call_marker(body)
                            .and_then(|marker| state.deferred_calls.get(&marker))
                    });
                    if let Some(call) = source_call {
                        self.clone_parameter_call(call, invocation, depth)?;
                        if let Some((&Statement::Return { value: None }, return_span)) =
                            body.span_iter().nth(1)
                        {
                            self.clone_parameter_return(
                                invocation,
                                None,
                                return_mode,
                                *return_span,
                            )?;
                            has_return = true;
                        }
                        continue;
                    }
                    let (returned, body) = self.capture_block(|parser| {
                        parser.clone_parameter_block(body, invocation, depth + 1, return_mode)
                    })?;
                    has_return |= returned;
                    self.generated_statement(Statement::Block(body), span)?;
                }
                Statement::If {
                    condition,
                    ref accept,
                    ref reject,
                } => {
                    let condition = self.invocation_value(invocation, condition)?;
                    let checkpoint = invocation.mapped_handles.len();
                    let (accept_return, accept) = self.capture_block(|parser| {
                        parser.clone_parameter_block(accept, invocation, depth + 1, return_mode)
                    })?;
                    if invocation.source_state.is_some() {
                        self.restore_invocation_expressions(invocation, checkpoint)?;
                    }
                    let (reject_return, reject) = self.capture_block(|parser| {
                        parser.clone_parameter_block(reject, invocation, depth + 1, return_mode)
                    })?;
                    if invocation.source_state.is_some() {
                        self.restore_invocation_expressions(invocation, checkpoint)?;
                    }
                    has_return |= accept_return || reject_return;
                    self.generated_statement(
                        Statement::If {
                            condition,
                            accept,
                            reject,
                        },
                        span,
                    )?;
                }
                Statement::Switch {
                    selector,
                    ref cases,
                } => {
                    let selector = self.invocation_value(invocation, selector)?;
                    self.parameter_expansion_budget
                        .charge_slice::<crate::SwitchCase>(cases.len(), span)?;
                    self.parameter_expansion_budget
                        .charge_nodes(cases.len(), span)?;
                    let mut mapped = Vec::with_capacity(cases.len());
                    let mut switch_return = false;
                    let checkpoint = invocation.mapped_handles.len();
                    for (case_index, case) in cases.iter().enumerate() {
                        let (returned, body) = self.capture_block(|parser| {
                            let mut has_return = false;
                            for following_case in &cases[case_index..] {
                                parser.parameter_expansion_budget.charge_work(1, span)?;
                                has_return |= parser.clone_parameter_block(
                                    &following_case.body,
                                    invocation,
                                    depth + 3,
                                    return_mode,
                                )?;
                                if !following_case.fall_through {
                                    break;
                                }
                            }
                            Ok(has_return)
                        })?;
                        self.restore_invocation_expressions(invocation, checkpoint)?;
                        switch_return |= returned;
                        mapped.push(crate::SwitchCase {
                            value: case.value,
                            body,
                            fall_through: false,
                        });
                    }
                    self.generated_statement(
                        Statement::Switch {
                            selector,
                            cases: mapped,
                        },
                        span,
                    )?;
                    if switch_return {
                        self.propagate_parameter_return(invocation, depth)?;
                    }
                    has_return |= switch_return;
                }
                Statement::Loop {
                    ref body,
                    ref continuing,
                    break_if,
                } => {
                    let checkpoint = invocation.mapped_handles.len();
                    let (body_return, body) = self.capture_block(|parser| {
                        parser.clone_parameter_block(body, invocation, depth + 1, return_mode)
                    })?;
                    if invocation.source_state.is_some() {
                        self.restore_invocation_expressions(invocation, checkpoint)?;
                    }
                    let ((continuing_return, break_if), continuing) =
                        self.capture_block(|parser| {
                            let returned = parser.clone_parameter_block(
                                continuing,
                                invocation,
                                depth + 2,
                                return_mode,
                            )?;
                            let condition = break_if
                                .map(|condition| parser.invocation_value(invocation, condition))
                                .transpose()?;
                            Ok((returned, condition))
                        })?;
                    if invocation.source_state.is_some() {
                        self.restore_invocation_expressions(invocation, checkpoint)?;
                    }
                    self.generated_statement(
                        Statement::Loop {
                            body,
                            continuing,
                            break_if,
                        },
                        span,
                    )?;
                    if body_return || continuing_return {
                        self.propagate_parameter_return(invocation, depth)?;
                    }
                    has_return |= body_return || continuing_return;
                }
                Statement::Store { pointer, value } => {
                    let pointer = self.invocation_expression(invocation, pointer)?;
                    let value = self.invocation_value(invocation, value)?;
                    match pointer {
                        MappedExpression::Value(pointer) => {
                            self.generated_statement(Statement::Store { pointer, value }, span)?
                        }
                        MappedExpression::Reference { binding, ty } => {
                            let mut reference = ReferenceValue {
                                reference: Reference::Bound(binding),
                                writable: true,
                                ty,
                                span,
                                constant: ConstantValue::default(),
                            };
                            self.decorate_reference(
                                &mut reference,
                                &mut |parser, pointer, _, span| {
                                    parser.generated_statement(
                                        Statement::Store { pointer, value },
                                        span,
                                    )
                                },
                            )?;
                        }
                        MappedExpression::Pointer(_) => return Err(expansion_error(span)),
                    }
                }
                Statement::Call {
                    function,
                    ref arguments,
                    result,
                } => {
                    self.parameter_expansion_budget
                        .charge_slice::<Handle<Expression>>(arguments.len(), span)?;
                    let mut mapped = Vec::with_capacity(arguments.len());
                    for &argument in arguments {
                        mapped.push(self.invocation_value(invocation, argument)?);
                    }
                    let mapped_result = if let Some(result) = result {
                        let mapped_result = self.append_call_result(function, span)?;
                        self.cache_invocation_expression(
                            invocation,
                            result,
                            MappedExpression::Value(mapped_result),
                        )?;
                        self.name_invocation_expression(
                            invocation.source,
                            result,
                            mapped_result,
                            span,
                        )?;
                        Some(mapped_result)
                    } else {
                        None
                    };
                    self.generated_statement(
                        Statement::Call {
                            function,
                            arguments: mapped,
                            result: mapped_result,
                        },
                        span,
                    )?;
                }
                Statement::Return { value } => {
                    has_return = true;
                    self.clone_parameter_return(invocation, value, return_mode, span)?;
                }
                Statement::Break => self.generated_statement(Statement::Break, span)?,
                Statement::Continue => self.generated_statement(Statement::Continue, span)?,
                Statement::ControlBarrier(flags) => {
                    self.generated_statement(Statement::ControlBarrier(flags), span)?
                }
                Statement::MemoryBarrier(flags) => {
                    self.generated_statement(Statement::MemoryBarrier(flags), span)?
                }
                _ => return Err(expansion_error(span)),
            }
        }
        Ok(has_return)
    }

    fn clone_parameter_return(
        &mut self,
        invocation: &mut Invocation<'_>,
        value: Option<Handle<Expression>>,
        return_mode: ReturnMode,
        span: Span,
    ) -> Result<(), ParseError> {
        let value = value
            .map(|value| self.invocation_value(invocation, value))
            .transpose()?;
        match return_mode {
            ReturnMode::Preserve => self.generated_statement(Statement::Return { value }, span),
            ReturnMode::Inline { tail_only } => {
                if let Some(value) = value {
                    let pointer = invocation.result.ok_or_else(|| expansion_error(span))?;
                    self.generated_statement(Statement::Store { pointer, value }, span)?;
                }
                if !tail_only {
                    let pointer = invocation.returned.ok_or_else(|| expansion_error(span))?;
                    let value = self.append(Expression::Literal(Literal::Bool(true)), span)?;
                    self.generated_statement(Statement::Store { pointer, value }, span)?;
                    self.generated_statement(Statement::Break, span)?;
                }
                Ok(())
            }
        }
    }

    fn clone_parameter_call(
        &mut self,
        call: &DeferredCall,
        invocation: &mut Invocation<'_>,
        depth: u32,
    ) -> Result<(), ParseError> {
        let state = invocation
            .source_state
            .ok_or_else(|| expansion_error(call.span))?;
        self.parameter_expansion_budget
            .charge_slice::<DeferredArgument>(call.arguments.len(), call.span)?;
        let mut arguments = Vec::with_capacity(call.arguments.len());
        for (index, argument) in call.arguments.iter().enumerate() {
            arguments.push(match *argument {
                DeferredArgument::Value(value) => {
                    DeferredArgument::Value(self.invocation_value(invocation, value)?)
                }
                DeferredArgument::Pointer(ref pointer) => {
                    let pointer = self.remap_pointer_value(state, pointer, invocation)?;
                    DeferredArgument::Pointer(self.boxed_pointer_value(pointer)?)
                }
                DeferredArgument::Reference(binding) => {
                    let parameter = &self.module.functions[call.function].arguments[index];
                    let TypeInner::Pointer { base, .. } = self.module.types[parameter.ty].inner
                    else {
                        return Err(expansion_error(call.span));
                    };
                    let ty = TypeResolution::Handle(base);
                    let reference =
                        self.remap_bound_reference(state, binding, &ty, invocation, call.span)?;
                    DeferredArgument::Reference(self.freeze_reference(reference)?)
                }
            });
        }
        let result_pointer = call
            .result_pointer
            .map(|pointer| self.invocation_value(invocation, pointer))
            .transpose()?;
        self.inline_parameter_call(
            DeferredCall {
                function: call.function,
                arguments,
                result_pointer,
                span: call.span,
            },
            depth,
        )
    }
}
