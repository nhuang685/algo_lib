use std::ops::{Bound, RangeBounds};

use crate::range::monoid::Monoid;
use crate::util::array2d::Arr2D;

#[derive(Clone, Debug)]
pub struct ST<S> {
    n: usize,
    d: Arr2D<S>,
}

impl<S: Monoid> ST<S> {
    pub fn from_slice<T>(values: &[T]) -> Self
    where
        S: From<T>,
        T: Clone,
    {
        if values.is_empty() {
            return Self {
                n: 0,
                d: Arr2D::new(S::e(), 0, 0),
            };
        }
        let n = values.len();
        let lg = n.ilog2() as usize;
        let mut d = Arr2D::new(S::e(), lg + 1, n);
        for (dest, val) in d[0].iter_mut().zip(values) {
            *dest = S::from(val.clone());
        }
        for i in 1..=lg {
            for j in 0..=n - (1 << i) {
                d[i][j] = d[(i - 1, j)]
                    .clone()
                    .op(d[(i - 1, j + (1 << (i - 1)))].clone());
            }
        }
        Self { n, d }
    }

    pub fn query(&self, bounds: impl RangeBounds<usize>) -> S {
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
            return S::e();
        }
        let lg = (r - l).ilog2() as usize;
        self.d[(lg, l)]
            .clone()
            .op(self.d[(lg, r - (1 << lg))].clone())
    }
}
