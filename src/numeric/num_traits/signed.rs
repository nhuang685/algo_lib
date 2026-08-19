use std::ops::Neg;

use crate::numeric::num_traits::integer::Integer;

pub trait Signed: Integer + Neg {
    fn abs(self) -> Self;
}

macro_rules! signed_impl {
    ($($t: ty)+) => {$(
        impl Signed for $t {
            fn abs(self) -> Self {
                <$t>::abs(self)
            }
        }
    )+}
}
signed_impl!(isize i8 i16 i32 i64 i128);
