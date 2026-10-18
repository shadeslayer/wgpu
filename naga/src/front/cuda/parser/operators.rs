use super::*;

pub(super) const ADDITION_NAME: &str = "operator+";

impl Parser<'_> {
    pub(super) fn operator_addition_name(&mut self, span: Span) -> Result<Span, ParseError> {
        let end = self.lexer.expect_span(Token::Operation('+'))?;
        Ok(span.until(&end))
    }

    pub(super) fn is_source_class(&self, ty: &TypeResolution) -> bool {
        matches!(
            *ty.inner_with(&self.module.types),
            TypeInner::Vector { .. } | TypeInner::Struct { .. }
        )
    }

    pub(super) fn addition(
        &mut self,
        left: ExpressionValue,
        precedence: u8,
        operator_span: Span,
    ) -> Result<ExpressionResult, ParseError> {
        let (right, right_body) = self.capture_block(|parser| parser.expression(precedence + 1))?;
        if self.is_source_class(left.ty()) || self.is_source_class(right.ty()) {
            let left = self.rvalue(left)?;
            self.append_block(right_body);
            let right = self.rvalue(right)?;
            let span = left.span.until(&right.span);
            self.call_with_arguments(ADDITION_NAME, alloc::vec![left, right], operator_span, span)
        } else {
            self.append_block(right_body);
            self.binary_value(BinaryOperator::Add, left, right)
                .map(Into::into)
        }
    }
}
