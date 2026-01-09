use num_complex::{self, Complex};
use crate::prelude::*;

pub trait ComplexDynamics: Dynamics<State = Complex<Float>, Param = Complex<Float>> {}
