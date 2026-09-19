use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

pub trait Arithmetic:
    Copy
    + Debug
    + Display
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + PartialEq
    + PartialOrd
{
    fn zero() -> Self;
    fn one() -> Self;
    fn two() -> Self;
    fn from_usize(val: usize) -> Self;
    fn from_u8(val: u8) -> Self;
}

macro_rules! arithmetic_impl_float {
    ($($t: ty)+) => {$(
        impl Arithmetic for $t {
            fn zero() -> Self {
                0.0
            }
            fn one() -> Self {
                1.0
            }
            fn two() -> Self {
                2.0
            }
            fn from_usize(val: usize) -> Self {
                val as $t
            }
            fn from_u8(val: u8) -> Self {
                val as $t
            }
        }
    )+};
}
arithmetic_impl_float!(f32 f64);
macro_rules! arithmetic_impl {
    ($($t: ty)+) => {$(
        impl Arithmetic for $t {
            fn zero() -> Self {
                0
            }
            fn one() -> Self {
                1
            }
            fn two() -> Self {
                2
            }
            fn from_usize(val: usize) -> Self {
                val as $t
            }
            fn from_u8(val: u8) -> Self {
                val as $t
            }
        }
    )+};
}
arithmetic_impl!(isize i8 i16 i32 i64 i128 usize u8 u16 u32 u64 u128);
