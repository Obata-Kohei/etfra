pub trait NormalizeEscInfo<T> {
    fn max_iter(&self) -> usize;
    fn prepare(&mut self, values: &[T]);
    fn normalize(&self, value: &T) -> f64;
}