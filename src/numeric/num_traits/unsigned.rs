use crate::numeric::num_traits::integer::Integer;

pub trait Unsigned: Integer {}

macro_rules! unsigned_impl {
    ($($t: ty)+) => {$(
        impl Unsigned for $t {}
    )+}
}
unsigned_impl!(usize u8 u16 u32 u64 u128);
