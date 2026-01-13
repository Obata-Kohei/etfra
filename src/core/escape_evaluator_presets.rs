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
    fn evaluate(&self, dynamics: &D, p: &D::Param) -> EscapeResult {
        let mut state = dynamics.initial_state(p);

        for i in 1..=self.max_iter {
            state = dynamics.step(&state, p);

            if self.condition.escaped(&state) {
                return EscapeResult {
                    escaped: true,
                    iter: i,
                    //last_state: Some(state),
                };
            }
        }

        EscapeResult {
            escaped: false,
            iter: self.max_iter,
            //last_state: None,
        }
    }
}

