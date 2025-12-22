use num_complex::{self, Complex};
use crate::escape_time_fractal::Float;

pub trait ComplexDynamics {
    fn initial_z(&self, c: Complex<Float>) -> Complex<Float>;
    fn step(&self, z: Complex<Float>, c: Complex<Float>) -> Complex<Float>;
}

pub struct Mandelbrot;

impl ComplexDynamics for Mandelbrot {
    fn initial_z(&self, _c: Complex<Float>) -> Complex<Float> {
        Complex::ZERO
    }
    fn step(&self, z: Complex<Float>, c: Complex<Float>) -> Complex<Float> {
        z * z + c
    }
}

pub struct Julia {
    pub c: Complex<Float>,
}

impl ComplexDynamics for Julia {
    fn initial_z(&self, z: Complex<Float>) -> Complex<Float> {
        z
    }

    fn step(&self, z: Complex<Float>, _: Complex<Float>) -> Complex<Float> {
        z * z + self.c
    }
}


pub struct BurningShip;

impl ComplexDynamics for BurningShip {
    fn initial_z(&self, _c: Complex<Float>) -> Complex<Float> {
        Complex::ZERO
    }
    fn step(&self, z: Complex<Float>, c: Complex<Float>) -> Complex<Float> {
        let z = Complex::new(z.re.abs(), z.im.abs());
        z * z + c
    }
}