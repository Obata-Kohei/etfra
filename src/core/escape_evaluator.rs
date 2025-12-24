use num_complex::{self, Complex};
use crate::core::complex_dynamics::ComplexDynamics;
use crate::util::types::Float;

pub trait EscapeEvaluator<D: ComplexDynamics> {
    type Output: Copy;
    fn evaluate(
        &self,
        dynamics: &D,
        c: Complex<Float>,
    ) -> Self::Output;
}
