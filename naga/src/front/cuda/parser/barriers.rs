use super::*;

// Naga's uniformity analysis does not yet validate control barriers.
pub(super) fn check_control_flow(
    block: &crate::Block,
    conditional: bool,
    mut may_have_returned: bool,
) -> Result<bool, ParseError> {
    for (statement, &span) in block.span_iter() {
        match *statement {
            Statement::ControlBarrier(_) if conditional || may_have_returned => {
                return Err(ParseError {
                    message: String::from(
                        "CUDA barriers require straight-line control flow with no prior possible return",
                    ),
                    span,
                });
            }
            Statement::Block(ref body) => {
                may_have_returned = check_control_flow(body, conditional, may_have_returned)?;
            }
            Statement::If {
                ref accept,
                ref reject,
                ..
            } => {
                let accept_returned = check_control_flow(accept, true, may_have_returned)?;
                let reject_returned = check_control_flow(reject, true, may_have_returned)?;
                may_have_returned |= accept_returned || reject_returned;
            }
            Statement::Loop {
                ref body,
                ref continuing,
                ..
            } => {
                may_have_returned |= check_control_flow(body, true, may_have_returned)?;
                may_have_returned |= check_control_flow(continuing, true, may_have_returned)?;
            }
            Statement::Return { .. } => may_have_returned = true,
            _ => {}
        }
    }
    Ok(may_have_returned)
}
