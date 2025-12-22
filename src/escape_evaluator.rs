use num_complex::{self, Complex};
use crate::complex_dynamics::ComplexDynamics;
use crate::escape_time_fractal::Float;

pub trait EscapeEvaluator<D: ComplexDynamics> {
    type Output: Copy;
    fn evaluate(
        &self,
        dynamics: &D,
        c: Complex<Float>,
    ) -> Self::Output;
}

pub struct EscapeByCount {
    pub max_iter: usize,
    pub escape_radius: Float,
}

impl<D: ComplexDynamics> EscapeEvaluator<D> for EscapeByCount {
    type Output = usize;

    fn evaluate(&self, dynamics: &D, c: Complex<Float>) -> usize {
        let escape_radius_sqr = self.escape_radius * self.escape_radius;
        let mut z = dynamics.initial_z(c);

        for i in 1..=self.max_iter {
            z = dynamics.step(z, c);
            if z.norm_sqr() > escape_radius_sqr {
                return i;
            }
        }

        self.max_iter
    }
}