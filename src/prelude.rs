use core::ops::*;
use num_traits::{Zero, One};

pub trait Dim: Copy + Clone + Sized {
	fn value(&self) -> usize;
	fn check(value: usize) -> Option<Self>;
}

#[derive(Copy, Clone)]
pub struct Dyn (usize);
impl Dim for Dyn {
	fn value(&self) -> usize {self.0}
	fn check(value: usize) -> Option<Self>  {Some(Self(value))}
}

#[derive(Copy, Clone)]
pub struct Stat<const N: usize> {}
impl<const N: usize> Dim for Stat<N> {
	fn value(&self) -> usize {N}
	fn check(value: usize) -> Option<Self>  {if value == N {Some(Self{})} else {None}}
}


pub trait Element: Clone {}
impl<T: Clone> Element for T {}

pub trait Scalar: Element 
	+ Add<Self, Output=Self> 
	+ Sub<Self, Output=Self> 
	+ Mul<Self, Output=Self> 
	+ Div<Self, Output=Self>
	+ Zero
	+ One
	{}
impl<T: Element 
	+ Add<Self, Output=Self> 
	+ Sub<Self, Output=Self> 
	+ Mul<Self, Output=Self> 
	+ Div<Self, Output=Self>
	+ Zero
	+ One
	> Scalar for T {}
