use crate::prelude::*;
pub trait Coloring<T> {
    fn prepare(&mut self, values: &[T]);  // 空実装でいい．
    fn apply(&self, value: T) -> Color;
}

pub trait NormalizeEscRes {
    fn normalize(&self, value: EscapeResult) -> Float;
}

pub trait ColorMap {
    fn map(&self, t: Float) -> Color;
}
