use super::*;

impl Parser<'_> {
    pub(super) fn conditional(
        &mut self,
        condition: ExpressionResult,
        start: usize,
    ) -> Result<ExpressionResult, ParseError> {
        let condition = self.truthy(condition.require_value()?)?;
        let (accept, accept_body) =
            self.capture_block(|parser| parser.comma_expression_result())?;
        self.expect_symbol(':')?;
        let (reject, reject_body) = self.capture_block(|parser| parser.expression_result(0))?;
        let span = self.lexer.span_from(start);
        match (accept, reject) {
            (ExpressionResult::Void(_), ExpressionResult::Void(_)) => {
                self.push_statement(
                    Statement::If {
                        condition: condition.handle,
                        accept: accept_body,
                        reject: reject_body,
                    },
                    span,
                );
                Ok(ExpressionResult::Void(span))
            }
            (
                ExpressionResult::Value(ExpressionValue::Reference(accept)),
                ExpressionResult::Value(ExpressionValue::Reference(reject)),
            ) if self.module.compare_types(&accept.ty, &reject.ty) => {
                let depth = 1 + self
                    .reference_depth(&accept)
                    .max(self.reference_depth(&reject));
                if depth > 64 {
                    return Err(ParseError {
                        message: String::from("CUDA reference nesting limit exceeded"),
                        span,
                    });
                }
                let ty = accept.ty.clone();
                let writable = accept.writable && reject.writable;
                let handle = self.function.conditional_references.append(
                    Some(ConditionalReference {
                        condition: condition.handle,
                        accept: ReferenceBranch {
                            body: accept_body,
                            reference: accept,
                        },
                        reject: ReferenceBranch {
                            body: reject_body,
                            reference: reject,
                        },
                        depth,
                    }),
                    span,
                );
                Ok(ExpressionResult::Value(ExpressionValue::Reference(
                    ReferenceValue {
                        reference: Reference::Conditional(handle),
                        writable,
                        ty,
                        span,
                    },
                )))
            }
            (ExpressionResult::Value(accept), ExpressionResult::Value(reject)) => {
                let ty = if self.module.compare_types(accept.ty(), reject.ty()) {
                    accept.ty().clone()
                } else {
                    scalar_resolution(Self::arithmetic_scalar(
                        self.require_scalar_type(accept.ty(), accept.span())?,
                        self.require_scalar_type(reject.ty(), reject.span())?,
                    ))
                };
                let pointer = self.temporary_pointer(&ty, span)?;
                let ((), accept) = self.capture_block(|parser| {
                    parser.append_block(accept_body);
                    let value = parser.convert_value(accept, &ty)?;
                    parser.push_statement(
                        Statement::Store {
                            pointer,
                            value: value.handle,
                        },
                        span,
                    );
                    Ok(())
                })?;
                let ((), reject) = self.capture_block(|parser| {
                    parser.append_block(reject_body);
                    let value = parser.convert_value(reject, &ty)?;
                    parser.push_statement(
                        Statement::Store {
                            pointer,
                            value: value.handle,
                        },
                        span,
                    );
                    Ok(())
                })?;
                self.push_statement(
                    Statement::If {
                        condition: condition.handle,
                        accept,
                        reject,
                    },
                    span,
                );
                let handle = self.append(Expression::Load { pointer }, span)?;
                Ok(Value { handle, ty, span }.into())
            }
            _ => Err(ParseError {
                message: String::from("conditional arms must both be scalar or both be void"),
                span,
            }),
        }
    }
}
