use super::*;

#[derive(Clone, Copy, PartialEq)]
pub(super) enum FunctionKind {
    Kernel,
    Device,
}

pub(super) struct FunctionContext<'a> {
    pub(super) kind: FunctionKind,
    pub(super) ir: Function,
    pub(super) emitter: crate::proc::Emitter,
    pub(super) symbol_table: SymbolTable<&'a str, Symbol>,
    pub(super) statement_depth: u32,
    pub(super) loop_depth: u32,
    pub(super) builtin_arguments: [Option<Handle<Expression>>; 3],
    pub(super) expression_depth: u32,
    pub(super) expression_graph_depths: Vec<u32>,
    pub(super) conditional_references: Arena<Option<ConditionalReference>>,
    pub(super) pending_array: Option<Handle<LocalVariable>>,
    pub(super) constant_locals: FastHashMap<Handle<LocalVariable>, Literal>,
}

impl FunctionContext<'_> {
    pub(super) fn new(kind: FunctionKind) -> Self {
        let ir = Function::default();
        let mut emitter = crate::proc::Emitter::default();
        emitter.start(&ir.expressions);
        Self {
            kind,
            ir,
            emitter,
            symbol_table: SymbolTable::default(),
            statement_depth: 0,
            loop_depth: 0,
            builtin_arguments: [None; 3],
            expression_depth: 0,
            expression_graph_depths: Vec::new(),
            conditional_references: Arena::new(),
            pending_array: None,
            constant_locals: FastHashMap::default(),
        }
    }
}

impl Parser<'_> {
    pub(super) fn append_call_result(
        &mut self,
        function: Handle<Function>,
        span: Span,
    ) -> Result<Handle<Expression>, ParseError> {
        self.function
            .ir
            .body
            .extend(self.function.emitter.finish(&self.function.ir.expressions));
        let handle = self.append(Expression::CallResult(function), span)?;
        self.function.emitter.start(&self.function.ir.expressions);
        Ok(handle)
    }

    pub(super) fn flush_emitter(&mut self) {
        self.function
            .ir
            .body
            .extend(self.function.emitter.finish(&self.function.ir.expressions));
        self.function.emitter.start(&self.function.ir.expressions);
    }

    pub(super) fn push_statement(&mut self, statement: Statement, span: Span) {
        self.flush_emitter();
        self.function.ir.body.push(statement, span);
    }

    pub(super) fn capture_block<T>(
        &mut self,
        parse: impl FnOnce(&mut Self) -> Result<T, ParseError>,
    ) -> Result<(T, crate::Block), ParseError> {
        self.flush_emitter();
        let parent_body = core::mem::take(&mut self.function.ir.body);
        let result = parse(self);
        self.flush_emitter();
        let captured_body = core::mem::replace(&mut self.function.ir.body, parent_body);
        result.map(|value| (value, captured_body))
    }

    pub(super) fn append_block(&mut self, block: crate::Block) {
        self.flush_emitter();
        self.function.ir.body.extend_block(block);
    }

    pub(super) fn prepend_block(&mut self, mut block: crate::Block) {
        self.flush_emitter();
        core::mem::swap(&mut self.function.ir.body, &mut block);
        self.append_block(block);
    }
}
