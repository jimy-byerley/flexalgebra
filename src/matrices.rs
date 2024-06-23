/*!
	Provides the common [Array] implementations, type aliases for common matrices, as well as array-specific methods
	
	The structs are array implementations, each allows a different kind of matrix with a dedicated memory layout.
	The type aliases are for convenience
*/

use super::prelude::*;
use super::matrix::*;

use core::marker::PhantomData;


pub type SMatrix<T, const R:usize, const C:usize> = Matrix<Static<T,R,C>>;
pub type DMatrix<T> = Matrix<Dynamic<T,Dyn,Dyn>>;
pub type MatrixView<'t,T,R=Dyn,C=Dyn> = Matrix<View<'t,T,R,C>>;
pub type MatrixViewMut<'t,T,R=Dyn,C=Dyn> = Matrix<View<'t,T,R,C>>;
pub type SVector<T, const R:usize> = Matrix<Static<T,R,1>>;
pub type DVector<T> = Matrix<Dynamic<T,Dyn,Stat<1>>>;
pub type VectorView<'t,T,R=Dyn> = Matrix<View<'t,T,R,Stat<1>>>;
pub type VectorViewMut<'t,T,R=Dyn> = Matrix<ViewMut<'t,T,R,Stat<1>>>;



/// column-major statically sized owned array
#[derive(Clone)]
pub struct Static<T: Element, const R: usize, const C: usize> {
	pub data: [[T; R]; C],
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
impl<T:Scalar + Copy, const R:usize>
	From<[T;R]> for Matrix<Static<T,R,1>>
{
	fn from(src: [T;R]) -> Self {Matrix(Static{
		data: [src],
	})}
}
impl<T:Scalar + Copy, const R:usize, const C:usize>
	From<[[T;R];C]> for Matrix<Static<T,R,C>>
{
	fn from(src: [[T;R];C]) -> Self {Matrix(Static{
		data: src,
	})}
}



/// column-major dynamically allocated owned array, sizing can be dynamic or static
#[derive(Clone)]
pub struct Dynamic<T: Element, R: Dim=Dyn, C: Dim=Dyn> {
	pub shape: (R, C),
	pub data: Vec<T>,
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
impl<T:Element, R:Dim, C:Dim> 
	Matrix<Dynamic<T,R,C>>
{
	pub fn try_from_vec(shape: [usize;2], src: Vec<T>) -> Option<Self> {
		assert!(src.len() >= shape[0]*shape[1]);
		Some(Matrix(Dynamic{
			data: src,
			shape: (R::check(shape[0])?, C::check(shape[1])?),
		}))
	}
}
impl<T:Element>
	From<Vec<T>> for Matrix<Dynamic<T, Dyn, Stat<1>>>
{
	fn from(src: Vec<T>) -> Self  {Matrix(Dynamic{
		shape: (Dyn(src.len()), Stat{}),
		data: src,
	})}
}


/// array referncing an immutable borrowed memory buffer
#[derive(Copy, Clone, Debug)]
pub struct View<'t, T: Element, R: Dim=Dyn, C: Dim=Dyn> {
	pub shape: (R, C),
	pub strides: (usize, usize),
	pub data: *const T,
	pub lifetime: PhantomData<&'t T>,
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
	/// create an immutable view on this matrix, this is useful for forwarding the matrix without moving its content
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
	/// create a view on this matrix with a new shape and dimensionality, or `None` if it is not possible due to stride or size reasons
	pub fn reshape<R2:Dim, C2:Dim>(&self, shape: [usize;2]) -> Option<Matrix<View<'_, A::Element, R2,C2>>> {
		let strides = self.strides();
		let step = strides[0].min(strides[1]);
		if (strides[0] % step) != 0 || (strides[1] % step) != 0 
			{return None}
		let previous = self.shape();
		if previous[0]*previous[1] != shape[0]*shape[1]
			{return None}
		Some(Matrix(View {
			shape: (R2::check(shape[0])?, C2::check(shape[1])?),
			strides: (step, step*shape[0]),
			data: self.as_ptr(),
			lifetime: PhantomData,
		}))
	}
	/// create a view with columns and rows switched
	pub fn transpose(&self) -> Matrix<View<'_, A::Element, A::C, A::R>> {
		let dim = self.dimensionality();
		let strides = self.strides();
		Matrix(View {
			shape: (dim.1, dim.0),
			strides: (strides[1], strides[0]),
			data: self.as_ptr(),
			lifetime: PhantomData,
		})
	}
}
impl<'t, T:Element, R:Dim, C:Dim> 
	Matrix<View<'t,T,R,C>>
{
	/**
		immutable view in a slice buffer, with specified shape
		
		may fail when the shape is too big to hold in the slice's length
	*/
	pub fn try_from_slice(shape: [usize;2], src: &'t [T]) -> Option<Self> {
		assert!(src.len() >= shape[0]*shape[1]);
		Some(Matrix(View{
			data: src.as_ptr(),
			shape: (R::check(shape[0])?, C::check(shape[1])?),
			strides: (1, shape[0]),
			lifetime: PhantomData,
		}))
	}
	/**
		immutable view in a slice buffer, with specified shape and strides
		
		may fail when the shape*strides is too big to hold in the slice's length
	*/
	pub fn try_from_strides(shape: [usize;2], strides: [usize;2], src: &'t [T]) -> Option<Self> {
		assert!(src.len() >= shape[0]*strides[0].max(shape[1]*strides[1]));
		Some(Matrix(View{
			data: src.as_ptr(),
			shape: (R::check(shape[0])?, C::check(shape[1])?),
			strides: (strides[0], strides[1]),
			lifetime: PhantomData,
		}))
	}
}
impl<'t, T:Element>
	From<&[T]> for Matrix<View<'t, T, Dyn, Stat<1>>>
{
	fn from(src: &[T]) -> Self  {Matrix(View{
		data: src.as_ptr(),
		shape: (Dyn(src.len()), Stat{}),
		strides: (1, src.len()),
		lifetime: PhantomData,
	})}
}


