use crate::core::real::Real;

pub trait Dynamics {
    type Real: Real;
    type State: State<Real = Self::Real>;

    /// xy平面上の点から初期状態を生成
    fn init_state(&self, point: (Self::Real, Self::Real)) -> Self::State;

    /// 1ステップ進める
    fn step(&self, state: &mut Self::State);
}