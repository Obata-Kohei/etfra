use num_traits::FromPrimitive;
use std::ops::{Add, Div, Mul, Sub};
use rug;

pub type Real = rug::Float;

pub struct Complex {
    re: Real,
    im: Real,
}

impl Complex {
    pub fn new(re: Real, im: Real) -> Self {
        Self {re, im}
    }
}

/* 以下は昔のコード

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
*/