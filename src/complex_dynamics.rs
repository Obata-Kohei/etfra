use crate::prelude::*;
use num_complex::{self, Complex};

pub trait ComplexDynamics: Dynamics<State = Complex<Float>, Param = Complex<Float>> {}
