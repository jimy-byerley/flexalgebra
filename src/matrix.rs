/*!
	Provide the fundamental matrix type and the underlying memory layout traits.
	
	It also implements the base data manipulation and conversion methods common to all matrices.
*/

use crate::prelude::*;

use core::iter::zip;
use core::ops::{Index, IndexMut};
use num_traits::{Zero, One};


/**
	The fundamental dense matrix type. 
	
	It is only a [newtype](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) wrapping an actual [Array] storage and adding all the matrix operations. The provided operations will depend on the traits the underlying array and its elements implements.
	
	This design has been adopted to overcome the [foreign implementation limit](https://doc.rust-lang.org/reference/items/implementations.html#orphan-rules) that would arrise if `Matrix` was a trait
	
	The indexing convention is always `[row, column]` whatever the underlying memory layout is
*/
#[derive(Clone)]
pub struct Matrix<A:Array> (pub A);

/**
	Expose a dense matrix layout, it can be implementing for any type. This is roughly the equivalent of the [buffer protocol](https://docs.python.org/3/c-api/buffer.html) in python specialized here for matrices
	
	The indexing convention is always `[row, column]` whatever the underlying memory layout is
*/
pub trait Array {
	/// array element type
	type Element: Element;
	/// rows dimensionality
	type R: Dim;
	/// columns dimensionality
	type C: Dim;
	
	/// size of the array in each dimension
	fn shape(&self) -> [usize; 2];
	/// step between values in each dimension
	fn strides(&self) -> [usize; 2];
	/// pointer to element `[0,0]` in the array
	fn as_ptr(&self) -> *const Self::Element;
}
/**
	Add the mutability to matrices contents for matrices based on this array, only supported by some array types
	
	Matrix based on such array supports inplace and filling operations
*/
pub trait ArrayMut: Array {
	/// MUST be the same pointer as [Array::as_ptr] but mutable
	fn as_mut_ptr(&mut self) -> *mut Self::Element;
}
/**
	Allow to instantiate an array and thus any matrix based on it.
	
	This typically adds constructors, and enables such type to be the result type of operators. Only supported by some array types
*/
pub trait ArrayOwned: ArrayMut {
	/// instantiate a new array with the given dimensions
    fn empty(shape: (Self::R, Self::C)) -> Self;
}
/**
	Designate the array type that will be used by operations applied on a matrix based on this array
	
	This is mendatory for all matrices involved in a non-inplace operation
*/
pub trait Compatible<R, C>: Array {
	type Owned: ArrayOwned<Element=Self::Element, R=R, C=C>;
}



impl<A:Array>
	Index<[usize; 2]> for Matrix<A>
{
	type Output = A::Element;
	/// `[row, column]` indexing
	fn index(&self, index: [usize; 2]) -> &Self::Output {
		assert!(zip(index, self.shape()).all(|(i,l)|  i<l));
		unsafe { &* self.as_ptr().add(zip(index, self.strides()).map(|(i,s)|  i*s).sum()) }
	}
}
impl<A:ArrayMut>
	IndexMut<[usize; 2]> for Matrix<A>
{
	fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
		assert!(zip(index, self.shape()).all(|(i,l)|  i<l));
		unsafe { &mut* self.as_mut_ptr().add(zip(index, self.strides()).map(|(i,s)|  i*s).sum()) }
	}
}
impl<A: ArrayOwned> Matrix<A> {
    pub fn new(shape: [usize;2]) -> Self {
        Self(A::empty((A::R::check(shape[0]).unwrap(), A::C::check(shape[1]).unwrap())))
    }
}


impl<A:Array> Matrix<A> {
	/// underlying array
	pub fn array(&self) -> &A   {&self.0}
	// forward array methods
	/// matrix size in each dimension
	pub fn shape(&self) -> [usize; 2]  {self.0.shape()}
	/// memory step between matrix elements (unit is the byte size of an element)
	pub fn strides(&self) -> [usize; 2]  {self.0.strides()}
	/// pointer to element `[0,0]`
	pub fn as_ptr(&self) -> *const A::Element  {self.0.as_ptr()}

	/// number of rows (same as `shape()[0]`)
	pub fn rows(&self) -> usize  {self.shape()[0]}
	/// number of columns (same as `shape()[1]`)
	pub fn columns(&self) -> usize  {self.shape()[1]}
	/// same as [Self::shape] but with [Dim] instances, this can be convenient when writing generics
	pub fn dimensionality(&self) -> (A::R, A::C) {(
		A::R::check(self.rows()).unwrap(), 
		A::C::check(self.columns()).unwrap(),
	)}
	/// number of elements in the matrix
	pub fn size(&self) -> usize {
		self.shape().iter().product()
	}
	
