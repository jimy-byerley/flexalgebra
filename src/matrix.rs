use crate::prelude::*;

use core::iter::zip;
use core::ops::*;
use std::fmt;
use num_traits::{Zero, One};


#[derive(Clone)]
pub struct Matrix<A:Array> (pub A);

pub trait Array
{
	type Element: Element;
	type R: Dim;
	type C: Dim;
	
	fn shape(&self) -> [usize; 2];
	fn strides(&self) -> [usize; 2];
	fn as_ptr(&self) -> *const Self::Element;
}
pub trait ArrayMut: Array
{
	fn as_mut_ptr(&mut self) -> *mut Self::Element;
}
pub trait ArrayOwned: ArrayMut {
    fn empty(shape: (Self::R, Self::C)) -> Self;
}
pub trait Compatible<R, C>: Array {
	type Owned: ArrayOwned<Element=Self::Element, R=R, C=C>;
}

impl<A: ArrayOwned> Matrix<A> {
    pub fn new(shape: [usize;2]) -> Self {
        Self(A::empty((A::R::check(shape[0]).unwrap(), A::C::check(shape[1]).unwrap())))
    }
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


impl<A:Array> Matrix<A> {
	// forward array methods
	pub fn shape(&self) -> [usize; 2]  {self.0.shape()}
	pub fn strides(&self) -> [usize; 2]  {self.0.strides()}
	pub fn as_ptr(&self) -> *const A::Element  {self.0.as_ptr()}

	pub fn rows(&self) -> usize  {self.shape()[0]}
	pub fn columns(&self) -> usize  {self.shape()[1]}
	
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
	
	pub fn set_field<F>(&mut self, field: F) -> &mut Self 
		where F: Fn([usize; 2]) -> A::Element
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
where A::Element: Zero
{
	pub fn set_zero(&mut self) -> &mut Self {
		self.set_field(|_| Zero::zero())
	}
}
impl<A:ArrayMut> Matrix<A>
where A::Element: One
{
	pub fn set_one(&mut self) -> &mut Self {
		self.set_field(|_| One::one())
	}
}
impl<A:ArrayMut> Matrix<A>
where A::Element: Zero + One
{
	pub fn set_identity(&mut self) -> &mut Self {
		self.set_field(|[i,j]|  if i==j {One::one()} else {Zero::zero()})
	}
}



impl<L:Array> Matrix<L>
where L::Element: Scalar
{
	pub fn add_to<'o, R, Out>(&self, right: &Matrix<R>, out: &'o mut Matrix<Out>) -> &'o mut Matrix<Out> 
	where 
		R: Array<Element=L::Element, R=L::R, C=L::C>,
		Out: ArrayMut<Element=L::Element, R=L::R, C=L::C>,
	{
		assert_eq!(self.shape(), right.shape());
		out.set_field(|i|  self[i].clone() + right[i].clone())
	}
	
	fn mul_to<'o, R, Out>(&self, right: &Matrix<R>, out: &'o mut Matrix<Out>) -> &'o mut Matrix<Out>
	where 
		R: Array<Element=L::Element, R=L::C>,
		Out: ArrayMut<Element=L::Element, R=L::R, C=R::C>
	{
		assert_eq!(self.shape()[1], right.shape()[0]);
		out.set_field(|i|  
				(0 .. self.shape()[1])
				.map(|d|  self[[i[0], d]].clone() * right[[d, i[1]]].clone())
				.reduce(Add::add).unwrap()
				)
	}
}


impl<L,R,RO,CO>
	Add<&Matrix<R>> for &Matrix<L>
where 
	L: Array<R=RO, C=CO> + Compatible<RO,CO>,
	R: Array<Element=L::Element, R=RO, C=CO>,
	L::Element: Scalar,
{
	type Output = Matrix<L::Owned>;
	fn add(self, right: &Matrix<R>) -> Self::Output {
		let mut new = Self::Output::new(self.shape());
		self.add_to(right, &mut new);
		new
	}
}

impl<L,R,RO,CO> 
	Mul<&Matrix<R>> for &Matrix<L>
where 
	L: Array<R=RO> + Compatible<RO, CO>,
	R: Array<Element=L::Element, R=L::C, C=CO>,
	L::Element: Scalar,
{
	type Output = Matrix<L::Owned>;
	fn mul(self, right: &Matrix<R>) -> Self::Output {
		let mut new = Self::Output::new([self.shape()[0], right.shape()[1]]);
		self.mul_to(right, &mut new);
		new
	}
}