/// array referncing a mutable borrowed memory buffer
#[derive(Debug)]
pub struct ViewMut<'t, T: Element, R: Dim=Dyn, C: Dim=Dyn> {
	pub shape: (R, C),
	pub strides: (usize, usize),
	pub data: *mut T,
	pub lifetime: PhantomData<&'t mut T>,
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
	/// create an immutable view on this matrix, this is useful for forwarding the matrix without moving its content
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
	/// create a view on this matrix with a new shape and dimensionality, or `None` if it is not possible due to stride or size reasons
	pub fn reshape_mut<R2:Dim, C2:Dim>(&mut self, shape: [usize;2]) -> Option<Matrix<ViewMut<'_, A::Element, R2,C2>>> {
		let strides = self.strides();
		let step = strides[0].min(strides[1]);
		if (strides[0] % step) != 0 || (strides[1] % step) != 0 
			{return None}
		let previous = self.shape();
		if previous[0]*previous[1] != shape[0]*shape[1]
			{return None}
		Some(Matrix(ViewMut {
			shape: (R2::check(shape[0])?, C2::check(shape[1])?),
			strides: (step, step*shape[0]),
			data: self.as_mut_ptr(),
			lifetime: PhantomData,
		}))
	}
	/// create a view with columns and rows switched
	pub fn transpose_mut(&mut self) -> Matrix<ViewMut<'_, A::Element, A::C, A::R>> {
		let dim = self.dimensionality();
		let strides = self.strides();
		Matrix(ViewMut {
			shape: (dim.1, dim.0),
			strides: (strides[1], strides[0]),
			data: self.as_mut_ptr(),
			lifetime: PhantomData,
		})
	}
}
impl<'t, T:Element, R:Dim, C:Dim> 
	Matrix<ViewMut<'t,T,R,C>>
{
	/**
		mutable view in a slice buffer, with specified shape
		
		may fail when the shape is too big to hold in the slice's length
	*/
	pub fn try_from_slice(shape: [usize;2], src: &'t mut [T]) -> Option<Self> {
		assert!(src.len() >= shape[0]*shape[1]);
		Some(Matrix(ViewMut{
			data: src.as_mut_ptr(),
			shape: (R::check(shape[0])?, C::check(shape[1])?),
			strides: (1, shape[0]),
			lifetime: PhantomData,
		}))
	}
	/**
		mutable view in a slice buffer, with specified shape and strides
		
		may fail when the shape*strides is too big to hold in the slice's length
	*/
	pub fn try_from_strides(shape: [usize;2], strides: [usize;2], src: &'t mut [T]) -> Option<Self> {
		assert!(src.len() >= shape[0]*strides[0].max(shape[1]*strides[1]));
		Some(Matrix(ViewMut{
			data: src.as_mut_ptr(),
			shape: (R::check(shape[0])?, C::check(shape[1])?),
			strides: (strides[0], strides[1]),
			lifetime: PhantomData,
		}))
	}
}
impl<'t, T:Element>
	From<&mut [T]> for Matrix<ViewMut<'t, T, Dyn, Stat<1>>>
{
	fn from(src: &mut [T]) -> Self  {Matrix(ViewMut{
		data: src.as_mut_ptr(),
		shape: (Dyn(src.len()), Stat{}),
		strides: (1, src.len()),
		lifetime: PhantomData,
	})}
}



#[test]
fn test_constructors() {
    let a = Matrix::<Dynamic<f32>>::zeros([5, 6]);
    let b = Matrix::<Static<f32, 5, 6>>::zeros();
    dbg!(a, b);
}

