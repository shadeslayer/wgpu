use super::*;

#[derive(Clone, Copy)]
pub(super) enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    And,
    InclusiveOr,
    ExclusiveOr,
    ShiftLeft,
    ShiftRight,
    LogicalAnd,
    LogicalOr,
    BitwiseNot,
    LogicalNot,
}

impl Operator {
    pub(super) const fn name(self) -> &'static str {
        match self {
            Self::Add => "operator+",
            Self::Subtract => "operator-",
            Self::Multiply => "operator*",
            Self::Divide => "operator/",
            Self::Modulo => "operator%",
            Self::And => "operator&",
            Self::InclusiveOr => "operator|",
            Self::ExclusiveOr => "operator^",
            Self::ShiftLeft => "operator<<",
            Self::ShiftRight => "operator>>",
            Self::LogicalAnd => "operator&&",
            Self::LogicalOr => "operator||",
            Self::BitwiseNot => "operator~",
            Self::LogicalNot => "operator!",
        }
    }

    const fn from_token(token: Token<'_>) -> Option<Self> {
        Some(match token {
            Token::Operation('+') => Self::Add,
            Token::Operation('-') => Self::Subtract,
            Token::Operation('*') => Self::Multiply,
            Token::Operation('/') => Self::Divide,
            Token::Operation('%') => Self::Modulo,
            Token::Operation('&') => Self::And,
            Token::Operation('|') => Self::InclusiveOr,
            Token::Operation('^') => Self::ExclusiveOr,
            Token::ShiftOperation('<') => Self::ShiftLeft,
            Token::ShiftOperation('>') => Self::ShiftRight,
            Token::LogicalOperation('&') => Self::LogicalAnd,
            Token::LogicalOperation('|') => Self::LogicalOr,
            Token::Operation('~') => Self::BitwiseNot,
            Token::Operation('!') => Self::LogicalNot,
            _ => return None,
        })
    }

    pub(super) const fn from_binary(operator: BinaryOperator) -> Option<Self> {
        Some(match operator {
            BinaryOperator::Add => Self::Add,
            BinaryOperator::Subtract => Self::Subtract,
            BinaryOperator::Multiply => Self::Multiply,
            BinaryOperator::Divide => Self::Divide,
            BinaryOperator::Modulo => Self::Modulo,
            BinaryOperator::And => Self::And,
            BinaryOperator::InclusiveOr => Self::InclusiveOr,
            BinaryOperator::ExclusiveOr => Self::ExclusiveOr,
            BinaryOperator::ShiftLeft => Self::ShiftLeft,
            BinaryOperator::ShiftRight => Self::ShiftRight,
            BinaryOperator::LogicalAnd => Self::LogicalAnd,
            BinaryOperator::LogicalOr => Self::LogicalOr,
            _ => return None,
        })
    }

    const fn accepts_arity(self, count: usize) -> bool {
        match self {
            Self::Add | Self::Subtract | Self::Multiply | Self::And => matches!(count, 1 | 2),
            Self::BitwiseNot | Self::LogicalNot => count == 1,
            _ => count == 2,
        }
    }
}

impl Parser<'_> {
    pub(super) fn operator_name(&mut self, span: Span) -> Result<(Operator, Span), ParseError> {
        let (token, end) = self.lexer.next()?;
        let operator = Operator::from_token(token).ok_or_else(|| ParseError {
            message: String::from("unsupported CUDA operator function"),
            span: end,
        })?;
        Ok((operator, span.until(&end)))
    }

    pub(super) fn is_source_class(&self, ty: &TypeResolution) -> bool {
        matches!(
            *ty.inner_with(&self.module.types),
            TypeInner::Vector { .. } | TypeInner::Struct { .. }
        )
    }

    pub(super) fn check_operator_parameters(
        &self,
        operator: Operator,
        span: Span,
    ) -> Result<(), ParseError> {
        if !operator.accepts_arity(self.function.ir.arguments.len()) {
            return Err(ParseError {
                message: format!("invalid number of parameters for `{}`", operator.name()),
                span,
            });
        }
        if !self
            .function
            .ir
            .arguments
            .iter()
            .any(|argument| self.is_source_class(&TypeResolution::Handle(argument.ty)))
        {
            return Err(ParseError {
                message: String::from(
                    "operator functions require a vector or structure class parameter",
                ),
                span,
            });
        }
        Ok(())
    }

    pub(super) fn binary_operator(
        &mut self,
        mut left: ExpressionValue,
        operator: BinaryOperator,
        precedence: u8,
        operator_span: Span,
    ) -> Result<ExpressionResult, ParseError> {
        if matches!(
            operator,
            BinaryOperator::ShiftLeft | BinaryOperator::ShiftRight
        ) {
            left = self.rvalue(left)?.into();
        }
        let (right, right_body) = self.capture_block(|parser| parser.expression(precedence + 1))?;
        if self.is_source_class(left.ty()) || self.is_source_class(right.ty()) {
            let left = self.rvalue(left)?;
            self.append_block(right_body);
            let right = self.rvalue(right)?;
            let span = left.span.until(&right.span);
            let name = Operator::from_binary(operator).unwrap().name();
            self.call_with_arguments(name, alloc::vec![left, right], operator_span, span)
        } else {
            self.append_block(right_body);
            self.binary_value(operator, left, right).map(Into::into)
        }
    }

    pub(super) fn unary_operator(
        &mut self,
        token: Token<'_>,
        operand: ExpressionValue,
        operator_span: Span,
    ) -> Result<ExpressionResult, ParseError> {
        let operator = Operator::from_token(token).unwrap();
        let value = self.rvalue(operand)?;
        let span = operator_span.until(&value.span);
        self.call_with_arguments(operator.name(), alloc::vec![value], operator_span, span)
    }
}
