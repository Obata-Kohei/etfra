use num_complex::{self, Complex};
use crate::prelude::*;

#[derive(Debug)]
pub struct EscapeByCount {
    max_iter: usize,
    escape_radius: Float,
}

impl EscapeByCount {
    pub fn new(max_iter: usize, escape_radius: Float) -> Self {
        Self { max_iter, escape_radius }
    }
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