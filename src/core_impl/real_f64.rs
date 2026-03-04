use crate::core::real::Real;

impl Real for f64 {
    fn zero(_: u32) -> Self {
        0.0
    }

    fn from_f64(_: u32, value: f64) -> Self {
        value
    }

    fn from_usize(prec: u32, value: usize) -> Self {
        value as f64
    }

    fn set(&mut self, other: &Self) {
        *self = *other;
    }

    fn set_zero(&mut self) {
        *self = 0.0;
    }

    fn square_mut(&mut self) {
        *self *= *self;
    }

    fn abs_mut(&mut self) {
        *self = self.abs();
    }

    fn add_assign_ref(&mut self, other: &Self) {
        *self += *other;
    }

    fn sub_assign_ref(&mut self, other: &Self) {
        *self -= *other;
    }

    fn mul_assign_ref(&mut self, other: &Self) {
        *self *= *other;
    }

    fn div_assign_ref(&mut self, other: &self) {
        *self /= *other;
    }
}