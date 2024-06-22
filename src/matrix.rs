use crate::prelude::*;

use core::iter::zip;
use core::ops::*;
use std::fmt;
use num_traits::{Zero, One};


pub trait Matrix
{
	type Element: Element;
	type R: Dim;
	type C: Dim;
	
	fn shape(&self) -> [usize; 2];
	fn strides(&self) -> [usize; 2];
	fn as_ptr(&self) -> *const Self::Element;
}
pub trait MatrixMut: Matrix
{
	fn as_mut_ptr(&mut self) -> *mut Self::Element;
}


impl<M: Matrix>
	Index<[usize; 2]> for M
{
	type Output = M::Element;
	fn index(&self, index: [usize; 2]) -> &Self::Output {
		assert!(zip(index, self.shape()).all(|(i,l)|  i<l));
		unsafe { &* self.as_ptr().add(zip(index, self.strides()).map(|(i,s)|  i*s).sum()) }
	}
}
impl<M: MatrixMut>
	IndexMut<[usize; 2]> for M
{
	fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
		assert!(zip(index, self.shape()).all(|(i,l)|  i<l));
		unsafe { &mut* self.as_mut_ptr().add(zip(index, self.strides()).map(|(i,s)|  i*s).sum()) }
	}
}


impl<M: MatrixMut> M {
	pub fn rows(&self) -> usize  {self.shape()[0]}
	pub fn columns(&self) -> usize  {self.shape()[1]}
	
	pub fn field<F>(&mut self, field: F) -> &mut Self 
		where F: Fn([usize; 2]) -> M::Element
	{
		for i in 0 .. self.rows() {
			for j in 0 .. self.columns() {
				let index = [i,j];
				self[index] = field(index);
			}
		}
		self
	}
	
	pub fn fill(&mut self, value: M::Element) -> &mut Self {
		self.field(|_| value.clone())
	}
	
	pub fn area(&self) -> usize {
		zip(self.shape(), self.strides()).map(|(l,s)|  l*s).sum()
	}
	pub fn is_contiguous(&self) -> bool {
		let shape = self.shape();
		let strides = self.strides();
		shape[0]*strides[0] == strides[1]  ||  shape[1]*strides[1] == strides[0]
	}
	pub fn as_slice(&self) -> Option<&[M::Element]> {
		if self.is_contiguous() {Some(unsafe {self.as_slice_unsafe()})}
		else {None}
	}
	pub fn as_slice_mut(&mut self) -> Option<&mut [M::Element]> {
		if self.is_contiguous() {Some(unsafe {self.as_slice_mut_unsafe()})}
		else {None}
	}
	pub unsafe fn as_slice_unsafe(&self) -> &[M::Element] {
		core::slice::from_raw_parts(self.as_ptr(), self.area())
	}
	pub unsafe fn as_slice_mut_unsafe(&mut self) -> &mut [M::Element] {
		core::slice::from_raw_parts_mut(self.as_mut_ptr(), self.area())
	}
}

// impl<T,R,C> dyn Matrix<T,R,C> 
// where
// 	T: Scalar + Add + Sub + Mul,
// 	R: Dim,
// 	C: Dim,
// {
// 	fn add_to<'s, M>(&'s self, other: &'s M) -> Self::Owned<R,C>
// 	where 
// 		M: Matrix<T,R,C>,
// 		T: Add<Output=T>,
// 	{
// 		assert_eq!(self.shape(), other.shape());
// 		
// 		Self::uninit(self.shape())
// 			.field(|i|  self[i].clone() + other[i].clone())
// 	}
// 	
// 	fn mul_to<'s,D,M>(&'s self, other: &'s M) -> Self::Owned<R,D>   
// 	where 
// 		M: Matrix<T,C,D>,
// 		T: Add<Output=T> + Mul<Output=T>,
// 	{
// 		assert_eq!(self.shape()[1], other.shape()[0]);
// 		
// 		Self::uninit([self.shape()[0], other.shape()[1]])
// 			.field(|i|  
// 				(0 .. self.shape()[1])
// 				.map(|d|  self[[i[0], d]].clone() * other[[d, i[1]]].clone())
// 				.reduce(T::add)
// 				)
// 	}
// }
impl<M: Matrix> M
where M::Element: Zero
{
	pub fn zeros(&mut self) -> &mut Self {
		self.field(|_| Zero::zero())
	}
}
impl<M: Matrix> M
where M::Element: One
{
	pub fn one(&mut self) -> &mut Self {
		self.field(|_| One::one())
	}
}
impl<M: Matrix> M
where M::Element: Zero + One
{
	pub fn identity(&mut self) -> &mut Self {
		self.field(|[i,j]|  if i==j {One::one()} else {Zero::zero()})
	}
}
