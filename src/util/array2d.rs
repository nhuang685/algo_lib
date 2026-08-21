use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
    slice::{Chunks, ChunksMut, Iter, IterMut},
};

#[derive(Clone, Default)]
pub struct Arr2D<T> {
    n: usize,
    m: usize,
    data: Vec<T>,
}
impl<T: Debug> Debug for Arr2D<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.iter().collect::<Vec<_>>().fmt(f)
    }
}
impl<T: Clone> Arr2D<T> {
    pub fn new(val: T, n: usize, m: usize) -> Arr2D<T> {
        Arr2D {
            n,
            m,
            data: vec![val; n * m],
        }
    }
}
impl<T> Arr2D<T> {
    pub fn from_fn<F>(n: usize, m: usize, mut f: F) -> Self
    where
        F: FnMut(usize, usize) -> T,
    {
        let mut data = Vec::with_capacity(n * m);
        for i in 0..n {
            for j in 0..m {
                data.push(f(i, j));
            }
        }
        Self { n, m, data }
    }
    pub fn iter(&self) -> Chunks<'_, T> {
        self.data.chunks(self.m)
    }
    pub fn iter_mut(&mut self) -> ChunksMut<'_, T> {
        self.data.chunks_mut(self.m)
    }
    pub fn iter_all(&self) -> Iter<'_, T> {
        self.data.iter()
    }
    pub fn iter_all_enumerate(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        self.iter_all()
            .enumerate()
            .map(|(i, val)| (i / self.m, i % self.m, val))
    }
    pub fn iter_all_mut(&mut self) -> IterMut<'_, T> {
        self.data.iter_mut()
    }
    pub fn iter_all_mut_enumerate(&mut self) -> impl Iterator<Item = (usize, usize, &mut T)> {
        self.data
            .iter_mut()
            .enumerate()
            .map(|(i, val)| (i / self.m, i % self.m, val))
    }
    pub fn len(&self) -> usize {
        self.n
    }
    pub fn len_col(&self) -> usize {
        self.m
    }
    pub fn is_empty(&self) -> bool {
        self.n == 0 || self.m == 0
    }
    pub fn get(&self, i: usize, j: usize) -> &T {
        &self.data[self.m * i + j]
    }
    pub fn get_mut(&mut self, i: usize, j: usize) -> &mut T {
        &mut self.data[self.m * i + j]
    }
}

impl<T> Index<usize> for Arr2D<T> {
    type Output = [T];
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[self.m * index..self.m * (index + 1)]
    }
}
impl<T> IndexMut<usize> for Arr2D<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[self.m * index..self.m * (index + 1)]
    }
}
impl<T> Index<(usize, usize)> for Arr2D<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[self.m * index.0 + index.1]
    }
}
impl<T> IndexMut<(usize, usize)> for Arr2D<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[self.m * index.0 + index.1]
    }
}
