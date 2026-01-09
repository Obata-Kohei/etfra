pub trait Dynamics {
    type State;
    type Param;

    fn initial_state(&self, p: &Self::Param) -> Self::State;
    fn step(&self, x: &Self::State, p: &Self::Param) -> Self::State;
}
