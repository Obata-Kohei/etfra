use crate::util::color::Color;

pub trait Coloring<T> {
    fn prepare(&mut self, values: &[T]);  // 空実装でいい．
    fn color(&self, value: T) -> Color;
}