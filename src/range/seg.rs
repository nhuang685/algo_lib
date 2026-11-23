use std::ops::{Bound, RangeBounds};

use crate::range::monoid::Monoid;

#[derive(Clone, Debug)]
pub struct Seg<S> {
    n: usize,
    len: usize,
    d: Vec<S>,
}
impl<S: Monoid> Seg<S> {
    fn new(size: usize) -> Self {
        let len = size.next_power_of_two();
        Seg {
            n: size,
            len,
            d: vec![S::e_1(); 2 * len],
        }
    }
    fn build(&mut self) {
        for i in (1..self.len).rev() {
            self.pull(i);
        }
    }
    pub fn with_size(size: usize) -> Self {
        let mut seg = Self::new(size);
        seg.build();
        seg
    }
    fn pull(&mut self, i: usize) {
        self.d[i] = self.d[i << 1].clone().op(self.d[(i << 1) + 1].clone());
    }
    pub fn set(&mut self, mut i: usize, val: S) {
        i += self.len;
        self.d[i] = val;
        i >>= 1;
        while i >= 1 {
            self.pull(i);
            i >>= 1;
        }
    }
    pub fn get(&self, i: usize) -> &S {
        &self.d[i + self.len]
    }
    pub fn all(&self) -> &S {
        &self.d[1]
    }
    pub fn query(&self, bounds: impl RangeBounds<usize>) -> S {
        let mut l = match bounds.start_bound() {
            Bound::Included(&x) => x,
            Bound::Excluded(&x) => x + 1,
            Bound::Unbounded => 0,
        };
        let mut r = match bounds.end_bound() {
            Bound::Included(&x) => x + 1,
            Bound::Excluded(&x) => x,
            Bound::Unbounded => self.n,
        };
        if l >= r {
            return S::e();
        }
        l += self.len;
        r += self.len - 1;
        let mut lv = S::e();
        let mut rv = S::e();
        while l <= r {
            if l & 1 == 1 {
                lv = lv.op(self.d[l].clone());
                l += 1;
            }
            if r & 1 == 0 {
                rv = self.d[r].clone().op(rv);
                r -= 1;
            }
            l >>= 1;
            r >>= 1;
        }
        lv.op(rv)
    }
}

impl<S> Seg<S>
where
    S: Monoid,
{
    pub fn from_slice<T>(value: &[T]) -> Self
    where
        S: From<T>,
        T: Clone,
    {
        let mut seg = Self::with_size(value.len());
        // seg.d[seg.len..seg.len + seg.n].clone_from_slice(value);
        for (dest, val) in seg.d[seg.len..seg.len + seg.n].iter_mut().zip(value) {
            *dest = S::from(val.clone());
        }
        seg.build();
        seg
    }
}
