use crate::prelude::*;

use core::iter::zip;
use core::ops::{Index, IndexMut};
use num_traits::{Zero, One};


#[derive(Clone)]
pub struct Matrix<A:Array> (pub A);

pub trait Array {
	type Element: Element;
	type R: Dim;
	type C: Dim;
	
	fn shape(&self) -> [usize; 2];
	fn strides(&self) -> [usize; 2];
	fn as_ptr(&self) -> *const Self::Element;
}
pub trait ArrayMut: Array {
	fn as_mut_ptr(&mut self) -> *mut Self::Element;
}
pub trait ArrayOwned: ArrayMut {
    fn empty(shape: (Self::R, Self::C)) -> Self;
}
pub trait Compatible<R, C>: Array {
	type Owned: ArrayOwned<Element=Self::Element, R=R, C=C>;
}



impl<A:Array>
	Index<[usize; 2]> for Matrix<A>
{
	type Output = A::Element;
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
	// forward array methods
	pub fn shape(&self) -> [usize; 2]  {self.0.shape()}
	pub fn strides(&self) -> [usize; 2]  {self.0.strides()}
	pub fn as_ptr(&self) -> *const A::Element  {self.0.as_ptr()}

	pub fn rows(&self) -> usize  {self.shape()[0]}
	pub fn columns(&self) -> usize  {self.shape()[1]}
	pub fn dimensionality(&self) -> (A::R, A::C) {(
		A::R::check(self.rows()).unwrap(), 
		A::C::check(self.columns()).unwrap(),
	)}
	
	pub fn area(&self) -> usize {
		zip(self.shape(), self.strides()).map(|(l,s)|  l*s).sum()
	}
	pub fn is_contiguous(&self) -> bool {
		let shape = self.shape();
		let strides = self.strides();
		shape[0]*strides[0] == strides[1]  ||  shape[1]*strides[1] == strides[0]
	}
	pub fn as_slice(&self) -> Option<&[A::Element]> {
		if self.is_contiguous() {Some(unsafe {self.as_slice_unchecked()})}
		else {None}
	}
	pub unsafe fn as_slice_unchecked(&self) -> &[A::Element] {
		core::slice::from_raw_parts(self.as_ptr(), self.area())
	}
}

impl<A:ArrayMut> Matrix<A> {
	// forward array methods
	pub fn as_mut_ptr(&mut self) -> *mut A::Element  {self.0.as_mut_ptr()}
	
	pub fn as_slice_mut(&mut self) -> Option<&mut [A::Element]> {
		if self.is_contiguous() {Some(unsafe {self.as_slice_mut_unchecked()})}
		else {None}
	}
	pub unsafe fn as_slice_mut_unchecked(&mut self) -> &mut [A::Element] {
		core::slice::from_raw_parts_mut(self.as_mut_ptr(), self.area())
	}
	
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
	
	pub fn set_full(&mut self, value: A::Element) -> &mut Self {
		self.set_field(|_| value.clone())
	}
}

impl<A:ArrayMut> Matrix<A>
where A::Element: Scalar
{
	pub fn set_zero(&mut self) -> &mut Self {
		self.set_field(|_| Zero::zero())
	}
	pub fn set_one(&mut self) -> &mut Self {
		self.set_field(|_| One::one())
	}
	pub fn set_identity(&mut self) -> &mut Self {
		self.set_field(|[i,j]|  if i==j {One::one()} else {Zero::zero()})
	}
}

impl<A, R:Dim, C:Dim> Matrix<A> 
where 
	A: Array<R=R,C=C> + Compatible<R,C>,
	A::Element: Clone,
{
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
