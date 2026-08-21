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
    fn max_value() -> Self;
    fn min_value() -> Self;
    fn inf() -> Self;
    fn downcast(val: Self::Up) -> Self;
    fn upcast(self) -> Self::Up;
    fn as_usize(self) -> usize;
    fn gcd(self, rhs: Self) -> Self {
        if rhs == Self::zero() {
            self
        } else {
            rhs.gcd(self % rhs)
        }
    }
    fn lcm(self, rhs: Self) -> Self {
        self / self.gcd(rhs) * rhs
    }
    fn mod_mul(self, rhs: Self, m: Self) -> Self {
        Self::downcast(self.upcast() * rhs.upcast() % m.upcast())
    }
    fn wrapping_add(self, rhs: Self) -> Self;
    fn wrapping_sub(self, rhs: Self) -> Self;
    fn wrapping_mul(self, rhs: Self) -> Self;
    fn wrapping_div(self, rhs: Self) -> Self;
}

macro_rules! integer_impl {
    ($t: ty, $up: ty, $inf: literal) => {
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
            fn max_value() -> Self {
                <$t>::MAX
            }
            fn min_value() -> Self {
                <$t>::MIN
            }
            fn inf() -> Self {
                $inf
            }
            fn downcast(val: Self::Up) -> Self {
                val as $t
            }
            fn upcast(self) -> Self::Up {
                self as $up
            }
            fn as_usize(self) -> usize {
                self as usize
            }
            fn wrapping_add(self, rhs: Self) -> Self {
                <$t>::wrapping_add(self, rhs)
            }
            fn wrapping_sub(self, rhs: Self) -> Self {
                <$t>::wrapping_sub(self, rhs)
            }
            fn wrapping_mul(self, rhs: Self) -> Self {
                <$t>::wrapping_mul(self, rhs)
            }
            fn wrapping_div(self, rhs: Self) -> Self {
                <$t>::wrapping_div(self, rhs)
            }
        }
    };
}
integer_impl!(i128, i128, 0x3f3f3f3f3f3f3f3f3f3f3f3f3f3f3f3f);
integer_impl!(i64, i128, 0x3f3f3f3f3f3f3f3f);
integer_impl!(i32, i64, 0x3f3f3f3f);
integer_impl!(i16, i32, 0x3f3f);
integer_impl!(i8, i16, 0x3f);
integer_impl!(isize, isize, 0x3f3f3f3f3f3f3f3f);
integer_impl!(u128, u128, 0x3f3f3f3f3f3f3f3f3f3f3f3f3f3f3f3f);
integer_impl!(u64, u128, 0x3f3f3f3f3f3f3f3f);
integer_impl!(u32, u64, 0x3f3f3f3f);
integer_impl!(u16, u32, 0x3f3f);
integer_impl!(u8, u16, 0x3f);
integer_impl!(usize, usize, 0x3f3f3f3f3f3f3f3f);
