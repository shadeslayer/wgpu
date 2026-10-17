use super::*;

impl<'a> Parser<'a> {
    pub(super) fn parameters(&mut self) -> Result<(), ParseError> {
        self.expect_symbol('(')?;
        if !self.take_symbol(')')? {
            if self.take_ident("void")? {
                self.expect_symbol(')')?;
            } else {
                loop {
                    self.parameter()?;
                    if self.take_symbol(')')? {
                        break;
                    }
                    self.expect_symbol(',')?;
                }
            }
        }
        Ok(())
    }

    pub(super) fn check_function_name(&self, name: &str, span: Span) -> Result<(), ParseError> {
        self.check_name(name, span)?;
        if self.functions.contains_key(name) {
            return Err(ParseError {
                message: format!("duplicate function or unsupported overload `{name}`"),
                span,
            });
        }
        Ok(())
    }

    pub(super) fn device_function(&mut self) -> Result<(), ParseError> {
        let result_span = self.lexer.peek()?.span;
        let scalar = if self.take_ident("void")? {
            None
        } else {
            Some(self.scalar_type()?)
        };
        let (name, span) = self.identifier()?;
        self.check_function_name(name, span)?;
        self.function = FunctionContext::new(FunctionKind::Device);
        self.function.ir.name = Some(String::from(name));
        if let Some(scalar) = scalar {
            self.function.ir.result = Some(crate::FunctionResult {
                ty: self.scalar_handle(scalar, result_span),
                binding: None,
            });
        }
        self.parameters()?;
        self.expect_symbol('{')?;
        let terminated = self.block_contents()?;
        if self.function.ir.result.is_some() && !terminated {
            return Err(ParseError {
                message: String::from("device helper must return a value on every supported path"),
                span,
            });
        }
        self.flush_emitter();
        if self.function.ir.result.is_none() {
            crate::proc::ensure_block_returns(&mut self.function.ir.body);
        }
        let function = core::mem::replace(
            &mut self.function,
            FunctionContext::new(FunctionKind::Kernel),
        );
        let handle = self.module.functions.append(function.ir, span);
        self.functions.insert(name, handle);
        Ok(())
    }

    pub(super) fn device_parameter(
        &mut self,
        name: &'a str,
        span: Span,
        scalar: Scalar,
        is_const: bool,
    ) -> Result<(), ParseError> {
        let ty = self.scalar_handle(scalar, span);
        let index = self.function.ir.arguments.len() as u32;
        self.function.ir.arguments.push(FunctionArgument {
            name: Some(String::from(name)),
            ty,
            binding: None,
        });
        let value = self.append(Expression::FunctionArgument(index), span)?;
        self.initialize_parameter(name, span, scalar, is_const, value)
    }

    pub(super) fn call(&mut self, name: &str, span: Span) -> Result<ExpressionResult, ParseError> {
        if self.lookup_symbol(name).is_some() {
            return Err(ParseError {
                message: format!("local value `{name}` cannot be called"),
                span,
            });
        }
        let function = self
            .functions
            .get(name)
            .copied()
            .ok_or_else(|| ParseError {
                message: format!("unknown or not previously defined device helper `{name}`"),
                span,
            })?;
        self.expect_symbol('(')?;
        let mut arguments = Vec::new();
        if self.lexer.peek()?.kind != TokenKind::Symbol(')') {
            loop {
                let Some(parameter) = self.module.functions[function]
                    .arguments
                    .get(arguments.len())
                else {
                    return Err(ParseError {
                        message: format!("too many arguments for device helper `{name}`"),
                        span,
                    });
                };
                let TypeInner::Scalar(scalar) = self.module.types[parameter.ty].inner else {
                    unreachable!()
                };
                let value = self.expression(0)?;
                arguments.push(self.convert_value(value, scalar)?.handle);
                if self.lexer.peek()?.kind == TokenKind::Symbol(')') {
                    break;
                }
                self.expect_symbol(',')?;
            }
        }
        let end = self.expect_symbol(')')?;
        if arguments.len() != self.module.functions[function].arguments.len() {
            return Err(ParseError {
                message: format!("too few arguments for device helper `{name}`"),
                span,
            });
        }
        let span = span.until(&end);
        let scalar = self.module.functions[function]
            .result
            .as_ref()
            .map(|result| {
                let TypeInner::Scalar(scalar) = self.module.types[result.ty].inner else {
                    unreachable!()
                };
                scalar
            });
        let value = if let Some(scalar) = scalar {
            let handle = self.append_call_result(function, span)?;
            Some(Value {
                handle,
                scalar,
                span,
            })
        } else {
            None
        };
        self.push_statement(
            Statement::Call {
                function,
                arguments,
                result: value.map(|value| value.handle),
            },
            span,
        );
        Ok(match value {
            Some(value) => value.into(),
            None => ExpressionResult::Void(span),
        })
    }
}
