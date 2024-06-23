use super::prelude::*;
use super::matrix::*;

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
	fn empty(_: (Self::R, Self::C)) -> Self  {Self{data: [[T::default(); R]; C]}}
}
impl<T: Element + Default + Copy, const R1:usize, const C1:usize, const R2:usize, const C2:usize>
	Compatible<Stat<R2>, Stat<C2>> for Static<T,R1,C1>
{
	type Owned = Static<T,R2,C2>;
}
/// constructors, they should ideally be placed in crate::matrix and benefit all impls instead of here, but [rust doesn't allow it yet](https://users.rust-lang.org/t/methods-implemented-for-specialized-structs-are-said-duplicates-when-specializing-over-exclusive-traits/113315/4)
impl<T:Scalar + Copy + Default, const R:usize, const C:usize> 
	Matrix<Static<T, R, C>>
{
	pub fn empty() -> Self         {Matrix::new([R,C])}
	pub fn zeros() -> Self         {let mut new = Matrix::new([R,C]); new.set_zero(); new}
	pub fn ones() -> Self          {let mut new = Matrix::new([R,C]); new.set_one(); new}
	pub fn identity() -> Self      {let mut new = Matrix::new([R,C]); new.set_identity(); new}
	pub fn full(value: T) -> Self  {let mut new = Matrix::new([R,C]); new.set_full(value); new}
	pub fn field<F>(field: F) -> Self     
	where F: FnMut([usize; 2]) -> T
		{let mut new = Matrix::new([R,C]); new.set_field(field); new}
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
/// constructors, they should ideally be placed in crate::matrix and benefit all impls instead of here, but [rust doesn't allow it yet](https://users.rust-lang.org/t/methods-implemented-for-specialized-structs-are-said-duplicates-when-specializing-over-exclusive-traits/113315/4)
impl<T:Scalar + Default, const R:usize, const C:usize> 
	Matrix<Dynamic<T, Stat<R>, Stat<C>>>
{
	pub fn empty() -> Self         {Matrix::new([R,C])}
	pub fn zeros() -> Self         {let mut new = Matrix::new([R,C]); new.set_zero(); new}
	pub fn ones() -> Self          {let mut new = Matrix::new([R,C]); new.set_one(); new}
	pub fn identity() -> Self      {let mut new = Matrix::new([R,C]); new.set_identity(); new}
	pub fn full(value: T) -> Self  {let mut new = Matrix::new([R,C]); new.set_full(value); new}
	pub fn field<F>(field: F) -> Self     
	where F: FnMut([usize; 2]) -> T
		{let mut new = Matrix::new([R,C]); new.set_field(field); new}
}
impl<T:Scalar + Default, const C:usize> 
	Matrix<Dynamic<T, Dyn, Stat<C>>>
{
	pub fn empty(size: usize) -> Self         {Matrix::new([size,C])}
	pub fn zeros(size: usize) -> Self         {let mut new = Matrix::new([size,C]); new.set_zero(); new}
	pub fn ones(size: usize) -> Self          {let mut new = Matrix::new([size,C]); new.set_one(); new}
	pub fn identity(size: usize) -> Self      {let mut new = Matrix::new([size,C]); new.set_identity(); new}
	pub fn full(size: usize, value: T) -> Self  {let mut new = Matrix::new([size,C]); new.set_full(value); new}
	pub fn field<F>(size: usize, field: F) -> Self     
	where F: FnMut([usize; 2]) -> T
		{let mut new = Matrix::new([size,C]); new.set_field(field); new}
}
impl<T:Scalar + Default, const R:usize> 
	Matrix<Dynamic<T, Stat<R>, Dyn>>
{
	pub fn empty(size: usize) -> Self         {Matrix::new([R,size])}
	pub fn zeros(size: usize) -> Self         {let mut new = Matrix::new([R,size]); new.set_zero(); new}
	pub fn ones(size: usize) -> Self          {let mut new = Matrix::new([R,size]); new.set_one(); new}
	pub fn identity(size: usize) -> Self      {let mut new = Matrix::new([R,size]); new.set_identity(); new}
	pub fn full(size: usize, value: T) -> Self  {let mut new = Matrix::new([R,size]); new.set_full(value); new}
	pub fn field<F>(size: usize, field: F) -> Self     
	where F: FnMut([usize; 2]) -> T
		{let mut new = Matrix::new([R,size]); new.set_field(field); new}
}
impl<T:Scalar + Default> 
	Matrix<Dynamic<T, Dyn, Dyn>>
{
	pub fn empty(shape: [usize; 2]) -> Self         {Matrix::new(shape)}
	pub fn zeros(shape: [usize; 2]) -> Self         {let mut new = Matrix::new(shape); new.set_zero(); new}
	pub fn ones(shape: [usize; 2]) -> Self          {let mut new = Matrix::new(shape); new.set_one(); new}
	pub fn identity(shape: [usize; 2]) -> Self      {let mut new = Matrix::new(shape); new.set_identity(); new}
	pub fn full(shape: [usize; 2], value: T) -> Self  {let mut new = Matrix::new(shape); new.set_full(value); new}
	pub fn field<F>(shape: [usize; 2], field: F) -> Self     
	where F: FnMut([usize; 2]) -> T
		{let mut new = Matrix::new(shape); new.set_field(field); new}
}



#[derive(Copy, Clone, Debug)]
pub struct View<'t, T: Element, R: Dim=Dyn, C: Dim=Dyn> {
	shape: (R, C),
	strides: (usize, usize),
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
	fn strides(&self) -> [usize; 2]  {[self.strides.0, self.strides.1]}
	fn as_ptr(&self) -> *const T        {self.data}
}
impl<T: Element + Default, R1:Dim, C1:Dim, R2:Dim, C2:Dim>
	Compatible<R2,C2> for View<'_,T,R1,C1>
{
	type Owned = Dynamic<T,R2,C2>;
}
impl<A:Array> Matrix<A> {
	pub fn view(&self) -> Matrix<View<'_, A::Element, A::R, A::C>> {
		let shape = self.shape();
		let strides = self.strides();
		Matrix(View {
			shape: (A::R::check(shape[0]).unwrap(), A::C::check(shape[1]).unwrap()),
			strides: (strides[0], strides[1]),
			data: self.as_ptr(),
			lifetime: PhantomData,
		})
	}
}



