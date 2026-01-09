use num_complex::Complex;
use crate::prelude::*;

#[derive(Debug)]
pub struct EscapeByNorm {
    pub escape_radius: Float,
}

impl EscapeCondition<Complex<Float>> for EscapeByNorm {
    fn escaped(&self, z: &Complex<Float>) -> bool {
        z.norm_sqr() > self.escape_radius * self.escape_radius
    }
}


#[derive(Debug)]
pub struct EscapeByBox {
    pub x_range: (Float, Float),
    pub y_range: (Float, Float),
}

impl EscapeCondition<Complex<Float>> for EscapeByBox{
    fn escaped(&self, z: &Complex<Float>) -> bool {
        z.re < self.x_range.0 || self.x_range.1 < z.re ||
        z.im < self.y_range.0 || self.y_range.1 < z.im
    }
    
}


#[derive(Debug)]
pub struct Converged {
    pub eps: Float,
}

impl EscapeCondition<Complex<Float>> for Converged {
    fn escaped(&self, z: &Complex<Float>) -> bool {
        z.norm_sqr() < self.eps * self.eps
    }
}
