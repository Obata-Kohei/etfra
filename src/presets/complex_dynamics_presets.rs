use crate::prelude::*;
use num_complex::{self, Complex};

// -----
/// Mandelbrot
// -----
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

    fn param_from_xy(&self, point: (Float, Float)) -> Self::Param {
        Complex::new(point.0, point.1)
    }

    fn initial_state(&self, _p: &Self::Param) -> Self::State {
        Complex::ZERO
    }

    fn step(&self, x: &Self::State, p: &Self::Param) -> Self::State {
        x * x + p
    }
}

impl ComplexDynamics for Mandelbrot {}

// -----
/// Julia
// -----
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

    fn param_from_xy(&self, point: (Float, Float)) -> Self::Param {
        Complex::new(point.0, point.1)
    }

    fn initial_state(&self, p: &Self::Param) -> Self::State {
        *p
    }

    fn step(&self, z: &Self::State, _p: &Self::Param) -> Self::State {
        z * z + self.c
    }
}

impl ComplexDynamics for Julia {}


// -----
/// Burning Ship
// -----
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

    fn param_from_xy(&self, point: (Float, Float)) -> Self::Param {
        Complex::new(point.0, point.1)
    }

    fn initial_state(&self, _p: &Self::Param) -> Self::State {
        Complex::ZERO
    }

    fn step(&self, z: &Self::State, c: &Self::Param) -> Self::State {
        let z = Complex::new(z.re.abs(), z.im.abs());
        z * z + c
    }
}


/// Tricorn (Mandelbar):
/// z[n + 1] = conj(z[n])^2 + c
/// `c` is the pixel coordinate, as in the Mandelbrot set.
#[derive(Debug, Default, Clone, Copy)]
pub struct Tricorn;

impl Tricorn {
    pub fn new() -> Self {
        Self {}
    }
}

impl Dynamics for Tricorn {
    type State = Complex<Float>;
    type Param = Complex<Float>;

    fn param_from_xy(&self, point: (Float, Float)) -> Self::Param {
        Complex::new(point.0, point.1)
    }

    fn initial_state(&self, _p: &Self::Param) -> Self::State {
        Complex::ZERO
    }

    fn step(&self, z: &Self::State, c: &Self::Param) -> Self::State {
        z.conj() * z.conj() + c
    }
}



/// Phoenix
/// z[n + 1] = z[n]^2 + c + feedback * z[n - 1]
/// The default `feedback` is -0.5. 
#[derive(Debug)]
pub struct Phoenix {
    pub feedback: Complex<Float>,
}

/// The state used for Phoenix fractal
/// current is z[n], previous is z[n - 1]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PhoenixState {
    pub current: Complex<Float>,
    pub previous: Complex<Float>,
}

impl Phoenix {
    pub fn new() -> Self {
        Self {
            feedback: Complex::new(-0.5, 0.0),
        }
    }

    pub fn with_feedback(feedback: Complex<Float>) -> Self {
        Self { feedback }
    }
}

impl Dynamics for Phoenix {
    type State = PhoenixState;
    type Param = Complex<Float>;

    fn param_from_xy(&self, point: (Float, Float)) -> Self::Param {
        Complex::new(point.0, point.1)
    }

    fn initial_state(&self, _p: &Self::Param) -> Self::State {
        PhoenixState {
            current: Complex::ZERO,
            previous: Complex::ZERO,
        }
    }

    fn step(&self, state: &Self::State, c: &Self::Param) -> Self::State {
        let next = state.current * state.current + c + self.feedback * state.previous;

        PhoenixState {
            current: next,
            previous: state.current,
        }
    }
}