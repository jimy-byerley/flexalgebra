
use crate::prelude::*;
use crate::matrix::*;
use crate::vectors::*;
use std::ops::*;


// statically allocated matrix

pub struct Static2<T, const R: usize, const C: usize> {
	pub data: [[T; R]; C],
}

impl<T: Default + Sized + Copy, const R: usize, const C: usize> 
Array2 for Static2<T, R, C> {
	type Scalar = T;
	type Owned = Self;
	
	fn shape(&self) -> [usize; 2]  {[R, C]}
}
impl<T: Default + Sized + Copy, const R: usize, const C: usize> 
Owned for Static2<T, R, C> {
	fn empty() -> Self  {Static2{data: [[T::default();R];C]}}
}
impl<T, const R: usize, const C: usize> 
Index<[usize; 2]> for Static2<T, R, C> {
	type Output = T;
	fn index(&self, i: [usize; 2]) -> &T  {&self.data[i[0]][i[1]]}
}
impl<T, const R: usize, const C: usize> 
IndexMut<[usize; 2]> for Static2<T, R, C> {
	fn index_mut(&mut self, i: [usize; 2]) -> &mut T  {&mut self.data[i[0]][i[1]]}
}


pub type Mat2<T> = Matrix<Static2<T, 2, 2>>;
pub type Mat3<T> = Matrix<Static2<T, 3, 3>>;
pub type Mat4<T> = Matrix<Static2<T, 4, 4>>;

// storage-specific methods

impl<T: Default + Sized + Copy, const R: usize, const C: usize> 
Matrix<Static2<T, R, C>> {
	
// 	fn column<'a>(&'a self, i: usize) -> &'a Vector<&Static1<T, R>> {}
// 	fn row<'a>(&'a self, i: usize) -> &'a Vector<&View1<T, C>> {}
}


#[test]
fn test_static() {
	let mat = Mat4::<f32>::identity();
	println!("mat: {}", mat);
}




pub struct SViewMatrix<'a, T, const R: usize, const C: usize> {
	pub data: &'a [T],
	pub stride: [usize; 2],
}

pub struct DViewMatrix<'a, T: Default> {
	pub data: &'a [T],
	pub shape: [usize; 2],
	pub strides: [usize; 2],
}
