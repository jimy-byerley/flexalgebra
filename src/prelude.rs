use core::ops::*;

pub trait Dim: Copy + Clone + Sized {
	fn value(&self) -> usize;
	fn check(value: usize) -> Option<Self>;
}

#[derive(Copy, Clone)]
pub struct Dyn (usize);
impl Dim for Dyn {
	fn value(&self) -> usize {self.0}
	fn check(value: usize) -> Option<Self> {Some(Self(value))}
}

#[derive(Copy, Clone)]
pub struct Stat<const N: usize> {}
impl<const N: usize> Dim for Stat<N> {
	fn value(&self) -> usize {N}
	fn check(value: usize) -> Option<Self> {None}
}


pub trait Element: Clone {}
pub trait Scalar: Element + Add + Sub + Mul {}
