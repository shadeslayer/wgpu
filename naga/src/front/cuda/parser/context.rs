use super::*;

impl Parser<'_> {
    pub(super) fn capture_block<T>(
        &mut self,
        parse: impl FnOnce(&mut Self) -> Result<T, ParseError>,
    ) -> Result<(T, crate::Block), ParseError> {
        let parent_body = core::mem::take(&mut self.function.body);
        let result = parse(self);
        let captured_body = core::mem::replace(&mut self.function.body, parent_body);
        result.map(|value| (value, captured_body))
    }

    pub(super) fn append_block(&mut self, block: crate::Block) {
        self.function.body.extend_block(block);
    }

    pub(super) fn prepend_block(&mut self, mut block: crate::Block) {
        core::mem::swap(&mut self.function.body, &mut block);
        self.append_block(block);
    }
}
