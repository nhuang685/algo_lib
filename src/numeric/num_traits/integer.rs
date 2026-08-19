use std::{
    hash::Hash,
    ops::{
        BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign, Not, Rem,
        RemAssign, Shl, ShlAssign, Shr, ShrAssign,
    },
};

use crate::numeric::num_traits::arithmetic::Arithmetic;

pub trait Integer:
    Arithmetic
    + Div<Output = Self>
    + DivAssign
    + Rem<Output = Self>
    + RemAssign
    + Shl<Output = Self>
    + ShlAssign
    + Shr<Output = Self>
    + ShrAssign
    + BitAnd<Output = Self>
    + BitAndAssign
    + BitOr<Output = Self>
    + BitOrAssign
    + BitXor<Output = Self>
    + BitXorAssign
    + Not
    + Eq
    + Ord
    + Hash
    + 'static
{
    type Up: From<Self> + Integer;
    fn max() -> Self;
    fn min() -> Self;
    fn downcast(val: Self::Up) -> Self;
    fn as_usize(self) -> usize;
    fn gcd(self, rhs: Self) -> Self {
        if rhs == Self::zero() {
            self
        } else {
            rhs.gcd(self % rhs)
        }
    }
}

macro_rules! integer_impl {
    ($t: ty, $up: ty) => {
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
        impl Integer for $t {
            type Up = $up;
            fn max() -> Self {
                <$t>::MAX
            }
            fn min() -> Self {
                <$t>::MIN
            }
            fn downcast(val: Self::Up) -> Self {
                val as $t
            }
            fn as_usize(self) -> usize {
                self as usize
            }
        }
    };
}
integer_impl!(i128, i128);
integer_impl!(i64, i128);
integer_impl!(i32, i64);
integer_impl!(i16, i32);
integer_impl!(i8, i16);
integer_impl!(isize, isize);
integer_impl!(u128, u128);
integer_impl!(u64, u128);
integer_impl!(u32, u64);
integer_impl!(u16, u32);
integer_impl!(u8, u16);
integer_impl!(usize, usize);
