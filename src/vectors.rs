
use crate::prelude::*;
use crate::vector::*;
use std::ops::*;


pub struct Static1<T, const D: usize> {
	data: [T; D],
}

impl<T: Default + Sized + Copy, const D: usize> 
Array1 for Static1<T, D> {
	type Scalar = T;
	type Owned = Self;
	
	fn len(&self) -> usize  {D}
}
impl<T: Default + Sized + Copy, const D: usize> 
Owned for Static1<T, D> {
	fn empty() -> Self  {Static1{data: [T::default();D]}}
}
impl<T, const D: usize> 
Index<usize> for Static1<T, D> {
	type Output = T;
	fn index(&self, i: usize) -> &T  {&self.data[i]}
}
impl<T, const D: usize> 
IndexMut<usize> for Static1<T, D> {
	fn index_mut(&mut self, i: usize) -> &mut T  {&mut self.data[i]}
}


pub type Vec2<T> = Vector<Static1<T, 2>>;
pub type Vec3<T> = Vector<Static1<T, 3>>;
pub type Vec4<T> = Vector<Static1<T, 4>>;
