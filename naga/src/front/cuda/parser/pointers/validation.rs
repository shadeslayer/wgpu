use super::*;

fn pointer_state_error(span: Span) -> ParseError {
    ParseError {
        message: String::from("invalid CUDA source pointer state"),
        span,
    }
}

fn pointer_depth_error(span: Span) -> ParseError {
    ParseError {
        message: String::from("CUDA pointer expression nesting limit exceeded"),
        span,
    }
}

#[derive(Clone, Copy)]
enum PointerDependency {
    Expression(Handle<PointerExpression>),
    Access(Handle<PointerAccess>),
    Reference(Handle<BoundReference>),
    Address(Handle<BoundAddress>),
}

struct PointerDepths {
    expressions: Vec<u32>,
    accesses: Vec<u32>,
    references: Vec<u32>,
    addresses: Vec<u32>,
}

impl PointerDepths {
    fn get(&self, node: PointerDependency) -> u32 {
        match node {
            PointerDependency::Expression(handle) => self.expressions[handle.index()],
            PointerDependency::Access(handle) => self.accesses[handle.index()],
            PointerDependency::Reference(handle) => self.references[handle.index()],
            PointerDependency::Address(handle) => self.addresses[handle.index()],
        }
    }

    fn set(&mut self, node: PointerDependency, value: u32) {
        let depth = match node {
            PointerDependency::Expression(handle) => &mut self.expressions[handle.index()],
            PointerDependency::Access(handle) => &mut self.accesses[handle.index()],
            PointerDependency::Reference(handle) => &mut self.references[handle.index()],
            PointerDependency::Address(handle) => &mut self.addresses[handle.index()],
        };
        *depth = value;
    }
}

fn pointer_dependencies(
    state: &FunctionParameterState,
    function: &Function,
    node: PointerDependency,
    mut visit: impl FnMut(PointerDependency) -> Result<(), ParseError>,
) -> Result<(), ParseError> {
    match node {
        PointerDependency::Expression(handle) => match state.pointers.expressions[handle].kind {
            PointerExpressionKind::Parameter(_) => {}
            PointerExpressionKind::Array(binding) | PointerExpressionKind::Address(binding) => {
                visit(PointerDependency::Reference(binding))?;
            }
            PointerExpressionKind::Offset { base, .. } => {
                visit(PointerDependency::Expression(base))?
            }
            PointerExpressionKind::Select { accept, reject, .. } => {
                visit(PointerDependency::Expression(accept))?;
                visit(PointerDependency::Expression(reject))?;
            }
        },
        PointerDependency::Access(handle) => {
            visit(PointerDependency::Expression(
                state.pointers.accesses[handle].pointer.expression,
            ))?;
        }
        PointerDependency::Reference(handle) => {
            for target in &state.bound_references[handle].targets {
                visit(PointerDependency::Address(target.address))?;
            }
        }
        PointerDependency::Address(handle) => {
            if let Expression::LocalVariable(local) =
                function.expressions[state.bound_addresses[handle].root]
            {
                if let Some(&access) = state.pointers.validation_addresses.get(&local) {
                    visit(PointerDependency::Access(access))?;
                }
            }
        }
    }
    Ok(())
}

fn captured_pointer(
    function: &Function,
    handle: Handle<Expression>,
    module: &Module,
    span: Span,
) -> Result<(), ParseError> {
    let expression = function
        .expressions
        .try_get(handle)
        .map_err(|_| pointer_state_error(span))?;
    let Expression::LocalVariable(local) = *expression else {
        return Err(pointer_state_error(span));
    };
    let local = function
        .local_variables
        .try_get(local)
        .map_err(|_| pointer_state_error(span))?;
    if module.types[local.ty].inner != TypeInner::Scalar(Scalar::U32) {
        return Err(pointer_state_error(span));
    }
    Ok(())
}

