pub trait EscapeCondition<S> {  // Dynamics::Stateを期待する
    fn escaped(&self, s: &S) -> bool;
}