use super::*;

impl Parser<'_> {
    fn flush_emitter(&mut self) {
        self.function
            .body
            .extend(self.emitter.finish(&self.function.expressions));
        self.emitter.start(&self.function.expressions);
    }

    pub(super) fn push_statement(&mut self, statement: Statement, span: Span) {
        self.flush_emitter();
        self.function.body.push(statement, span);
    }

    pub(super) fn capture_block<T>(
        &mut self,
        parse: impl FnOnce(&mut Self) -> Result<T, ParseError>,
    ) -> Result<(T, crate::Block), ParseError> {
        self.flush_emitter();
        let parent_body = core::mem::take(&mut self.function.body);
        let result = parse(self);
        self.flush_emitter();
        let captured_body = core::mem::replace(&mut self.function.body, parent_body);
        result.map(|value| (value, captured_body))
    }

    pub(super) fn append_block(&mut self, block: crate::Block) {
        self.flush_emitter();
        self.function.body.extend_block(block);
    }

    pub(super) fn prepend_block(&mut self, mut block: crate::Block) {
        self.flush_emitter();
        core::mem::swap(&mut self.function.body, &mut block);
        self.append_block(block);
    }
}
