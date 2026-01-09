use crate::prelude::*;

pub trait EscapeEvaluator<D: Dynamics> {
    fn evaluate(
        &self,
        dynamics: &D,
        p: &D::Param,
    ) -> EscapeResult;
}

pub struct EscapeResult {
    escaped: bool,
    iter: usize,
    max_iter: usize,
}

impl EscapeResult {
    pub fn new(escaped: bool, iter: usize, max_iter: usize) -> Self {
        Self {escaped, iter, max_iter}
    }
}
