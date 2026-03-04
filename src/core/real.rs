use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

pub trait Real:
    Clone
    + PartialOrd
    + Send
    + Sync
    + AddAssign<Self>
    + SubAssign<Self>
    + MulAssign<Self>
    + DivAssign<Self>
{
    /* ===== 生成 ===== */

    /// 指定精度で 0 を生成
    fn zero(prec: u32) -> Self;

    /// 指定精度で f64 から生成
    fn from_f64(prec: u32, value: f64) -> Self;

    // 指定制度で usize から生成
    fn from_usize(prec: u32, value: usize) -> Self;

    /* ===== 代入・再利用 ===== */

    /// self = other
    fn set(&mut self, other: &Self);

    /// self = 0
    fn set_zero(&mut self);

    /* ===== in-place 演算 ===== */
    /// self *= self
    fn square_mut(&mut self);

    /// self = |self|
    fn abs_mut(&mut self);

    /// self += other
    fn add_assign_ref(&mut self, other: &Self);

    /// self -= other
    fn sub_assign_ref(&mut self, other: &Self);

    /// self *= other
    fn mul_assign_ref(&mut self, other: &Self);

    /// self /= other
    fn div_assign_ref(&mut self, other: &Self);
}