use crate::util::color::Color;

pub trait Coloring<T>
where T: Copy
{
    fn color(&self, value: T) -> Color;
}


