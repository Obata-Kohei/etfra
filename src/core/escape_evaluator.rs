use crate::prelude::*;

pub trait EscapeEvaluator<D: Dynamics> {    
    fn evaluate(
        &self,
        dynamics: &D,
        p: &D::Param,
    ) -> EscapeResult;
}

#[derive(Debug, Default)]
pub struct EscapeResult {
    pub escaped: bool,
    pub iter: usize,
    //pub nu: Float,  // smooth coloring
}

impl EscapeResult {
    pub fn new(escaped: bool, iter: usize) -> Self {
        Self {escaped, iter}
    }
}
