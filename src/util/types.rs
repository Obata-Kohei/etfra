use num_traits::FromPrimitive;
use std::ops::{Add, Sub, Mul, Div};

pub type Float = f64;

pub trait FloatLike:
    Clone
    + Send
    + Sync
    + FromPrimitive
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
{
}

impl FloatLike for f64 {}

/*
use rug::Float as RugFloat;

impl FloatLike for RugFloat {}
のように書く
*/