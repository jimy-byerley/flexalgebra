use super::prelude::*;
use super::matrix::*;
use super::owned::*;

use core::marker::PhantomData;


#[derive(Clone)]
pub struct Static<T: Element, const R: usize, const C: usize> {
	data: [[T; R]; C],
}
impl<T: Element, const R:usize, const C:usize> 
	Array for Static<T,R,C>
{
	type Element = T;
	type R = Stat<R>;
	type C = Stat<C>;
	fn shape(&self) -> [usize; 2]   {[R, C]}
	fn strides(&self) -> [usize; 2] {[1, R]}
	fn as_ptr(&self) -> *const T        {self.data.as_ptr() as _}
}	
impl<T: Element, const R:usize, const C:usize> 
	ArrayMut for Static<T,R,C>
{
	fn as_mut_ptr(&mut self) -> *mut T  {self.data.as_mut_ptr() as _}
}
impl<T: Element + Default + Copy, const R: usize, const C: usize> 
	ArrayOwned for Static<T,R,C>
{
	fn empty(shape: (Self::R, Self::C)) -> Self  {Self{data: [[T::default(); R]; C]}}
}
impl<T: Element + Default + Copy, const R1:usize, const C1:usize, const R2:usize, const C2:usize>
	Compatible<Stat<R2>, Stat<C2>> for Static<T,R1,C1>
{
	type Owned = Static<T,R2,C2>;
}

#[derive(Clone)]
pub struct Dynamic<T: Element, R: Dim=Dyn, C: Dim=Dyn> {
	shape: (R, C),
	data: Vec<T>,
}
impl<T: Element, R: Dim, C: Dim> 
	Array for Dynamic<T,R,C> 
{
	type Element = T;
	type R = R;
	type C = C;
	fn shape(&self) -> [usize; 2]    {[self.shape.0.value(), self.shape.1.value()]}
	fn strides(&self) -> [usize; 2]  {[1, self.shape.0.value()]}
	fn as_ptr(&self) -> *const T        {self.data.as_ptr()}
}	
impl<T: Element, R: Dim, C: Dim> 
	ArrayMut for Dynamic<T,R,C> 
{
	fn as_mut_ptr(&mut self) -> *mut T  {self.data.as_mut_ptr()}
}
impl<T: Element + Default, R: Dim, C: Dim> 
	ArrayOwned for Dynamic<T,R,C>
{
	fn empty(shape: (Self::R, Self::C)) -> Self {
		Self {
			shape,
			data: vec![T::default(); shape.0.value() * shape.1.value()],
		}
	}
}
impl<T: Element + Default, R1:Dim, C1:Dim, R2:Dim, C2:Dim>
	Compatible<R2,C2> for Dynamic<T,R1,C1>
{
	type Owned = Dynamic<T,R2,C2>;
}


#[derive(Copy, Clone, Debug)]
pub struct View<'t, T: Element, R: Dim=Dyn, C: Dim=Dyn> {
	shape: (R, C),
	strides: (R, C),
	data: *const T,
	lifetime: PhantomData<&'t T>,
}
impl<T: Element, R: Dim, C: Dim>
	Array for View<'_, T,R,C>
{
	type Element = T;
	type R = R;
	type C = C;
	fn shape(&self) -> [usize; 2]    {[self.shape.0.value(), self.shape.1.value()]}
	fn strides(&self) -> [usize; 2]  {[self.strides.0.value(), self.strides.1.value()]}
	fn as_ptr(&self) -> *const T        {self.data}
}

#[derive(Debug)]
pub struct ViewMut<'t, T: Element, R: Dim=Dyn, C: Dim=Dyn> {
	shape: (R, C),
	strides: (R, C),
	data: *mut T,
	lifetime: PhantomData<&'t mut T>,
}
impl<T: Element, R: Dim, C: Dim>
	Array for ViewMut<'_, T,R,C>
{
	type Element = T;
	type R = R;
	type C = C;
	fn shape(&self) -> [usize; 2]    {[self.shape.0.value(), self.shape.1.value()]}
	fn strides(&self) -> [usize; 2]  {[self.strides.0.value(), self.strides.1.value()]}
	fn as_ptr(&self) -> *const T        {self.data as _}
}
impl<T: Element, R: Dim, C: Dim>
	ArrayMut for ViewMut<'_, T,R,C>
{
	fn as_mut_ptr(&mut self) -> *mut T  {self.data}
}


impl<A:Array> Matrix<A> {
	pub fn view(&self) -> Matrix<View<'_, A::Element, A::R, A::C>> {
		let shape = self.shape();
		let strides = self.strides();
		Matrix(View {
			shape: (A::R::check(shape[0]).unwrap(), A::C::check(shape[1]).unwrap()),
			strides: (A::R::check(strides[0]).unwrap(), A::C::check(strides[1]).unwrap()),
			data: self.as_ptr(),
			lifetime: PhantomData,
		})
	}
}
impl<A:ArrayMut> Matrix<A> {
	pub fn view_mut(&mut self) -> Matrix<ViewMut<'_, A::Element, A::R, A::C>> {
		let shape = self.shape();
		let strides = self.strides();
		Matrix(ViewMut {
			shape: (A::R::check(shape[0]).unwrap(), A::C::check(shape[1]).unwrap()),
			strides: (A::R::check(strides[0]).unwrap(), A::C::check(strides[1]).unwrap()),
			data: self.as_mut_ptr(),
			lifetime: PhantomData,
		})
	}
}
