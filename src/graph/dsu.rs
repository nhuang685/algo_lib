use std::mem;

pub struct DSU {
    vals: Vec<i32>,
    ncc: usize,
}
impl DSU {
    pub fn with_size(n: usize) -> Self {
        Self {
            vals: vec![-1; n],
            ncc: n,
        }
    }
    pub fn find(&mut self, i: usize) -> usize {
        if self.vals[i] < 0 {
            i
        } else {
            self.vals[i] = self.find(self.vals[i] as usize) as i32;
            self.vals[i] as usize
        }
    }
    pub fn unite(&mut self, mut u: usize, mut v: usize) -> bool {
        u = self.find(u);
        v = self.find(v);
        if u == v {
            return false;
        }
        if self.vals[u] > self.vals[v] {
            mem::swap(&mut u, &mut v);
        }
        self.vals[u] += self.vals[v];
        self.vals[v] = u as i32;
        self.ncc -= 1;
        true
    }
    pub fn cc_size(&mut self, i: usize) -> usize {
        let rt = self.find(i);
        -self.vals[rt] as usize
    }
    pub fn num_cc(&self) -> usize {
        self.ncc
    }
}