#[derive(Debug)]
pub struct ViewMut<'t, T: Element, R: Dim=Dyn, C: Dim=Dyn> {
	shape: (R, C),
	strides: (usize, usize),
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
	fn strides(&self) -> [usize; 2]  {[self.strides.0, self.strides.1]}
	fn as_ptr(&self) -> *const T        {self.data as _}
}
impl<T: Element + Default, R1:Dim, C1:Dim, R2:Dim, C2:Dim>
	Compatible<R2,C2> for ViewMut<'_,T,R1,C1>
{
	type Owned = Dynamic<T,R2,C2>;
}
impl<T: Element, R: Dim, C: Dim>
	ArrayMut for ViewMut<'_, T,R,C>
{
	fn as_mut_ptr(&mut self) -> *mut T  {self.data}
}
impl<A:ArrayMut> Matrix<A> {
	pub fn view_mut(&mut self) -> Matrix<ViewMut<'_, A::Element, A::R, A::C>> {
		let shape = self.shape();
		let strides = self.strides();
		Matrix(ViewMut {
			shape: (A::R::check(shape[0]).unwrap(), A::C::check(shape[1]).unwrap()),
			strides: (strides[0], strides[1]),
			data: self.as_mut_ptr(),
			lifetime: PhantomData,
		})
	}
}


#[test]
fn test_constructors() {
    let a = Matrix::<Dynamic<f32>>::zeros([5, 6]);
    let b = Matrix::<Static<f32, 5, 6>>::zeros();
    dbg!(a, b);
}

