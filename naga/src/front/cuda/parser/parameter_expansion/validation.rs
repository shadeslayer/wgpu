use super::*;

impl ExpansionBudget {
    pub(super) fn check_control_depth(
        &mut self,
        root: &crate::Block,
        initial_depth: u32,
        span: Span,
    ) -> Result<(), ParseError> {
        self.charge_slice::<(&crate::Block, u32)>(1, span)?;
        let mut pending = alloc::vec![(root, initial_depth)];
        while let Some((block, depth)) = pending.pop() {
            if depth > 127 {
                return Err(limit_error(span));
            }
            for statement in block {
                self.charge_work(1, span)?;
                let mut child = |body, weight| -> Result<(), ParseError> {
                    self.charge_slice::<(&crate::Block, u32)>(1, span)?;
                    pending.push((body, depth + weight));
                    Ok(())
                };
                match *statement {
                    Statement::Block(ref body) => child(body, 1)?,
                    Statement::If {
                        ref accept,
                        ref reject,
                        ..
                    } => {
                        child(accept, 1)?;
                        child(reject, 1)?;
                    }
                    Statement::Switch { ref cases, .. } => {
                        for case in cases {
                            child(&case.body, 3)?;
                        }
                    }
                    Statement::Loop {
                        ref body,
                        ref continuing,
                        ..
                    } => {
                        child(body, 1)?;
                        child(continuing, 2)?;
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
    fn name(&mut self, name: Option<&str>, span: Span) -> Result<(), ParseError> {
        self.charge_bytes(name.map_or(0, str::len), span)
    }

    fn expression(&mut self, expression: &Expression, span: Span) -> Result<(), ParseError> {
        self.charge_work(1, span)?;
        if let Expression::Compose { ref components, .. } = *expression {
            self.charge_slice::<Handle<Expression>>(components.len(), span)?;
        }
        Ok(())
    }

    pub(super) fn inventory_function(
        &mut self,
        function: &Function,
        span: Span,
        initial_depth: u32,
    ) -> Result<(), ParseError> {
        self.name(function.name.as_deref(), span)?;
        self.charge_nodes(
            function.expressions.len() + function.local_variables.len() + function.arguments.len(),
            span,
        )?;
        self.charge_slice::<(Expression, Span)>(function.expressions.len(), span)?;
        self.charge_slice::<(LocalVariable, Span)>(function.local_variables.len(), span)?;
        self.charge_slice::<FunctionArgument>(function.arguments.len(), span)?;
        self.charge_map_entry::<Handle<Expression>, String>(
            function.named_expressions.len(),
            span,
        )?;
        for argument in &function.arguments {
            self.name(argument.name.as_deref(), span)?;
        }
        for (_, local) in function.local_variables.iter() {
            self.name(local.name.as_deref(), span)?;
        }
        for (_, name) in &function.named_expressions {
            self.name(Some(name), span)?;
        }
        for (handle, expression) in function.expressions.iter() {
            self.expression(expression, function.expressions.get_span(handle))?;
        }
        self.inventory_block(&function.body, initial_depth, span)
    }

    pub(super) fn inventory_block(
        &mut self,
        root: &crate::Block,
        initial_depth: u32,
        span: Span,
    ) -> Result<(), ParseError> {
        self.charge_slice::<(&crate::Block, u32)>(1, span)?;
        let mut pending = alloc::vec![(root, initial_depth)];
        while let Some((block, depth)) = pending.pop() {
            if depth > 127 {
                return Err(limit_error(span));
            }
            self.charge_nodes(block.len(), span)?;
            self.charge_slice::<(Statement, Span)>(block.len(), span)?;
            for (statement, statement_span) in block.span_iter() {
                self.charge_work(1, *statement_span)?;
                let mut child = |body, weight| -> Result<(), ParseError> {
                    self.charge_slice::<(&crate::Block, u32)>(1, *statement_span)?;
                    pending.push((body, depth + weight));
                    Ok(())
                };
                match *statement {
                    Statement::Block(ref body) => child(body, 1)?,
                    Statement::If {
                        ref accept,
                        ref reject,
                        ..
                    } => {
                        child(accept, 1)?;
                        child(reject, 1)?;
                    }
                    Statement::Switch { ref cases, .. } => {
                        for case in cases {
                            child(&case.body, 3)?;
                        }
                        self.charge_slice::<crate::SwitchCase>(cases.len(), *statement_span)?;
                        self.charge_nodes(cases.len(), *statement_span)?;
                    }
                    Statement::Loop {
                        ref body,
                        ref continuing,
                        ..
                    } => {
                        child(body, 1)?;
                        child(continuing, 2)?;
                    }
                    Statement::Call { ref arguments, .. } => {
                        self.charge_slice::<Handle<Expression>>(arguments.len(), *statement_span)?;
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    pub(super) fn inventory_module(
        &mut self,
        module: &Module,
        span: Span,
    ) -> Result<(), ParseError> {
        self.charge_bytes(size_of::<Module>(), span)?;
        self.charge_map_entry::<Type, Span>(module.types.len(), span)?;
        self.charge_slice::<(crate::Constant, Span)>(module.constants.len(), span)?;
        self.charge_slice::<(crate::Override, Span)>(module.overrides.len(), span)?;
        self.charge_slice::<(GlobalVariable, Span)>(module.global_variables.len(), span)?;
        self.charge_slice::<(Expression, Span)>(module.global_expressions.len(), span)?;
        self.charge_slice::<(Function, Span)>(module.functions.len(), span)?;
        self.charge_slice::<EntryPoint>(module.entry_points.len(), span)?;
        self.charge_nodes(
            module.types.len()
                + module.constants.len()
                + module.global_variables.len()
                + module.global_expressions.len()
                + module.functions.len(),
            span,
        )?;
        for (_, ty) in module.types.iter() {
            self.charge_work(1, span)?;
            self.name(ty.name.as_deref(), span)?;
            if let TypeInner::Struct { ref members, .. } = ty.inner {
                self.charge_slice::<StructMember>(members.len(), span)?;
                self.charge_nodes(members.len(), span)?;
                for member in members {
                    self.charge_work(1, span)?;
                    self.name(member.name.as_deref(), span)?;
                }
            }
        }
        for (_, constant) in module.constants.iter() {
            self.name(constant.name.as_deref(), span)?;
        }
        for (_, global) in module.global_variables.iter() {
            self.name(global.name.as_deref(), span)?;
        }
        for (_, expression) in module.global_expressions.iter() {
            self.expression(expression, span)?;
        }
        for (handle, function) in module.functions.iter() {
            self.inventory_function(function, module.functions.get_span(handle), 1)?;
        }
        for entry in &module.entry_points {
            self.name(Some(&entry.name), span)?;
            self.inventory_function(&entry.function, span, 1)?;
        }
        Ok(())
    }
}

impl Parser<'_> {
    pub(super) fn inventory_parameter_state(
        &mut self,
        state: &FunctionParameterState,
        span: Span,
    ) -> Result<(), ParseError> {
        let budget = &mut self.parameter_expansion_budget;
        budget.charge_bytes(size_of::<FunctionParameterState>(), span)?;
        budget.charge_slice::<u32>(state.expression_graph_depths.len(), span)?;
        budget.charge_work(state.expression_graph_depths.len(), span)?;
        budget.charge_map_entry::<Handle<Expression>, DeferredCall>(
            state.deferred_calls.len(),
            span,
        )?;
        budget.charge_work(state.deferred_calls.len(), span)?;
        for call in state.deferred_calls.values() {
            budget.charge_slice::<DeferredArgument>(call.arguments.len(), span)?;
            budget.charge_work(call.arguments.len(), span)?;
            for argument in &call.arguments {
                if matches!(*argument, DeferredArgument::Pointer(_)) {
                    budget.charge_slice::<PointerValue>(1, span)?;
                }
            }
        }
        budget.charge_nodes(
            state.bound_addresses.len() + state.bound_references.len(),
            span,
        )?;
        budget.charge_slice::<(BoundAddress, Span)>(state.bound_addresses.len(), span)?;
        budget.charge_work(state.bound_addresses.len(), span)?;
        for (_, address) in state.bound_addresses.iter() {
            budget.charge_slice::<reference_bindings::AddressProjection>(
                address.projections.len(),
                span,
            )?;
            budget.charge_work(address.projections.len(), span)?;
        }
        budget.charge_map_entry::<BoundAddress, Handle<BoundAddress>>(
            state.bound_address_indices.len(),
            span,
        )?;
        budget.charge_work(state.bound_address_indices.len(), span)?;
        for address in state.bound_address_indices.keys() {
            budget.charge_slice::<reference_bindings::AddressProjection>(
                address.projections.len(),
                span,
            )?;
            budget.charge_work(address.projections.len(), span)?;
        }
        budget.charge_slice::<(BoundReference, Span)>(state.bound_references.len(), span)?;
        budget.charge_work(state.bound_references.len(), span)?;
        for (_, reference) in state.bound_references.iter() {
            budget
                .charge_slice::<reference_bindings::BoundTarget>(reference.targets.len(), span)?;
            budget.charge_work(reference.targets.len(), span)?;
        }
        self.inventory_pointer_state(&state.pointers, span)
    }

    pub(super) fn validate_parameter_templates(
        &mut self,
        order: &[Handle<Function>],
    ) -> Result<(), ParseError> {
        let span = order.first().map_or(Span::default(), |&function| {
            self.module.functions.get_span(function)
        });
        for &handle in order {
            if let Some(state) = self.parameter_states.remove(&handle) {
                let function = core::mem::take(&mut self.module.functions[handle]);
                self.validate_pointer_state(
                    &state,
                    &function,
                    self.module.functions.get_span(handle),
                )?;
                self.module.functions[handle] = function;
                self.parameter_states.insert(handle, state);
            }
        }
        if let Some(state) = self.kernel_parameter_state.take() {
            let function = core::mem::take(&mut self.module.entry_points[0].function);
            self.validate_pointer_state(&state, &function, span)?;
            self.module.entry_points[0].function = function;
            self.kernel_parameter_state = Some(state);
        }
        self.parameter_expansion_budget
            .inventory_module(&self.module, span)?;
        let mut validation = self.module.clone();
        validation.entry_points.clear();
        call_graph::reorder_functions(
            &mut validation,
            order,
            &mut self.parameter_expansion_budget,
            span,
        )?;
        crate::compact::compact(&mut validation, crate::compact::KeepUnused::Yes);
        crate::valid::Validator::new(
            crate::valid::ValidationFlags::all(),
            crate::valid::Capabilities::empty(),
        )
        .validate(&validation)
        .map_err(|error| ParseError {
            span: error.spans().next().map_or(span, |context| context.0),
            message: format!("invalid CUDA device helper: {error}"),
        })?;
        Ok(())
    }
}
