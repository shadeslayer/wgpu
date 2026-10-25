//! A frontend for a restricted subset of CUDA kernel source.

use alloc::string::String;

mod lexer;
mod number;
mod parser;
mod preprocess;

/// Options supplied by the CUDA kernel's launch configuration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Options {
    /// CUDA launch block dimensions, used for both the entry point and `blockDim`.
    pub workgroup_size: [u32; 3],
}

impl Default for Options {
    fn default() -> Self {
        Self {
            workgroup_size: [1; 3],
        }
    }
}

/// An error in the supported CUDA source subset.
#[derive(Clone, Debug, thiserror::Error)]
#[error("{message}")]
pub struct ParseError {
    pub message: String,
    pub span: crate::Span,
}

/// Parse a CUDA kernel into Naga's intermediate representation.
pub fn parse_str(source: &str, options: &Options) -> Result<crate::Module, ParseError> {
    // Match the validator's maximum workgroup dimension.
    const MAX_WORKGROUP_DIMENSION: u32 = 0x4000;
    if options
        .workgroup_size
        .iter()
        .any(|&dimension| dimension == 0 || dimension > MAX_WORKGROUP_DIMENSION)
        || options
            .workgroup_size
            .into_iter()
            .try_fold(1u32, u32::checked_mul)
            .is_none()
    {
        return Err(ParseError {
            message: String::from(
                "workgroup dimensions must be 1..=16384 and their product must fit u32",
            ),
            span: crate::Span::UNDEFINED,
        });
    }
    let prepared = preprocess::prepare(source)?;
    let lexer = match prepared.as_ref() {
        Some(prepared) => lexer::Lexer::from_prepared(source, prepared),
        None => lexer::Lexer::new(source)?,
    };
    parser::Parser::new(lexer, options).parse()
}
