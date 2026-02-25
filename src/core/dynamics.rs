use crate::prelude::*;

pub enum IterResult {
    Continue,
    Escaped,
    Converged(usize),
    MaxIter,
}

pub trait DynamicalSystem {
    type State;

    fn init(
        &self,
        pixel: (Real, Real),
        param: (Real, Real),
    ) -> Self::State;

    fn step(&self, state: &mut Self::State);

    fn classify(&self, state: &Self::State) -> IterResult;
}






///
/*
pub trait Dynamics {
    type State;
    type Param;

    fn param_from_xy(&self, point: (Float, Float)) -> Self::Param;

    fn initial_state(&self, p: &Self::Param) -> Self::State;
    fn step(&self, x: &Self::State, p: &Self::Param) -> Self::State;
}
*/