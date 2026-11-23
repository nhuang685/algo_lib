use std::marker::PhantomData;

use crate::numeric::modular::BaseMod;
use crate::numeric::num_traits::unsigned::Unsigned;

#[derive(Clone, Debug)]
pub struct Combo<S: Unsigned, Mint: BaseMod<S>, const LIM: usize = 4194304> {
    pub fac: Vec<Mint>,
    pub ifac: Vec<Mint>,
    phantom: PhantomData<S>,
}

impl<S: Unsigned, Mint: BaseMod<S>, const LIM: usize> Combo<S, Mint, LIM> {
    pub fn new() -> Self {
        let mut fac = vec![Mint::one(); LIM + 1];
        let mut ifac = vec![Mint::one(); LIM + 1];
        for i in 2..=LIM {
            fac[i] = fac[i - 1] * Mint::from_usize(i);
        }
        ifac[LIM] = fac[LIM].inv();
        for i in (0..LIM).rev() {
            ifac[i] = ifac[i + 1] * Mint::from_usize(i + 1).inv();
        }
        Self {
            fac,
            ifac,
            phantom: PhantomData,
        }
    }
}
impl<S: Unsigned, Mint: BaseMod<S>, const LIM: usize> Default for Combo<S, Mint, LIM> {
    fn default() -> Self {
        Self::new()
    }
}
impl<S: Unsigned, Mint: BaseMod<S>, const LIM: usize> Combo<S, Mint, LIM> {
    pub fn inv(&self, n: usize) -> Mint {
        self.fac[n - 1] * self.ifac[n]
    }
    pub fn comb(&self, n: usize, k: usize) -> Mint {
        if n < k {
            Mint::zero()
        } else {
            self.fac[n] * self.ifac[k] * self.ifac[n - k]
        }
    }
    pub fn perm(&self, n: usize, k: usize) -> Mint {
        if n < k {
            Mint::zero()
        } else {
            self.fac[n] * self.ifac[n - k]
        }
    }
}
