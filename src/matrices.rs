use super::prelude::*;
use super::matrix::*;

use core::marker::PhantomData;



pub struct SMatrix<T: Element, const R: usize, const C: usize> {
	data: [[T; C]; R],
}
impl<T: Element, const R: usize, const C: usize> 
	Matrix for SMatrix<T,R,C>
{
	type Element = T;
	type R = Stat<R>;
	type C = Stat<C>;
	fn shape(&self) -> [usize; 2]   {[R, C]}
	fn strides(&self) -> [usize; 2] {[C, 1]}
	fn as_ptr(&self) -> *const T        {self.data.as_ptr() as _}
}	
impl<T: Element, const R: usize, const C: usize> 
	MatrixMut for SMatrix<T,R,C>
{
	fn as_mut_ptr(&mut self) -> *mut T  {self.data.as_mut_ptr() as _}
}

pub struct DMatrix<T: Element, R: Dim=Dyn, C: Dim=Dyn> {
	shape: (R, C),
	strides: (R, C),
	data: Vec<T>,
}
impl<T: Element, R: Dim, C: Dim> 
	Matrix for DMatrix<T,R,C> 
{
	type Element = T;
	type R = R;
	type C = C;
	fn shape(&self) -> [usize; 2]    {[self.shape.0.value(), self.shape.1.value()]}
	fn strides(&self) -> [usize; 2]  {[self.strides.0.value(), self.strides.1.value()]}
	fn as_ptr(&self) -> *const T        {self.data.as_ptr()}
}	
impl<T: Element, R: Dim, C: Dim> 
	MatrixMut for DMatrix<T,R,C> 
{
	fn as_mut_ptr(&mut self) -> *mut T  {self.data.as_mut_ptr()}
}



pub struct ViewMatrix<'t, T: Element, R: Dim=Dyn, C: Dim=Dyn> {
	shape: (R, C),
	strides: (R, C),
	data: *const T,
	lifetime: PhantomData<&'t T>,
}
impl<T: Element, R: Dim, C: Dim>
	Matrix for ViewMatrix<'_, T,R,C>
{
	type Element = T;
	type R = R;
	type C = C;
	fn shape(&self) -> [usize; 2]    {[self.shape.0.value(), self.shape.1.value()]}
	fn strides(&self) -> [usize; 2]  {[self.strides.0.value(), self.strides.1.value()]}
	fn as_ptr(&self) -> *const T        {self.data}
}

pub struct ViewMatrixMut<'t, T: Element, R: Dim=Dyn, C: Dim=Dyn> {
	shape: (R, C),
	strides: (R, C),
	data: *mut T,
	lifetime: PhantomData<&'t mut T>,
}
impl<T: Element, R: Dim, C: Dim>
	Matrix for ViewMatrixMut<'_, T,R,C>
{
	type Element = T;
	type R = R;
	type C = C;
	fn shape(&self) -> [usize; 2]    {[self.shape.0.value(), self.shape.1.value()]}
	fn strides(&self) -> [usize; 2]  {[self.strides.0.value(), self.strides.1.value()]}
	fn as_ptr(&self) -> *const T        {self.data as _}
}
impl<T: Element, R: Dim, C: Dim>
	MatrixMut for ViewMatrixMut<'_, T,R,C>
{
	fn as_mut_ptr(&mut self) -> *mut T  {self.data}
}


impl dyn Matrix {
	fn view(&self) -> ViewMatrix<M::Element, M::R, M::C> {
		let shape = self.shape();
		let strides = self.strides();
		ViewMatrix {
			shape: (M::R::check(shape[0]).unwrap(), M::C::check(shape[1]).unwrap()),
			strides: (M::R::check(strides[0]).unwrap(), M::C::check(strides[1]).unwrap()),
			data: self.as_ptr(),
		}
	}
}
impl dyn MatrixMut {
	fn view_mut(&mut self) -> ViewMatrixMut<M::Element, M::R, M::C> {
		let shape = self.shape();
		let strides = self.strides();
		ViewMatrixMut {
			shape: (M::R::check(shape[0]).unwrap(), M::C::check(shape[1]).unwrap()),
			strides: (M::R::check(strides[0]).unwrap(), M::C::check(strides[1]).unwrap()),
			data: self.as_mut_ptr(),
		}
	}
}
