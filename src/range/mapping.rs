use crate::range::monoid::Monoid;
pub trait Mapping<S>: Clone
where
    S: Monoid,
{
    fn apply(&self, val: S) -> S;
    fn comp(self, g: Self) -> Self;
    fn id() -> Self;
}
