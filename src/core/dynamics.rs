use crate::prelude::*;

pub trait Dynamics {
    type State;
    type Param;

    fn param_from_xy(&self, point: (Float, Float)) ->Self::Param;

    fn initial_state(&self, p: &Self::Param) -> Self::State;
    fn step(&self, x: &Self::State, p: &Self::Param) -> Self::State;
}
