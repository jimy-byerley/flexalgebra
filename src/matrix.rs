use std::ops::*;
use std::fmt;
use num_traits::{Zero, One};

use crate::prelude::*;


/** Array2 interface, this trait allows the use of any data structure as a matrix */
pub trait Array2: 
	Index<[usize; 2], Output=<Self as Array2>::Scalar>
	+IndexMut<[usize; 2], Output=<Self as Array2>::Scalar>
{
	type Scalar: Sized + Copy;
	type Owned: Array2<Scalar=Self::Scalar> + Owned;
	
	fn shape(&self) -> [usize; 2];
}


/** wraps an object exposing the Array2 interface to perform matrix operations */
pub struct Matrix<A: Array2> {
	pub array: A,
}

// basic initializer

impl<A: Array2>  From<A> for Matrix<A> {
	fn from(value: A) -> Self   {Self{array: value}}
}
impl<T1, T2, A1, A2>  From<&Matrix<A2>> for Matrix<A1> 
where
	T2: Into<T1> + Clone,
	A1: Array2<Scalar=T1> + Owned + Default,
	A2: Array2<Scalar=T2>,
{
	fn from(value: &Matrix<A2>) -> Self  {Self::from_field(|i|  value[i].clone().into())}
}

// items access

impl<A: Array2> Index<[usize; 2]> for Matrix<A> {
	type Output = A::Scalar;
	fn index(&self, i: [usize; 2]) -> &Self::Output  {&self.array[i]}
}
impl<A: Array2> IndexMut<[usize; 2]> for Matrix<A> {
	fn index_mut(&mut self, i: [usize; 2]) -> &mut Self::Output  {&mut self.array[i]}
}

// general methods

impl<A: Array2> Matrix<A> {

	pub fn shape(&self) -> [usize; 2]	{self.array.shape()}
	pub fn rows(&self) -> usize			{self.shape()[0]}
	pub fn columns(&self) -> usize		{self.shape()[1]}
	
}
impl<A: Array2 + Owned> Matrix<A> {
	/// build a matrix from a function returning a value each index
	pub fn from_field<F>(field: F) -> Self
	where F: Fn([usize; 2]) -> A::Scalar 
	{
		let mut new = Self::from(A::empty());
		for i in 0..new.rows() {
			for j in 0..new.columns() {
				let index = [i,j];
				new[index] = field(index);
			}
		}
		new
	}
	
	/// build a matrix full of the given value
	pub fn full(value: &A::Scalar) -> Self {
		Self::from_field(|_| *value)
	}
}

// custom initializers

impl<S, A> Matrix<A> 
where 
	S: Zero,
	A: Array2<Scalar=S> + Owned,
{
	pub fn zeros() -> Self {
		Self::from_field(|_| A::Scalar::zero())
	}
// 	fn diagonal(diag: Self::Vector) -> Self {}
}
impl<S, A> Matrix<A> 
where 
	S: Zero + One,
	A: Array2<Scalar=S> + Owned,
{
 	pub fn identity() -> Self {
		Self::from_field(|[i,j]| 
			if i==j  {A::Scalar::one()} 
			else {A::Scalar::zero()}
			)
 	}
}

// matrix operators

impl<S,A> fmt::Display for Matrix<A>
where
	S: fmt::Display,
	A: Array2<Scalar=S>,
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "[")?;
		for i in 0 .. self.rows() {
			write!(f, " [")?;
			for j in 0 .. self.columns() {
				write!(f, "{}, ", self[[i,j]])?;
			}
			write!(f, "] \n ")?;
		}
		write!(f, "] ")?;
		Ok(())
	}
}

impl<S,A> Add<&Matrix<A>> for Matrix<A>
where 
	S: Add<S, Output=S> + Copy,
	A: Array2<Scalar=S>,
{
	type Output = Matrix<A::Owned>;
	fn add(self, other: &Self) -> Self::Output {
		assert_eq!(self.shape(), other.shape(), "matrices dimensions mismatch in addition");
		Self::Output::from_field(|index|  self[index] + other[index])
	}
}
impl<S,A> Add<S> for Matrix<A> 
where 
	S: Add<S, Output=S> + Copy,
	A: Array2<Scalar=S>,
{
	type Output = Matrix<A::Owned>;
	fn add(self, other: S) -> Self::Output {		
		Self::Output::from_field(|index|  self[index] + other)
	}
}

