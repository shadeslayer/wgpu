use super::*;

impl Parser<'_> {
    pub(super) fn conditional(
        &mut self,
        condition: ExpressionResult,
        start: usize,
    ) -> Result<ExpressionResult, ParseError> {
        let condition = self.truthy(condition.require_scalar()?)?;
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
                ExpressionResult::Scalar(ScalarExpression::Reference(accept)),
                ExpressionResult::Scalar(ScalarExpression::Reference(reject)),
            ) if accept.scalar == reject.scalar => {
                let depth = 1 + self
                    .reference_depth(&accept)
                    .max(self.reference_depth(&reject));
                if depth > 64 {
                    return Err(ParseError {
                        message: String::from("CUDA reference nesting limit exceeded"),
                        span,
                    });
                }
                let scalar = accept.scalar;
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
                Ok(ExpressionResult::Scalar(ScalarExpression::Reference(
                    ReferenceValue {
                        reference: Reference::Conditional(handle),
                        writable,
                        scalar,
                        span,
                    },
                )))
            }
            (ExpressionResult::Scalar(accept), ExpressionResult::Scalar(reject)) => {
                let scalar = if accept.scalar() == reject.scalar() {
                    accept.scalar()
                } else {
                    Self::arithmetic_scalar(accept.scalar(), reject.scalar())
                };
                let pointer = self.temporary_pointer(scalar, span)?;
                let ((), accept) = self.capture_block(|parser| {
                    parser.append_block(accept_body);
                    let value = parser.convert_value(accept, scalar)?;
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
                    let value = parser.convert_value(reject, scalar)?;
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
                Ok(Value {
                    handle,
                    scalar,
                    span,
                }
                .into())
            }
            _ => Err(ParseError {
                message: String::from("conditional arms must both be scalar or both be void"),
                span,
            }),
        }
    }
}
