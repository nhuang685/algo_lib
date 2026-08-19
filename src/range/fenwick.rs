use std::ops::{Bound, RangeBounds};

use crate::numeric::num_traits::arithmetic::Arithmetic;

pub struct Fenwick<T> {
    n: usize,
    vals: Vec<T>,
}
impl<T> Fenwick<T>
where
    T: Arithmetic,
{
    pub fn with_size(n: usize) -> Self {
        Self {
            n,
            vals: vec![T::zero(); n + 1],
        }
    }
    pub fn upd(&mut self, mut i: usize, val: T) {
        i += 1;
        while i <= self.n {
            self.vals[i] += val;
            i += i & i.wrapping_neg();
        }
    }
    pub fn sum(&self, mut i: usize) -> T {
        let mut sum = T::zero();
        while i > 0 {
            sum += self.vals[i];
            i += i & i.wrapping_neg();
        }
        sum
    }
    pub fn query(&self, bounds: impl RangeBounds<usize>) -> T {
        let l = match bounds.start_bound() {
            Bound::Included(&x) => x,
            Bound::Excluded(&x) => x + 1,
            Bound::Unbounded => 0,
        };
        let r = match bounds.end_bound() {
            Bound::Included(&x) => x + 1,
            Bound::Excluded(&x) => x,
            Bound::Unbounded => self.n,
        };
        if l >= r {
            T::zero()
        } else {
            self.sum(r) - self.sum(l)
        }
    }
}
