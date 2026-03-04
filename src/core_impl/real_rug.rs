use crate::core::real::Real;
use rug::Float;

impl Real for Float {
    fn zero(prec: u32) -> Self {
        Float::with_val(prec, 0)
    }

    fn from_f64(prec: u32, value: f64) -> Self {
        Float::with_val(prec, value)
    }

    fn from_usize(prec: u32, value: usize) -> Self {
        Float::with_val(prec, value)
    }

    fn set(&mut self, other: &Self) {
        self.assign(other);
    }

    fn set_zero(&mut self) {
        self.assign(0);
    }

    fn square_mut(&mut self) {
        let tmp = self.clone();
        *self *= tmp;
    }

    fn abs_mut(&mut self) {
        Float::abs_mut(self);
    }

    fn add_assign_ref(&mut self, other: &Self) {
        *self += other;
    }

    fn sub_assign_ref(&mut self, other: &Self) {
        *self -= other;
    }

    fn mul_assign_ref(&mut self, other: &Self) {
        *self *= other;
    }

    fn div_assign_ref(&mut self, other: &self) {
        *self /= other;
    }

}