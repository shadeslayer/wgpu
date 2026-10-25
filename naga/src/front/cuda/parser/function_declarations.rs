use super::*;

#[derive(Clone, Copy, PartialEq)]
pub(super) enum Inlining {
    Inline,
    ForceInline,
    NoInline,
}

#[derive(Clone, Copy)]
pub(super) struct FunctionCall {
    pub(super) function: Handle<Function>,
    pub(super) span: Span,
}

pub(super) struct FunctionDeclaration {
    pub(super) has_host: bool,
    pub(super) is_static: bool,
    pub(super) inlining: Option<Inlining>,
    pub(super) is_defined: bool,
    pub(super) calls: Vec<FunctionCall>,
}

pub(super) struct IntrinsicDeclaration<'a> {
    pub(super) name: &'a str,
    pub(super) parameters: Vec<Handle<Type>>,
    pub(super) has_host: bool,
    pub(super) is_static: bool,
    pub(super) inlining: Option<Inlining>,
}

pub(super) fn check_redeclaration(
    previous_has_host: bool,
    previous_is_static: bool,
    previous_inlining: Option<Inlining>,
    has_host: bool,
    is_static: bool,
    inlining: Option<Inlining>,
    span: Span,
) -> Result<(), ParseError> {
    if previous_has_host != has_host {
        return Err(ParseError {
            message: String::from("incompatible CUDA execution spaces in helper redeclaration"),
            span,
        });
    }
    if is_static && !previous_is_static {
        return Err(ParseError {
            message: String::from(
                "static helper redeclaration conflicts with prior external linkage",
            ),
            span,
        });
    }
    if previous_inlining.is_some() && inlining.is_some() && previous_inlining != inlining {
        return Err(ParseError {
            message: String::from("incompatible inlining specifiers in helper redeclaration"),
            span,
        });
    }
    Ok(())
}
