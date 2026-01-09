use num_complex::{self, Complex};
use crate::prelude::*;

#[derive(Debug)]
pub struct Mandelbrot;

impl Mandelbrot {
    pub fn new() -> Self {
        Self {}
    }
}

impl Dynamics for Mandelbrot {
    type State = Complex<Float>;
    type Param = Complex<Float>;

    fn initial_state(&self, _p: &Self::Param) -> Self::State {
        Complex::ZERO
    }

    fn step(&self, x: &Self::State, p: &Self::Param) -> Self::State {
        x * x + p
    }
}

impl ComplexDynamics for Mandelbrot {}


#[derive(Debug)]
pub struct Julia {
    pub c: Complex<Float>,
}

impl Julia {
    pub fn new(c: Complex<Float>) -> Self {
        Self { c }
    }
}

impl Dynamics for Julia {
    type State = Complex<Float>;
    type Param = Complex<Float>;

    fn initial_state(&self, p: &Self::Param) -> Self::State {
        *p
    }

    fn step(&self, z: &Self::State, _p: &Self::Param) -> Self::State {
        z * z + self.c
    }
}

impl ComplexDynamics for Julia {}


#[derive(Debug)]
pub struct BurningShip;

impl BurningShip {
    pub fn new() -> Self {
        Self {}
    }
}

impl Dynamics for BurningShip {
    type State = Complex<Float>;
    type Param = Complex<Float>;

    fn initial_state(&self, _p: &Self::Param) -> Self::State {
        Complex::ZERO
    }

    fn step(&self, z: &Self::State, c: &Self::Param) -> Self::State {
        let z = Complex::new(z.re.abs(),  z.im.abs());
        z * z + c
    }
}
