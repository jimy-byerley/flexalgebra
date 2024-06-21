use super::prelude::*;
use super::matrix::*;

pub struct SMatrix<T: Element, const R: usize, const C: usize> {
	data: [[T; C]; R],
}
impl<T: Element, const R: usize, const C: usize> 
	Matrix<T,Stat<R>,Stat<C>> for SMatrix<T,R,C>
{
	fn shape(&self) -> [usize; 2]   {[R, C]}
	fn strides(&self) -> [usize; 2] {[C, 1]}
	fn as_ptr(&self) -> *const T        {self.data.as_ptr() as _}
	fn as_mut_ptr(&mut self) -> *mut T  {self.data.as_mut_ptr() as _}
}

pub struct DMatrix<T: Element, R: Dim=Dyn, C: Dim=Dyn> {
	shape: (R, C),
	strides: (R, C),
	data: Vec<T>,
}
impl<T: Element, R: Dim, C: Dim> 
	Matrix<T,R,C> for DMatrix<T,R,C> 
{
	fn shape(&self) -> [usize; 2]    {[self.shape.0.value(), self.shape.1.value()]}
	fn strides(&self) -> [usize; 2]  {[self.strides.0.value(), self.strides.1.value()]}
	fn as_ptr(&self) -> *const T        {self.data.as_ptr()}
	fn as_mut_ptr(&mut self) -> *mut T  {self.data.as_mut_ptr()}
}

pub struct VMatrix<T: Element, R: Dim=Dyn, C: Dim=Dyn> {
	shape: (R, C),
	strides: (R, C),
	data: *mut T,
}
impl<T: Element, R: Dim, C: Dim>
	Matrix<T,R,C> for VMatrix<T,R,C>
{
	fn shape(&self) -> [usize; 2]    {[self.shape.0.value(), self.shape.1.value()]}
	fn strides(&self) -> [usize; 2]  {[self.strides.0.value(), self.strides.1.value()]}
	fn as_ptr(&self) -> *const T        {self.data as _}
	fn as_mut_ptr(&mut self) -> *mut T  {self.data}
}
impl<T: Scalar, R: Dim, C: Dim> dyn Matrix<T,R,C> {
	fn view(&self) -> VMatrix<T,R,C> {
		let shape = self.shape();
		let strides = self.strides();
		VMatrix {
			shape: (R::check(shape[0]).unwrap(), C::check(shape[1]).unwrap()),
			strides: (R::check(strides[0]).unwrap(), C::check(strides[1]).unwrap()),
			data: self.as_mut_ptr(),
		}
	}
}
