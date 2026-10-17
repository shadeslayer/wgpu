use naga::front::cuda::{parse_str, Options};

pub(crate) fn validate(source: &str) -> (naga::Module, naga::valid::ModuleInfo) {
    let module = parse_str(source, &Options::default()).unwrap();
    let info = naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::empty(),
    )
    .validate(&module)
    .unwrap();
    (module, info)
}

#[derive(Clone, Copy, Debug)]
enum Evaluated {
    Number(f64),
    Global(usize, usize),
    Local(usize),
}

impl Evaluated {
    fn number(self) -> f64 {
        match self {
            Self::Number(value) => value,
            other => panic!("expected numeric value, got {other:?}"),
        }
    }
}

struct Interpreter<'a> {
    module: &'a naga::Module,
    info: &'a naga::valid::ModuleInfo,
    globals: Vec<Vec<f64>>,
    locals: Vec<f64>,
    expression_values: Vec<Option<Evaluated>>,
}

impl Interpreter<'_> {
    fn value(&self, handle: naga::Handle<naga::Expression>) -> Evaluated {
        if let Some(value) = self.expression_values[handle.index()] {
            return value;
        }
        match self.module.entry_points[0].function.expressions[handle] {
            naga::Expression::Literal(naga::Literal::I32(value)) => {
                Evaluated::Number(f64::from(value))
            }
            naga::Expression::Literal(naga::Literal::U32(value)) => {
                Evaluated::Number(f64::from(value))
            }
            naga::Expression::Literal(naga::Literal::F32(value)) => {
                Evaluated::Number(f64::from(value))
            }
            naga::Expression::Literal(naga::Literal::Bool(value)) => {
                Evaluated::Number(f64::from(u8::from(value)))
            }
            naga::Expression::GlobalVariable(global) => Evaluated::Global(global.index(), 0),
            naga::Expression::LocalVariable(local) => Evaluated::Local(local.index()),
            ref expression => panic!("expression was not emitted: {expression:?}"),
        }
    }

    fn emit(&mut self, handle: naga::Handle<naga::Expression>) {
        let expression = &self.module.entry_points[0].function.expressions[handle];
        let value = match *expression {
            naga::Expression::Access { base, index } => match self.value(base) {
                Evaluated::Global(global_index, element_offset) => Evaluated::Global(
                    global_index,
                    element_offset + self.value(index).number() as usize,
                ),
                other => panic!("unexpected indexed pointer: {other:?}"),
            },
            naga::Expression::Load { pointer } => Evaluated::Number(match self.value(pointer) {
                Evaluated::Global(global_index, element_index) => {
                    self.globals[global_index][element_index]
                }
                Evaluated::Local(local) => self.locals[local],
                other => panic!("unexpected load pointer: {other:?}"),
            }),
            naga::Expression::As { expr, kind, .. } => {
                Evaluated::Number(Self::convert(self.value(expr).number(), kind))
            }
            naga::Expression::Unary { op, expr } => {
                let value = self.value(expr).number();
                Evaluated::Number(match op {
                    naga::UnaryOperator::Negate => -value,
                    naga::UnaryOperator::LogicalNot => f64::from(u8::from(value == 0.0)),
                    _ => panic!("unexpected unary operation: {op:?}"),
                })
            }
            naga::Expression::Binary { op, left, right } => {
                let left_value = self.value(left).number();
                let right_value = self.value(right).number();
                let scalar = self.info.get_entry_point(0)[handle]
                    .ty
                    .inner_with(&self.module.types)
                    .scalar()
                    .unwrap();
                let value = match op {
                    naga::BinaryOperator::Add => left_value + right_value,
                    naga::BinaryOperator::Subtract => left_value - right_value,
                    naga::BinaryOperator::Multiply => left_value * right_value,
                    naga::BinaryOperator::Divide if scalar.kind != naga::ScalarKind::Float => {
                        (left_value / right_value).trunc()
                    }
                    naga::BinaryOperator::Divide => left_value / right_value,
                    naga::BinaryOperator::Equal => f64::from(u8::from(left_value == right_value)),
                    naga::BinaryOperator::NotEqual => {
                        f64::from(u8::from(left_value != right_value))
                    }
                    naga::BinaryOperator::Less => f64::from(u8::from(left_value < right_value)),
                    _ => panic!("unexpected binary operation: {op:?}"),
                };
                Evaluated::Number(Self::convert(value, scalar.kind))
            }
            ref expression => panic!("unexpected emitted expression: {expression:?}"),
        };
        self.expression_values[handle.index()] = Some(value);
    }

    fn convert(value: f64, kind: naga::ScalarKind) -> f64 {
        match kind {
            naga::ScalarKind::Sint => f64::from(value as i64 as i32),
            naga::ScalarKind::Uint => f64::from(value as i64 as u32),
            naga::ScalarKind::Float => f64::from(value as f32),
            naga::ScalarKind::Bool => f64::from(u8::from(value != 0.0)),
            _ => panic!("unexpected scalar kind"),
        }
    }

    fn run(&mut self, block: &naga::Block) -> bool {
        for statement in block {
            match statement {
                naga::Statement::Emit(range) => {
                    for handle in range.clone() {
                        self.emit(handle);
                    }
                }
                naga::Statement::Store { pointer, value } => {
                    let value = self.value(*value).number();
                    match self.value(*pointer) {
                        Evaluated::Global(global_index, element_index) => {
                            self.globals[global_index][element_index] = value
                        }
                        Evaluated::Local(local) => self.locals[local] = value,
                        other => panic!("unexpected store pointer: {other:?}"),
                    }
                }
                naga::Statement::Block(body) => {
                    if self.run(body) {
                        return true;
                    }
                }
                naga::Statement::If {
                    condition,
                    accept,
                    reject,
                } => {
                    let selected = if self.value(*condition).number() != 0.0 {
                        accept
                    } else {
                        reject
                    };
                    if self.run(selected) {
                        return true;
                    }
                }
                naga::Statement::Return { value: None } => return true,
                other => panic!("unexpected statement: {other:?}"),
            }
        }
        false
    }
}

pub(crate) fn execute(source: &str, initial_output: &[i32]) -> Vec<i32> {
    let (module, info) = validate(source);
    let function = &module.entry_points[0].function;
    let mut interpreter = Interpreter {
        module: &module,
        info: &info,
        globals: vec![initial_output.iter().copied().map(f64::from).collect()],
        locals: vec![0.0; function.local_variables.len()],
        expression_values: vec![None; function.expressions.len()],
    };
    interpreter.run(&function.body);
    interpreter
        .globals
        .remove(0)
        .into_iter()
        .map(|value| value as i32)
        .collect()
}