	/// number of elements in the matrix
	pub fn area(&self) -> usize {
		zip(self.shape(), self.strides()).map(|(l,s)|  (l-1)*s).sum::<usize>() + 1
	}
	/// `True` if the indexed memory is contiguous
	pub fn is_contiguous(&self) -> bool {
		let shape = self.shape();
		let strides = self.strides();
		shape[0]*strides[0] == strides[1]  ||  shape[1]*strides[1] == strides[0]
	}
	/// cast as a slice if the buffer is contiguous
	pub fn as_slice(&self) -> Option<&[A::Element]> {
		if self.is_contiguous() {Some(unsafe {self.as_slice_unchecked()})}
		else {None}
	}
	/// cast as a slice without contiguity check
	pub unsafe fn as_slice_unchecked(&self) -> &[A::Element] {
		core::slice::from_raw_parts(self.as_ptr(), self.area())
	}
}

impl<A:ArrayMut> Matrix<A> {
	// forward array methods
	/// same as [Self::as_ptr] but mutable
	pub fn as_mut_ptr(&mut self) -> *mut A::Element  {self.0.as_mut_ptr()}
	/// same as [Self::as_slice] but mutable
	pub fn as_slice_mut(&mut self) -> Option<&mut [A::Element]> {
		if self.is_contiguous() {Some(unsafe {self.as_slice_mut_unchecked()})}
		else {None}
	}
	/// same as [Self::as_slice_unchecked] but mutable
	pub unsafe fn as_slice_mut_unchecked(&mut self) -> &mut [A::Element] {
		core::slice::from_raw_parts_mut(self.as_mut_ptr(), self.area())
	}
	
	/// set every element in the matrix using the given closure
	pub fn set_field<F>(&mut self, mut field: F) -> &mut Self 
		where F: FnMut([usize; 2]) -> A::Element
	{
		for i in 0 .. self.rows() {
			for j in 0 .. self.columns() {
				let index = [i,j];
				self[index] = field(index);
			}
		}
		self
	}
	/// set every element in the array to the given value
	pub fn set_full(&mut self, value: A::Element) -> &mut Self {
		self.set_field(|_| value.clone())
	}
}

impl<A:ArrayMut> Matrix<A>
where A::Element: Scalar
{
	/// fill the matrix with zeros
	pub fn set_zero(&mut self) -> &mut Self {
		self.set_field(|_| Zero::zero())
	}
	/// fill the matrix with ones
	pub fn set_one(&mut self) -> &mut Self {
		self.set_field(|_| One::one())
	}
	/// fill the matrix with ones on the diagonal and zeros elsewhere
	pub fn set_identity(&mut self) -> &mut Self {
		self.set_field(|[i,j]|  if i==j {One::one()} else {Zero::zero()})
	}
}

impl<A, R:Dim, C:Dim> Matrix<A> 
where 
	A: Array<R=R,C=C> + Compatible<R,C>,
{
	/// copy this matrix data into a new matrix based on an [ArrayOwned]
	pub fn owned(&self) -> Matrix<A::Owned>  {Matrix::from(self)}
}
impl<Src, Dst:ArrayOwned> 
	From<&Matrix<Src>> for Matrix<Dst>
where
	Src: Array<Element=Dst::Element, R=Dst::R, C=Dst::C>
{
	fn from(src: &Matrix<Src>) -> Matrix<Dst> {
		let mut new = Matrix::new(src.shape());
		new.set_field(|i| src[i].clone());
		new
	}
}
// use std::error::Error;
// pub enum ConversionError<E: Error> {
// 	ColumnsMismatch,
// 	RowsMismatch,
// 	Element(E),
// }
// impl<Src:Array, Dst:ArrayOwned, E:Error> 
// 	TryFrom<&Matrix<Src>> for Matrix<Dst>
// where
// 	Src::Element: TryInto<Dst::Element, Error=E>
// {
// 	type Error = ConversionError<<Src::Element as TryInto<Dst::Element>>::Error>;
// 	fn try_from(src: &Matrix<Src>) -> Result<Matrix<Dst>, Self::Error> {
// 		let shape = src.shape();
// 		let mut dst = Matrix(Dst::empty((
// 			Dst::R::check(shape[0]).ok_or(ConversionError::RowsMismatch)?,
// 			Dst::C::check(shape[1]).ok_or(ConversionError::ColumnsMismatch)?,
// 			)));
// 		for i in 0 .. dst.rows() {
// 			for j in 0 .. dst.rows() {
// 				let index = [i,j];
// 				dst[index] = src[index].clone().try_into().map_err(|e| ConversionError::Element(e))?;
// 			}
// 		}
// 		Ok(dst)
// 	}
// }

