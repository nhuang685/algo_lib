use crate::numeric::num_traits::integer::Integer;

pub fn ext_eucl<T: Integer>(a: T, b: T) -> Option<(T, T)> {
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
