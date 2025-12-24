use num_complex::{self, Complex};
use crate::prelude::*;

pub struct Mandelbrot;

impl Mandelbrot {
    pub fn new() -> Self {
        Self {}
    }
}

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

impl Julia {
    pub fn new(c: Complex<Float>) -> Self {
        Self { c }
    }
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

impl BurningShip {
    pub fn new() -> Self {
        Self {}
    }
}

impl ComplexDynamics for BurningShip {
    fn initial_z(&self, _c: Complex<Float>) -> Complex<Float> {
        Complex::ZERO
    }
    fn step(&self, z: Complex<Float>, c: Complex<Float>) -> Complex<Float> {
        let z = Complex::new(z.re.abs(), z.im.abs());
        z * z + c
    }
}