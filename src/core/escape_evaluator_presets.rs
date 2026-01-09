use crate::prelude::*;

#[derive(Debug)]
pub struct EscapeByCount<C> {
    pub max_iter: usize,
    pub condition: C,
}

impl<D, C> EscapeEvaluator<D> for EscapeByCount<C>
where
    D: Dynamics,
    C: EscapeCondition<D::State>,
{
    type Output = usize;

    fn evaluate(&self, dynamics: &D, p: &<D as Dynamics>::Param,) -> Self::Output {
        let mut state = dynamics.initial_state(p);

        for i in 1..=self.max_iter {
            if self.condition.escaped(&state) {
                return i;
            }
            state = dynamics.step(&state, p);
        }

        self.max_iter
    }
}

