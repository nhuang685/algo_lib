use crate::create_value;
use crate::numeric::num_traits::arithmetic::Arithmetic;
use crate::numeric::num_traits::integer::Integer;
use crate::numeric::num_traits::signed::Signed;
use crate::numeric::num_traits::unsigned::Unsigned;
use crate::util::value::Value;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::{Product, Sum};
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub fn ext_eucl<T: Signed>(a: T, b: T) -> Option<(T, T)> {
    if a < b {
        ext_eucl(b, a).map(|(a, b)| (b, a))
    } else if b == T::zero() {
        if a == T::one() {
            Some((a, T::zero()))
        } else {
            None
        }
    } else {
        ext_eucl(b, a % b).map(|(x, y)| (y, x - (a / b) * y))
    }
}

pub trait BaseMod<S: Unsigned>: Arithmetic + Eq + Ord + Hash + Default {
    fn val(self) -> S;
    fn modu() -> S;
}

macro_rules! from_unsigned_lower {
    ($name: ident, $s: ty, $($t: ty)+) => {$(
        impl<V: Value<$s>> From<$t> for $name<V> {
            fn from(value: $t) -> Self {
                Self {
                    v: value as $s % V::val(),
                    phantom: PhantomData
                }
            }
        }
    )+};
}
macro_rules! from_unsigned_upper {
    ($name: ident, $s: ty, $($t: ty)+) => {$(
        impl<V: Value<$s>> From<$t> for $name<V> {
            fn from(value: $t) -> Self {
                Self {
                    v: (value % V::val() as $t) as $s,
                    phantom: PhantomData
                }
            }
        }
    )+};
}
macro_rules! from_signed_lower {
    ($name: ident, $s: ty, $($t: ty)+) => {$(
        impl<V: Value<$s>> From<$t> for $name<V> {
            fn from(value: $t) -> Self {
                let mut val = value as i64 % (V::val() as i64);
                if val < 0 {
                    val += V::val() as i64;
                }
                Self {
                    v: val as $s,
                    phantom: PhantomData
                }
            }
        }
    )+};
}
macro_rules! from_signed_upper {
    ($name: ident, $s: ty, $($t: ty)+) => {$(
        impl<V: Value<$s>> From<$t> for $name<V> {
            fn from(value: $t) -> Self {
                let mut val = value % (V::val() as $t);
                if val < 0 {
                    val += V::val() as $t;
                }
                Self {
                    v: val as $s,
                    phantom: PhantomData
                }
            }
        }
    )+};
}
macro_rules! mod_impl {
    ($name: ident, $s: ty) => {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
        pub struct $name<V: Value<$s>> {
            v: $s,
            phantom: PhantomData<V>,
        }
        impl<V: Value<$s>> Display for $name<V> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                Display::fmt(&self.v, f)
            }
        }
        impl<V: Value<$s>> Debug for $name<V> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                Debug::fmt(&self.v, f)
            }
        }

        from_unsigned_lower!($name, $s, u8 u16 u32);
        from_unsigned_upper!($name, $s, usize u64 u128);
        from_signed_lower!($name, $s, i8 i16 i32);
        from_signed_upper!($name, $s, isize i64 i128);

        impl<V: Value<$s>> AddAssign for $name<V> {
            fn add_assign(&mut self, rhs: Self) {
                self.v += rhs.v;
                if self.v >= Self::modu() {
                    self.v -= Self::modu();
                }
            }
        }
        impl<V: Value<$s>> Add for $name<V> {
            type Output = Self;
            fn add(mut self, rhs: Self) -> Self::Output {
                self += rhs;
                self
            }
        }
        impl<V: Value<$s>> SubAssign for $name<V> {
            fn sub_assign(&mut self, rhs: Self) {
                self.v = self.v.wrapping_sub(rhs.v);
                if self.v >= Self::modu() {
                    self.v = self.v.wrapping_add(Self::modu())
                }
            }
        }
        impl<V: Value<$s>> Sub for $name<V> {
            type Output = Self;
            fn sub(mut self, rhs: Self) -> Self::Output {
                self -= rhs;
                self
            }
        }

        impl<V: Value<$s>> MulAssign for $name<V> {
            fn mul_assign(&mut self, rhs: Self) {
                self.v = self.v.mod_mul(rhs.v, Self::modu());
            }
        }
        impl<V: Value<$s>> Mul for $name<V> {
            type Output = Self;
            fn mul(mut self, rhs: Self) -> Self::Output {
                self *= rhs;
                self
            }
        }

        impl<V: Value<$s>> Neg for $name<V> {
            type Output = Self;
            fn neg(self) -> Self::Output {
                Self::from(Self::modu() - self.v)
            }
        }

        impl<V: Value<$s>> Arithmetic for $name<V> {
            fn zero() -> Self {
                Self::raw(0)
            }
            fn one() -> Self {
                Self::raw(1)
            }
            fn two() -> Self {
                Self::one() + Self::one()
            }
            fn from_u8(val: u8) -> Self {
                Self::from(val)
            }
            fn from_usize(val: usize) -> Self {
                Self::from(val)
            }
        }

        impl<V: Value<$s>> $name<V> {
            pub fn raw(v: $s) -> Self {
                Self {
                    v,
                    phantom: PhantomData,
                }
            }
            pub fn pow<T: Integer>(mut self, mut exp: T) -> Self {
                let mut res = Self::one();
                while exp > T::zero() {
                    if exp % T::two() == T::one() {
                        res *= self;
                    }
                    self *= self;
                    exp /= T::two();
                }
                res
            }
            pub fn inv(&self) -> Self {
                match self.inv_checked() {
                    Some(val) => val,
                    None => panic!("gcd({}, {}) != 1", self.v, Self::modu()),
                }
            }
            pub fn inv_checked(&self) -> Option<Self> {
                ext_eucl(self.v as i64, Self::modu() as i64).map(|val| Self::from(val.0))
            }
            pub fn last_k(n: $s, k: $s) -> Self {
                (n..n - k).map(Self::from).fold(Self::one(), |a, b| a * b)
            }
        }

        impl<V: Value<$s>> Sum for $name<V> {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::zero(), Self::add)
            }
        }
        impl<'a, V: Value<$s>> Sum<&'a $name<V>> for $name<V> {
            fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
                iter.fold(Self::zero(), |a, &b| a + b)
            }
        }
        impl<V: Value<$s>> Product for $name<V> {
            fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::one(), $name::mul)
            }
        }
        impl<'a, V: Value<$s>> Product<&'a $name<V>> for $name<V> {
            fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
                iter.fold(Self::one(), |a, &b| a * b)
            }
        }
        impl<V: Value<$s>> BaseMod<$s> for $name<V> {
            fn val(self) -> $s {
                self.v
            }
            fn modu() -> $s {
                V::val()
            }
        }
    }
}

mod_impl!(Modular, u32);
mod_impl!(Modular64, u64);

create_value!(V1e9_7, pub v1e9_7: u32 = 1_000_000_007);
pub type Mint1e9_7 = Modular<V1e9_7>;
create_value!(V998244353, pub v998244353: u32 = 998_244_353);
pub type Mint998244353 = Modular<V998244353>;
