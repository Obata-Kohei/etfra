pub trait NormalizeEscInfo<T> {
    fn prepare(&mut self, values: &[T]);
    fn normalize(&self, value: &T) -> f64;
}