fn mark_pointer_expression(
    live: &mut [bool],
    handle: Handle<Expression>,
    before: Option<Handle<Expression>>,
    span: Span,
) -> Result<(), ParseError> {
    if before.is_some_and(|before| handle.index() >= before.index()) {
        return Err(pointer_state_error(span));
    }
    let used = live
        .get_mut(handle.index())
        .ok_or_else(|| pointer_state_error(span))?;
    *used = true;
    Ok(())
}

impl Parser<'_> {
    pub(in super::super) fn inventory_pointer_state(
        &mut self,
        state: &PointerState,
        span: Span,
    ) -> Result<(), ParseError> {
        let budget = &mut self.parameter_expansion_budget;
        budget.charge_bytes(size_of::<PointerState>(), span)?;
        budget.charge_nodes(state.expressions.len() + state.accesses.len(), span)?;
        budget.charge_slice::<(PointerExpression, Span)>(state.expressions.len(), span)?;
        budget.charge_slice::<(PointerAccess, Span)>(state.accesses.len(), span)?;
        budget.charge_work(state.expressions.len() + state.accesses.len(), span)?;
        for (_, access) in state.accesses.iter() {
            budget.charge_slice::<AddressProjection>(access.projections.len(), span)?;
            budget.charge_work(access.projections.len(), span)?;
        }
        budget.charge_map_entry::<Handle<LocalVariable>, Handle<PointerAccess>>(
            state.validation_addresses.len(),
            span,
        )?;
        budget.charge_map_entry::<Handle<PointerAccess>, Handle<LocalVariable>>(
            state.access_addresses.len(),
            span,
        )?;
        budget.charge_work(
            state.validation_addresses.len() + state.access_addresses.len(),
            span,
        )
    }

    fn pointer_projection_type(
        &self,
        ty: &TypeResolution,
        projection: &AddressProjection,
        span: Span,
    ) -> Result<TypeResolution, ParseError> {
        match *ty.inner_with(&self.module.types) {
            TypeInner::Array { base, size, .. } => {
                if let (&AddressProjection::Field(index), ArraySize::Constant(length)) =
                    (projection, size)
                {
                    if index >= length.get() {
                        return Err(pointer_state_error(span));
                    }
                }
                Ok(TypeResolution::Handle(base))
            }
            TypeInner::Struct { ref members, .. } => {
                let AddressProjection::Field(index) = *projection else {
                    return Err(pointer_state_error(span));
                };
                members
                    .get(index as usize)
                    .map(|member| TypeResolution::Handle(member.ty))
                    .ok_or_else(|| pointer_state_error(span))
            }
            TypeInner::Vector { size, scalar } => {
                if let AddressProjection::Field(index) = *projection {
                    if index >= size as u32 {
                        return Err(pointer_state_error(span));
                    }
                }
                Ok(scalar_resolution(scalar))
            }
            _ => Err(pointer_state_error(span)),
        }
    }

    fn source_bound_address_type(
        &mut self,
        state: &FunctionParameterState,
        function: &Function,
        handle: Handle<BoundAddress>,
        span: Span,
    ) -> Result<TypeResolution, ParseError> {
        let address = state
            .bound_addresses
            .try_get(handle)
            .map_err(|_| pointer_state_error(span))?;
        self.parameter_expansion_budget
            .charge_work(address.projections.len() + 1, span)?;
        let root = function
            .expressions
            .try_get(address.root)
            .map_err(|_| pointer_state_error(span))?;
        let ty = match *root {
            Expression::LocalVariable(local) => function
                .local_variables
                .try_get(local)
                .map(|local| local.ty)
                .map_err(|_| pointer_state_error(span))?,
            Expression::GlobalVariable(global) => self
                .module
                .global_variables
                .try_get(global)
                .map(|global| global.ty)
                .map_err(|_| pointer_state_error(span))?,
            Expression::FunctionArgument(index) => {
                let argument = function
                    .arguments
                    .get(index as usize)
                    .ok_or_else(|| pointer_state_error(span))?;
                let TypeInner::Pointer { base, .. } = self.module.types[argument.ty].inner else {
                    return Err(pointer_state_error(span));
                };
                base
            }
            _ => return Err(pointer_state_error(span)),
        };
        let mut ty = TypeResolution::Handle(ty);
        for projection in &address.projections {
            if let AddressProjection::Index(captured) = *projection {
                captured_pointer(function, captured, &self.module, span)?;
            }
            ty = self.pointer_projection_type(&ty, projection, span)?;
        }
        Ok(ty)
    }

    fn validate_source_pointer_value(
        &mut self,
        state: &PointerState,
        value: &PointerValue,
    ) -> Result<(), ParseError> {
        self.parameter_expansion_budget.charge_work(1, value.span)?;
        let expression = state
            .expressions
            .try_get(value.expression)
            .map_err(|_| pointer_state_error(value.span))?;
        let TypeInner::Pointer {
            base,
            space: AddressSpace::Function,
        } = *value.ty.inner_with(&self.module.types)
        else {
            return Err(pointer_state_error(value.span));
        };
        if base != expression.pointee {
            return Err(pointer_state_error(value.span));
        }
        if let Some(address) = value.constant_address {
            if address.extent == 0
                || address.offset < 0
                || address.offset > i64::from(address.extent)
            {
                return Err(pointer_state_error(value.span));
            }
        }
        Ok(())
    }

    pub(in super::super) fn validate_pointer_state(
        &mut self,
        state: &FunctionParameterState,
        function: &Function,
        span: Span,
    ) -> Result<(), ParseError> {
        if state.pointers.is_empty() {
            return Ok(());
        }
        for (handle, expression) in state.pointers.expressions.iter() {
            let span = state.pointers.expressions.get_span(handle);
            self.parameter_expansion_budget.charge_work(1, span)?;
            self.module
                .types
                .check_contains_handle(expression.pointee)
                .map_err(|_| pointer_state_error(span))?;
            let expected_depth = match expression.kind {
                PointerExpressionKind::Parameter(index) => {
                    let argument = function
                        .arguments
                        .get(index as usize)
                        .ok_or_else(|| pointer_state_error(span))?;
                    if !matches!(self.module.types[argument.ty].inner,
                        TypeInner::Pointer { base, space: AddressSpace::Function } if base == expression.pointee)
                    {
                        return Err(pointer_state_error(span));
                    }
                    1
                }
                PointerExpressionKind::Array(binding) | PointerExpressionKind::Address(binding) => {
                    let reference = state
                        .bound_references
                        .try_get(binding)
                        .map_err(|_| pointer_state_error(span))?;
                    for target in &reference.targets {
                        let ty =
                            self.source_bound_address_type(state, function, target.address, span)?;
                        let pointee = match expression.kind {
                            PointerExpressionKind::Array(_) => {
                                match *ty.inner_with(&self.module.types) {
                                    TypeInner::Array { base, .. } => base,
                                    _ => return Err(pointer_state_error(span)),
                                }
                            }
                            _ => {
                                if !self
                                    .module
                                    .compare_types(&ty, &TypeResolution::Handle(expression.pointee))
                                {
                                    return Err(pointer_state_error(span));
                                }
                                expression.pointee
                            }
                        };
                        if pointee != expression.pointee {
                            return Err(pointer_state_error(span));
                        }
                    }
                    1
                }
                PointerExpressionKind::Offset { base, offset, .. } => {
                    captured_pointer(function, offset, &self.module, span)?;
                    let base = state
                        .pointers
                        .expressions
                        .try_get(base)
                        .map_err(|_| pointer_state_error(span))?;
                    if base.pointee != expression.pointee {
                        return Err(pointer_state_error(span));
                    }
                    base.depth
                        .checked_add(1)
                        .ok_or_else(|| pointer_state_error(span))?
                }
                PointerExpressionKind::Select {
                    selector,
                    accept,
                    reject,
                } => {
                    captured_pointer(function, selector, &self.module, span)?;
                    let accept = state
                        .pointers
                        .expressions
                        .try_get(accept)
                        .map_err(|_| pointer_state_error(span))?;
                    let reject = state
                        .pointers
                        .expressions
                        .try_get(reject)
                        .map_err(|_| pointer_state_error(span))?;
                    if accept.pointee != expression.pointee || reject.pointee != expression.pointee
                    {
                        return Err(pointer_state_error(span));
                    }
                    accept
                        .depth
                        .max(reject.depth)
                        .checked_add(1)
                        .ok_or_else(|| pointer_state_error(span))?
                }
            };
            if expected_depth > 64 {
                return Err(pointer_depth_error(span));
            }
            if expression.depth != expected_depth {
                return Err(pointer_state_error(span));
            }
        }
        for (handle, access) in state.pointers.accesses.iter() {
            let span = state.pointers.accesses.get_span(handle);
            self.validate_source_pointer_value(&state.pointers, &access.pointer)?;
            self.parameter_expansion_budget
                .charge_work(access.projections.len(), span)?;
            let mut ty = TypeResolution::Handle(
                state.pointers.expressions[access.pointer.expression].pointee,
            );
            for projection in &access.projections {
                if let AddressProjection::Index(captured) = *projection {
                    captured_pointer(function, captured, &self.module, span)?;
                }
                ty = self.pointer_projection_type(&ty, projection, span)?;
            }
            if !self.module.compare_types(&ty, &access.ty) {
                return Err(pointer_state_error(span));
            }
        }
        self.parameter_expansion_budget.charge_work(
            state.pointers.validation_addresses.len() + state.pointers.access_addresses.len(),
            span,
        )?;
        for (&local, &handle) in &state.pointers.validation_addresses {
            let access = state
                .pointers
                .accesses
                .try_get(handle)
                .map_err(|_| pointer_state_error(span))?;
            let variable = function
                .local_variables
                .try_get(local)
                .map_err(|_| pointer_state_error(span))?;
            if variable.init.is_some()
                || !self
                    .module
                    .compare_types(&TypeResolution::Handle(variable.ty), &access.ty)
                || state.pointers.access_addresses.get(&handle) != Some(&local)
            {
                return Err(pointer_state_error(span));
            }
        }
        for (&handle, &local) in &state.pointers.access_addresses {
            if state.pointers.validation_addresses.get(&local) != Some(&handle) {
                return Err(pointer_state_error(span));
            }
        }
        for (handle, _) in state.bound_addresses.iter() {
            self.source_bound_address_type(state, function, handle, span)?;
        }
        for (_, reference) in state.bound_references.iter() {
            self.parameter_expansion_budget
                .charge_work(reference.targets.len() + 1, span)?;
            if reference.targets.is_empty()
                || (reference.selector.is_none() && reference.targets.len() != 1)
            {
                return Err(pointer_state_error(span));
            }
            if let Some(selector) = reference.selector {
                captured_pointer(function, selector, &self.module, span)?;
            }
            for target in &reference.targets {
                state
                    .bound_addresses
                    .check_contains_handle(target.address)
                    .map_err(|_| pointer_state_error(span))?;
            }
        }
        for call in state.deferred_calls.values() {
            self.parameter_expansion_budget
                .charge_work(call.arguments.len() + 1, call.span)?;
            for argument in &call.arguments {
                match *argument {
                    parameter_calls::DeferredArgument::Pointer(ref pointer) => {
                        self.validate_source_pointer_value(&state.pointers, pointer)?
                    }
                    parameter_calls::DeferredArgument::Reference(binding) => state
                        .bound_references
                        .check_contains_handle(binding)
                        .map_err(|_| pointer_state_error(call.span))?,
                    parameter_calls::DeferredArgument::Value(value) => function
                        .expressions
                        .check_contains_handle(value)
                        .map_err(|_| pointer_state_error(call.span))?,
                }
            }
        }
        self.validate_pointer_dependency_depth(state, function, span)
    }

    fn validate_pointer_dependency_depth(
        &mut self,
        state: &FunctionParameterState,
        function: &Function,
        span: Span,
    ) -> Result<(), ParseError> {
        let count = state.pointers.expressions.len()
            + state.pointers.accesses.len()
            + state.bound_references.len()
            + state.bound_addresses.len();
        self.parameter_expansion_budget
            .charge_slice::<u32>(count, span)?;
        let mut depths = PointerDepths {
            expressions: alloc::vec![0; state.pointers.expressions.len()],
            accesses: alloc::vec![0; state.pointers.accesses.len()],
            references: alloc::vec![0; state.bound_references.len()],
            addresses: alloc::vec![0; state.bound_addresses.len()],
        };
        let mut pending = Vec::new();
        for root in state
            .pointers
            .expressions
            .iter()
            .map(|(handle, _)| PointerDependency::Expression(handle))
            .chain(
                state
                    .pointers
                    .accesses
                    .iter()
                    .map(|(handle, _)| PointerDependency::Access(handle)),
            )
        {
            self.parameter_expansion_budget
                .charge_slice::<(PointerDependency, bool)>(1, span)?;
            pending.push((root, false));
            while let Some((node, finish)) = pending.pop() {
                self.parameter_expansion_budget.charge_work(1, span)?;
                if finish {
                    let mut maximum = 0;
                    pointer_dependencies(state, function, node, |dependency| {
                        self.parameter_expansion_budget.charge_work(1, span)?;
                        let depth = depths.get(dependency);
                        if depth == 0 || depth == u32::MAX {
                            return Err(pointer_state_error(span));
                        }
                        maximum = maximum.max(depth - 1);
                        Ok(())
                    })?;
                    let weight = u32::from(matches!(
                        node,
                        PointerDependency::Expression(_) | PointerDependency::Access(_)
                    ));
                    let depth = maximum + weight;
                    if depth > 64 {
                        return Err(pointer_depth_error(span));
                    }
                    depths.set(node, depth + 1);
                } else {
                    match depths.get(node) {
                        u32::MAX => return Err(pointer_state_error(span)),
                        0 => {}
                        _ => continue,
                    }
                    depths.set(node, u32::MAX);
                    self.parameter_expansion_budget
                        .charge_slice::<(PointerDependency, bool)>(1, span)?;
                    pending.push((node, true));
                    pointer_dependencies(state, function, node, |dependency| {
                        self.parameter_expansion_budget.charge_work(1, span)?;
                        self.parameter_expansion_budget
                            .charge_slice::<(PointerDependency, bool)>(1, span)?;
                        pending.push((dependency, false));
                        Ok(())
                    })?;
                }
            }
        }
        Ok(())
    }

    pub(in super::super) fn check_no_live_pointer_accesses(
        &mut self,
        function: &Function,
        state: &FunctionParameterState,
        span: Span,
    ) -> Result<(), ParseError> {
        if state.pointers.validation_addresses.is_empty() {
            return Ok(());
        }
        self.parameter_expansion_budget
            .charge_slice::<bool>(function.expressions.len(), span)?;
        let mut live = alloc::vec![false; function.expressions.len()];
        self.parameter_expansion_budget
            .charge_slice::<&crate::Block>(1, span)?;
        let mut pending = alloc::vec![&function.body];
        for (_, local) in function.local_variables.iter() {
            self.parameter_expansion_budget.charge_work(1, span)?;
            if let Some(initial) = local.init {
                mark_pointer_expression(&mut live, initial, None, span)?;
            }
        }
        while let Some(block) = pending.pop() {
            for (statement, statement_span) in block.span_iter() {
                let span = *statement_span;
                self.parameter_expansion_budget.charge_work(1, span)?;
                let mut child = |body| -> Result<(), ParseError> {
                    self.parameter_expansion_budget
                        .charge_slice::<&crate::Block>(1, span)?;
                    pending.push(body);
                    Ok(())
                };
                match *statement {
                    Statement::Emit(ref range) => {
                        self.parameter_expansion_budget
                            .charge_work(range.clone().count(), span)?;
                        for expression in range.clone() {
                            mark_pointer_expression(&mut live, expression, None, span)?;
                        }
                    }
                    Statement::Block(ref body) => child(body)?,
                    Statement::If {
                        condition,
                        ref accept,
                        ref reject,
                    } => {
                        mark_pointer_expression(&mut live, condition, None, span)?;
                        child(accept)?;
                        child(reject)?;
                    }
                    Statement::Switch {
                        selector,
                        ref cases,
                    } => {
                        mark_pointer_expression(&mut live, selector, None, span)?;
                        for case in cases {
                            child(&case.body)?;
                        }
                    }
                    Statement::Loop {
                        ref body,
                        ref continuing,
                        break_if,
                    } => {
                        child(body)?;
                        child(continuing)?;
                        if let Some(condition) = break_if {
                            mark_pointer_expression(&mut live, condition, None, span)?;
                        }
                    }
                    Statement::Store { pointer, value } => {
                        mark_pointer_expression(&mut live, pointer, None, span)?;
                        mark_pointer_expression(&mut live, value, None, span)?;
                    }
                    Statement::Call {
                        ref arguments,
                        result,
                        ..
                    } => {
                        self.parameter_expansion_budget
                            .charge_work(arguments.len(), span)?;
                        for &argument in arguments {
                            mark_pointer_expression(&mut live, argument, None, span)?;
                        }
                        if let Some(result) = result {
                            mark_pointer_expression(&mut live, result, None, span)?;
                        }
                    }
                    Statement::Return { value } => {
                        if let Some(value) = value {
                            mark_pointer_expression(&mut live, value, None, span)?;
                        }
                    }
                    Statement::Break
                    | Statement::Continue
                    | Statement::ControlBarrier(_)
                    | Statement::MemoryBarrier(_) => {}
                    _ => return Err(pointer_state_error(span)),
                }
            }
        }
        self.parameter_expansion_budget
            .charge_work(function.expressions.len(), span)?;
        for (handle, expression) in function.expressions.iter().rev() {
            if !live[handle.index()] {
                continue;
            }
            let span = function.expressions.get_span(handle);
            let mut operand = |value| -> Result<(), ParseError> {
                self.parameter_expansion_budget.charge_work(1, span)?;
                mark_pointer_expression(&mut live, value, Some(handle), span)
            };
            match *expression {
                Expression::LocalVariable(local) => {
                    if state.pointers.validation_addresses.contains_key(&local) {
                        return Err(ParseError {
                            message: String::from(
                                "unresolved CUDA pointer access in retained function",
                            ),
                            span,
                        });
                    }
                }
                Expression::Literal(_)
                | Expression::Constant(_)
                | Expression::Override(_)
                | Expression::ZeroValue(_)
                | Expression::FunctionArgument(_)
                | Expression::GlobalVariable(_)
                | Expression::CallResult(_) => {}
                Expression::Compose { ref components, .. } => {
                    for &component in components {
                        operand(component)?;
                    }
                }
                Expression::Access { base, index } => {
                    operand(base)?;
                    operand(index)?;
                }
                Expression::AccessIndex { base, .. } => operand(base)?,
                Expression::Load { pointer } => operand(pointer)?,
                Expression::Unary { expr, .. } | Expression::As { expr, .. } => operand(expr)?,
                Expression::Binary { left, right, .. } => {
                    operand(left)?;
                    operand(right)?;
                }
                Expression::Select {
                    condition,
                    accept,
                    reject,
                } => {
                    operand(condition)?;
                    operand(accept)?;
                    operand(reject)?;
                }
                Expression::Math {
                    arg,
                    arg1,
                    arg2,
                    arg3,
                    ..
                } => {
                    operand(arg)?;
                    for argument in [arg1, arg2, arg3].into_iter().flatten() {
                        operand(argument)?;
                    }
                }
                Expression::Relational { argument, .. } => operand(argument)?,
                Expression::Splat { value, .. } => operand(value)?,
                Expression::Swizzle { vector, .. } => operand(vector)?,
                _ => return Err(pointer_state_error(span)),
            }
        }
        Ok(())
    }
}
