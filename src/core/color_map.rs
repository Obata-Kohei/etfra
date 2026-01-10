use crate::prelude::*;

pub trait ColorMap {
    fn map(&self, t: f64) -> Color;
}