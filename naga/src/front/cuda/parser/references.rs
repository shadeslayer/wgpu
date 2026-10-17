use super::*;

impl Parser<'_> {
    pub(super) fn temporary_pointer(
        &mut self,
        scalar: Scalar,
        span: Span,
    ) -> Result<Handle<Expression>, ParseError> {
        let ty = self.scalar_handle(scalar, span);
        let local = self.function.ir.local_variables.append(
            LocalVariable {
                name: None,
                ty,
                init: None,
            },
            span,
        );
        self.append(Expression::LocalVariable(local), span)
    }

    pub(super) fn reference_depth(&self, reference: &ReferenceValue) -> u32 {
        match reference.reference {
            Reference::Pointer(_) => 0,
            Reference::Conditional(handle) => {
                self.function.conditional_references[handle]
                    .as_ref()
                    .unwrap()
                    .depth
            }
        }
    }

    pub(super) fn rvalue(&mut self, expression: ScalarExpression) -> Result<Value, ParseError> {
        let reference = match expression {
            ScalarExpression::Value(value) => return Ok(value),
            ScalarExpression::Reference(reference) => reference,
        };
        let scalar = reference.scalar;
        let span = reference.span;
        if let Reference::Pointer(pointer) = reference.reference {
            return self.load_pointer(pointer, scalar, span);
        }
        let pointer = self.temporary_pointer(scalar, span)?;
        self.consume_reference(reference, &mut |parser, source, scalar, span| {
            let value = parser.load_pointer(source, scalar, span)?;
            parser.push_statement(
                Statement::Store {
                    pointer,
                    value: value.handle,
                },
                span,
            );
            Ok(())
        })?;
        let handle = self.append(Expression::Load { pointer }, span)?;
        Ok(Value {
            handle,
            scalar,
            span,
        })
    }

    pub(super) fn load_pointer(
        &mut self,
        pointer: Handle<Expression>,
        scalar: Scalar,
        span: Span,
    ) -> Result<Value, ParseError> {
        let handle = self.append(Expression::Load { pointer }, span)?;
        Ok(Value {
            handle,
            scalar,
            span,
        })
    }

    pub(super) fn discard_result(&mut self, result: ExpressionResult) -> Result<(), ParseError> {
        if let ExpressionResult::Scalar(ScalarExpression::Reference(reference)) = result {
            self.consume_reference(reference, &mut |_, _, _, _| Ok(()))?;
        }
        Ok(())
    }

    pub(super) fn decorate_reference(
        &mut self,
        reference: &mut ReferenceValue,
        action: &mut impl FnMut(&mut Self, Handle<Expression>, Scalar, Span) -> Result<(), ParseError>,
    ) -> Result<(), ParseError> {
        let handle = match reference.reference {
            Reference::Pointer(pointer) => {
                return action(self, pointer, reference.scalar, reference.span);
            }
            Reference::Conditional(handle) => handle,
        };
        let mut conditional = self.function.conditional_references[handle].take().unwrap();
        for branch in [&mut conditional.accept, &mut conditional.reject] {
            let ((), body) = self.capture_block(|parser| {
                parser.append_block(core::mem::take(&mut branch.body));
                parser.decorate_reference(&mut branch.reference, action)
            })?;
            branch.body = body;
        }
        self.function.conditional_references[handle] = Some(conditional);
        Ok(())
    }

    pub(super) fn consume_reference(
        &mut self,
        reference: ReferenceValue,
        action: &mut impl FnMut(&mut Self, Handle<Expression>, Scalar, Span) -> Result<(), ParseError>,
    ) -> Result<(), ParseError> {
        let handle = match reference.reference {
            Reference::Pointer(pointer) => {
                return action(self, pointer, reference.scalar, reference.span);
            }
            Reference::Conditional(handle) => handle,
        };
        let conditional = self.function.conditional_references[handle].take().unwrap();
        let ((), accept) = self.capture_block(|parser| {
            parser.append_block(conditional.accept.body);
            parser.consume_reference(conditional.accept.reference, action)
        })?;
        let ((), reject) = self.capture_block(|parser| {
            parser.append_block(conditional.reject.body);
            parser.consume_reference(conditional.reject.reference, action)
        })?;
        self.push_statement(
            Statement::If {
                condition: conditional.condition,
                accept,
                reject,
            },
            reference.span,
        );
        Ok(())
    }
}
