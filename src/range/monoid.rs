use std::ops::Add;

pub trait Monoid: Clone + Add<Output = Self> + Default {}
