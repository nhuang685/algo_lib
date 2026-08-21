use std::{iter, mem};

pub struct Graph<T> {
    n: usize,
    first: Vec<Option<usize>>,
    nxt: Vec<Option<usize>>,
    v: Vec<T>,
}
impl<T> Graph<T> {
    pub fn new(n: usize, m: usize) -> Self {
        Graph {
            n,
            first: vec![None; n],
            nxt: Vec::with_capacity(m),
            v: Vec::with_capacity(m),
        }
    }
    pub fn add(&mut self, u: usize, v: T) {
        self.nxt.push(self.first[u]);
        self.first[u] = Some(self.v.len());
        self.v.push(v);
    }
    pub fn list(&self, u: usize) -> GraphIterator<'_, T> {
        GraphIterator {
            graph: self,
            cur: self.first[u],
        }
    }
    pub fn list_mut(&mut self, u: usize) -> GraphIteratorMut<'_, T> {
        GraphIteratorMut {
            nxt: &self.nxt,
            v: &mut self.v,
            cur: self.first[u],
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = (usize, &T)> {
        (0..self.n).flat_map(|i| iter::repeat(i).zip(self.list(i)))
    }
}
impl Graph<usize> {
    pub fn biadd(&mut self, u: usize, v: usize) {
        self.add(u, v);
        self.add(v, u);
    }
}
pub struct GraphIterator<'a, T> {
    graph: &'a Graph<T>,
    cur: Option<usize>,
}
impl<'a, T> Iterator for GraphIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.cur.map(|e| {
            let v = &self.graph.v[e];
            self.cur = self.graph.nxt[e];
            v
        })
    }
}

pub struct GraphIteratorMut<'a, T> {
    nxt: &'a [Option<usize>],
    v: &'a mut [T],
    cur: Option<usize>,
}
impl<'a, T> Iterator for GraphIteratorMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        let e = self.cur?;
        let (bef, aft) = mem::take(&mut self.v).split_at_mut(e);
        self.cur = self.nxt[e];
        self.v = bef;
        Some(&mut aft[0])
    }
}
