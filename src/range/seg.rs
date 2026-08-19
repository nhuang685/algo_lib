use std::ops::{Bound, RangeBounds};

use crate::range::monoid::Monoid;

#[derive(Clone, Debug)]
pub struct Seg<S> {
    n: usize,
    len: usize,
    d: Vec<S>,
}
impl<S: Monoid> Seg<S> {
    pub fn with_size(size: usize) -> Self {
        let len = size.next_power_of_two();
        Seg {
            n: size,
            len,
            d: vec![S::default(); 2 * len],
        }
    }
    fn pull(&mut self, i: usize) {
        self.d[i] = self.d[i << 1].clone() + self.d[(i << 1) + 1].clone();
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
        // l += self.len;
        // r += self.len;
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
            return S::default();
        }
        l += self.len;
        r += self.len - 1;
        let mut lv = S::default();
        let mut rv = S::default();
        while l <= r {
            if l & 1 == 1 {
                lv = lv + self.d[l].clone();
                l += 1;
            }
            if r & 1 == 0 {
                rv = self.d[r].clone() + rv;
                r -= 1;
            }
        }
        lv + rv
    }
}

impl<S> From<&[S]> for Seg<S>
where
    S: Monoid,
{
    fn from(value: &[S]) -> Self {
        let mut seg = Self::with_size(value.len());
        seg.d[seg.len..seg.len + seg.n].clone_from_slice(value);
        for i in (1..seg.len - 1).rev() {
            seg.pull(i);
        }
        seg
    }
}
