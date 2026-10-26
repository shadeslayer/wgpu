use super::*;

const NODE_LIMIT: usize = 1_048_576;
const WORK_LIMIT: usize = 4_194_304;
const BYTE_LIMIT: usize = 67_108_864;

#[derive(Default)]
pub(in super::super) struct ExpansionBudget {
    active: bool,
    nodes: usize,
    work: usize,
    bytes: usize,
}

impl ExpansionBudget {
    pub(in super::super) const fn activate(&mut self) -> bool {
        let newly_active = !self.active;
        self.active = true;
        newly_active
    }

    fn charge(
        active: bool,
        used: &mut usize,
        count: usize,
        limit: usize,
        span: Span,
    ) -> Result<(), ParseError> {
        if active {
            if count > limit - *used {
                return Err(limit_error(span));
            }
            *used += count;
        }
        Ok(())
    }

    pub(in super::super) fn charge_nodes(
        &mut self,
        count: usize,
        span: Span,
    ) -> Result<(), ParseError> {
        Self::charge(self.active, &mut self.nodes, count, NODE_LIMIT, span)
    }

    pub(in super::super) fn charge_work(
        &mut self,
        count: usize,
        span: Span,
    ) -> Result<(), ParseError> {
        Self::charge(self.active, &mut self.work, count, WORK_LIMIT, span)
    }

    pub(in super::super) fn charge_bytes(
        &mut self,
        count: usize,
        span: Span,
    ) -> Result<(), ParseError> {
        Self::charge(self.active, &mut self.bytes, count, BYTE_LIMIT, span)
    }

    pub(in super::super) fn charge_slice<T>(
        &mut self,
        count: usize,
        span: Span,
    ) -> Result<(), ParseError> {
        let bytes = count
            .checked_mul(size_of::<T>())
            .ok_or_else(|| limit_error(span))?;
        self.charge_bytes(bytes, span)
    }

    pub(in super::super) fn charge_map_entry<K, V>(
        &mut self,
        count: usize,
        span: Span,
    ) -> Result<(), ParseError> {
        let entry = size_of::<K>()
            .checked_add(size_of::<V>())
            .and_then(|size| size.checked_add(size_of::<usize>()))
            .and_then(|size| size.checked_mul(2))
            .and_then(|size| size.checked_mul(count))
            .ok_or_else(|| limit_error(span))?;
        self.charge_bytes(entry, span)
    }
}

pub(super) fn limit_error(span: Span) -> ParseError {
    ParseError {
        message: String::from("CUDA helper parameter expansion limit exceeded"),
        span,
    }
}
