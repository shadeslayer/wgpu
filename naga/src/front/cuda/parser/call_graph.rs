use super::*;

#[derive(Clone, Copy, PartialEq)]
enum VisitState {
    Unvisited,
    Visiting,
    Complete,
}

fn emitted_calls(function: &Function) -> Vec<FunctionCall> {
    let mut calls = Vec::new();
    let mut blocks = alloc::vec![&function.body];
    while let Some(block) = blocks.pop() {
        for (statement, span) in block.span_iter() {
            match *statement {
                Statement::Call { function, .. } => calls.push(FunctionCall {
                    function,
                    span: *span,
                }),
                Statement::Block(ref body) => blocks.push(body),
                Statement::If {
                    ref accept,
                    ref reject,
                    ..
                } => {
                    blocks.push(reject);
                    blocks.push(accept);
                }
                Statement::Switch { ref cases, .. } => {
                    for case in cases.iter().rev() {
                        blocks.push(&case.body);
                    }
                }
                Statement::Loop {
                    ref body,
                    ref continuing,
                    ..
                } => {
                    blocks.push(continuing);
                    blocks.push(body);
                }
                _ => {}
            }
        }
    }
    calls
}

fn remap_calls(function: &mut Function, handles: &[Option<Handle<Function>>]) {
    for (_, expression) in function.expressions.iter_mut() {
        if let Expression::CallResult(ref mut callee) = *expression {
            *callee = handles[callee.index()].unwrap();
        }
    }
    let mut blocks: Vec<&mut [Statement]> = alloc::vec![&mut function.body];
    while let Some(block) = blocks.pop() {
        for statement in block {
            match *statement {
                Statement::Call {
                    ref mut function, ..
                } => {
                    *function = handles[function.index()].unwrap();
                }
                Statement::Block(ref mut body) => blocks.push(body),
                Statement::If {
                    ref mut accept,
                    ref mut reject,
                    ..
                } => {
                    blocks.push(reject);
                    blocks.push(accept);
                }
                Statement::Switch { ref mut cases, .. } => {
                    for case in cases {
                        blocks.push(&mut case.body);
                    }
                }
                Statement::Loop {
                    ref mut body,
                    ref mut continuing,
                    ..
                } => {
                    blocks.push(continuing);
                    blocks.push(body);
                }
                _ => {}
            }
        }
    }
}

impl Parser<'_> {
    pub(super) fn finish_functions(&mut self) -> Result<(), ParseError> {
        let mut retained = alloc::vec![true; self.module.functions.len()];
        let mut dependencies = Vec::with_capacity(self.module.functions.len());
        for (handle, function) in self.module.functions.iter() {
            let mut calls = Vec::new();
            if let Some(declaration) = self.function_declarations.get(&handle) {
                retained[handle.index()] = declaration.is_defined;
                calls.extend_from_slice(&declaration.calls);
            }
            calls.extend(emitted_calls(function));
            dependencies.push(calls);
        }
        let missing_definition = |call: FunctionCall| ParseError {
            message: format!(
                "device helper `{}` has no definition",
                self.module.functions[call.function]
                    .name
                    .as_deref()
                    .unwrap_or_default(),
            ),
            span: call.span,
        };
        for &call in &self.kernel_calls {
            if !retained[call.function.index()] {
                return Err(missing_definition(call));
            }
        }
        let mut states = alloc::vec![VisitState::Unvisited; self.module.functions.len()];
        let mut order = Vec::with_capacity(self.module.functions.len());
        let mut pending = Vec::new();
        for (root, _) in self.module.functions.iter() {
            if !retained[root.index()] || states[root.index()] != VisitState::Unvisited {
                continue;
            }
            states[root.index()] = VisitState::Visiting;
            pending.push((root, 0));
            while let Some(&mut (caller, ref mut next_call)) = pending.last_mut() {
                if *next_call == dependencies[caller.index()].len() {
                    let (caller, _) = pending.pop().unwrap();
                    states[caller.index()] = VisitState::Complete;
                    order.push(caller);
                    continue;
                }
                let call = dependencies[caller.index()][*next_call];
                *next_call += 1;
                if !retained[call.function.index()] {
                    return Err(missing_definition(call));
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
        let mut handles = alloc::vec![None; self.module.functions.len()];
        let mut functions: Vec<_> = self
            .module
            .functions
            .drain()
            .map(|(_, function, span)| Some((function, span)))
            .collect();
        for previous in order {
            let (function, span) = functions[previous.index()].take().unwrap();
            let handle = self.module.functions.append(function, span);
            handles[previous.index()] = Some(handle);
        }
        for (_, function) in self.module.functions.iter_mut() {
            remap_calls(function, &handles);
        }
        for entry in &mut self.module.entry_points {
            remap_calls(&mut entry.function, &handles);
        }
        Ok(())
    }
}
