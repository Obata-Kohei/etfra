use num_complex::{self, Complex};
use crate::util::types::Float;

pub trait ComplexDynamics {
    fn initial_z(&self, c: Complex<Float>) -> Complex<Float>;
    fn step(&self, z: Complex<Float>, c: Complex<Float>) -> Complex<Float>;
}
