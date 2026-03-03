pub trait Real {}
pub trait Complex<R: Real> {}

pub struct Plane {}

pub trait State {
    // 力学系に入力するStateの初期状態を返す．
    fn new(&self) -> Self;
}

pub trait Dynamics {
    type State;

    // 入力されたStateをもとに力学系を計算する．stateは変更される
    fn apply(&self, &mut state: Self::State);
}

pub trait EscapeCondition<S> {
    // Dynamics::Stateを期待する
    fn escaped(&self, s: &S) -> bool;
}

pub trait EscapeEvaluator<D: Dynamics> {
    fn evaluate(&self, dynamics: &D, state: &D::State) -> EscapeResult;
}

#[derive(Debug, Default)]
pub struct EscapeResult {
    pub escaped: bool,
    pub iter: usize,
    //pub nu: Float,  // smooth coloring
}

impl EscapeResult {
    pub fn new(escaped: bool, iter: usize) -> Self {
        Self { escaped, iter }
    }
}

pub trait NormalizeEscInfo<T> {
    fn max_iter(&self) -> usize;
    fn prepare(&mut self, values: &[T]);
    fn normalize(&self, value: &T) -> f64;
}

pub trait ColorMap {
    fn map(&self, t: f64) -> Color;
}

pub struct Coloring<N, M> {
    pub normalizer: N,
    pub color_map: M,
}
