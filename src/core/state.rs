use crate::core::real::Real;

/// Dynamicsに入力する状態．
pub trait State {
    type Real: Real;
    /// 初期状態を返す．
    fn new(init_point: (Self::Real, Self::Real)) -> Self;
}