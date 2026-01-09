use crate::prelude::*;

pub trait EscapeEvaluator<D: Dynamics> {
    type Output;

    fn evaluate(
        &self,
        dynamics: &D,
        p: &D::Param,
    ) -> Self::Output;
}

pub struct EscapeResult {
    escaped: bool,
    iter: usize,
    max_iter: usize,

}
