pub trait Monoid: Clone {
    fn op(self, rhs: Self) -> Self;
    fn e() -> Self;
    fn e_1() -> Self {
        Self::e()
    }
}
