/*!
	Provide the dimension structs and matrix elements traits
*/

use core::ops::*;
use num_traits::{Zero, One};

/**
	Dimensionnality specification for an [Array](crate::Array)
	
	An instance of it stores a dimension size, but depending on the type implementing this trait, the dimension size may be stored in memory [Dyn], or statically known [Stat]
*/
pub trait Dim: Copy + Clone + Sized {
	/// return the value of this dimension specification
	fn value(&self) -> usize;
	/// check that the requested size is allowed by this dimensionality and return a new dimension specification for this size
	fn check(value: usize) -> Option<Self>;
}

/// dynamically determined dimension. The dimension size is stored in the instance's memory
#[derive(Copy, Clone)]
pub struct Dyn (pub usize);
impl Dim for Dyn {
	fn value(&self) -> usize {self.0}
	fn check(value: usize) -> Option<Self>  {Some(Self(value))}
}
/// statically determined dimension. The dimension size is stored in the type known as compile type
#[derive(Copy, Clone)]
pub struct Stat<const N: usize> {}
impl<const N: usize> Dim for Stat<N> {
	fn value(&self) -> usize {N}
	fn check(value: usize) -> Option<Self>  {if value == N {Some(Self{})} else {None}}
}

/// supertrait for minimal requirements on [Array](crate::Array) and [Matrix](crate::Matrix) elements
pub trait Element: Clone {}
impl<T: Clone> Element for T {}

/// supertrait for [Matrix](crate::Matrix) elements allowing linear algebra operations
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
