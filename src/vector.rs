// use std::ops::*;
// use std::iter::*;
// use num_traits::{Zero, One, Float};
// 
// use crate::prelude::*;
// 
// 
// /** Array1 interface, this trait allows the use of any data structure as a vector */
// pub trait Array1:
// 	Index<usize, Output=<Self as Array1>::Scalar>
// 	+IndexMut<usize, Output=<Self as Array1>::Scalar>
// {
// 	type Scalar: Sized + Copy;
// 	type Owned: Array1<Scalar=Self::Scalar> + Owned;
// 	
// 	fn len(&self) -> usize;
// }
// 
// /** wraps an object exposing the Array1 interface to perform vector operations */
// pub struct Vector<A: Array1> {
// 	pub array: A,
// }
// 
// // basic initializer
// 
// impl<A: Array1>  From<A> for Vector<A> {
// 	fn from(value: A) -> Self   {Self{array: value}}
// }
// impl<T, A1, A2>  From<&Vector<A2>> for Vector<A1> 
// where
// 	T: Clone,
// 	A1: Array1<Scalar=T> + Owned + Default,
// 	A2: Array1<Scalar=T>,
// {
// 	fn from(value: &Vector<A2>) -> Self  {Self::from_field(|i|  value[i].clone())}
// }
// 
// // items access
// 
// impl<A: Array1> Index<usize> for Vector<A> {
// 	type Output = A::Scalar;
// 	fn index(&self, i: usize) -> &Self::Output  {&self.array[i]}
// }
// impl<A: Array1> IndexMut<usize> for Vector<A> {
// 	fn index_mut(&mut self, i: usize) -> &mut Self::Output  {&mut self.array[i]}
// }
// 
// // general methods
// 
// impl<A: Array1> Vector<A> {
// 	/// value on the x axis (1st dimension)  (panics if this dimension does not exist)
//  	pub fn x<'a>(&'a self) -> &'a A::Scalar   {&self[0]}
//  	/// value on the y axis (2nd dimension)  (panics if this dimension does not exist)
//  	pub fn y<'a>(&'a self) -> &'a A::Scalar   {&self[1]}
//  	/// value on the z axis (3rd dimension)  (panics if this dimension does not exist)
//  	pub fn z<'a>(&'a self) -> &'a A::Scalar   {&self[2]}
//  	/// value on the w axis (4th dimension)  (panics if this dimension does not exist)
//  	pub fn w<'a>(&'a self) -> &'a A::Scalar   {&self[3]}
// }
// 
// impl<A: Array1> Vector<A> {
// 	pub fn len(&self) -> usize  {self.array.len()}
// }
// 
// impl<A: Array1 + Owned> Vector<A> {
// 	/// wrapper to A::empty
// 	pub fn empty() -> Self {
// 		Self::from(A::empty())
// 	}
// 	
// 	/// build a vector from a function returning a value each index
// 	pub fn from_field<F>(field: F) -> Self
// 	where F: Fn(usize) -> A::Scalar
// 	{
// 		let mut new = Self::empty();
// 		for i in 0..new.len() {
// 			new[i] = field(i);
// 		}
// 		new
// 	}
// 	
// 	/// build a vector full of the given value
// 	pub fn full(value: &A::Scalar) -> Self {
// 		Self::from_field(|_| *value)
// 	}
// }
// 
// // custom initializers
// 
// impl<S, A> Vector<A> 
// where 
// 	S: Zero + One,
// 	A: Array1<Scalar=S> + Owned,
// {
// 	/// unit vector in the ith direction  or null vector if this dimension does not exist
//  	pub fn ith(i: usize) -> Self {
// 		Self::from_field(|j| 
// 			if i==j  {A::Scalar::one()} 
// 			else {A::Scalar::zero()}
// 			)
//  	}
//  	/// unit X vector (1st dimension)  or null vector if this dimension does not exist
//  	#[allow(non_snake_case)]
//  	pub fn X() -> Self   {Self::ith(0)}
//  	/// unit Y vector (2nd dimension)  or null vector if this dimension does not exist
//  	#[allow(non_snake_case)]
//  	pub fn Y() -> Self   {Self::ith(1)}
//  	/// unit Z vector (3rd dimension)  or null vector if this dimension does not exist
//  	#[allow(non_snake_case)]
//  	pub fn Z() -> Self   {Self::ith(2)}
//  	/// unit W vector (4rd dimension)  or null vector if this dimension does not exist
//  	#[allow(non_snake_case)]
//  	pub fn W() -> Self   {Self::ith(3)}
// }
// 
// // vector operators
// 
// impl<S,A1,A2> Add<&Vector<A2>> for &Vector<A1>
// where 
// 	S: Add<S, Output=S> + Copy,
// 	A1: Array1<Scalar=S>,
// 	A2: Array1<Scalar=S>,
// {
// 	type Output = Vector<A1::Owned>;
// 	fn add(self, other: &Vector<A2>) -> Self::Output {
// 		assert_eq!(self.len(), other.len(), "vectors dimensions mismatch in addition");
// 		Self::Output::from_field(|index|  self[index] + other[index])
// 	}
// }
// impl<S,A> Add<&S> for &Vector<A> 
// where 
// 	S: Add<S, Output=S> + Copy,
// 	A: Array1<Scalar=S>,
// {
// 	type Output = Vector<A::Owned>;
// 	fn add(self, other: &S) -> Self::Output {		
// 		Self::Output::from_field(|index|  self[index] + *other)
// 	}
// }
// 
// 
// impl<S,A1,A2> Mul<&Vector<A2>> for &Vector<A1>
// where 
// 	S: Mul<S, Output=S> + Copy,
// 	A1: Array1<Scalar=S>,
// 	A2: Array1<Scalar=S>,
// {
// 	type Output = Vector<A1::Owned>;
// 	fn mul(self, other: &Vector<A2>) -> Self::Output {
// 		assert_eq!(self.len(), other.len(), "vectors dimensions mismatch in product");
// 		Self::Output::from_field(|index|  self[index] * other[index])
// 	}
// }
// impl<S,A> Mul<&S> for &Vector<A> 
// where 
// 	S: Mul<S, Output=S> + Copy,
// 	A: Array1<Scalar=S>,
// {
// 	type Output = Vector<A::Owned>;
// 	fn mul(self, other: &S) -> Self::Output {		
// 		Self::Output::from_field(|index|  self[index] * *other)
// 	}
// }
// 
// 
// impl<S,A1,A2> Div<&Vector<A2>> for &Vector<A1>
// where 
// 	S: Div<S, Output=S> + Copy,
// 	A1: Array1<Scalar=S>,
// 	A2: Array1<Scalar=S>,
// {
// 	type Output = Vector<A1::Owned>;
// 	fn div(self, other: &Vector<A2>) -> Self::Output {
// 		assert_eq!(self.len(), other.len(), "vectors dimensions mismatch in product");
// 		Self::Output::from_field(|index|  self[index] / other[index])
// 	}
// }
// impl<S,A> Div<&S> for &Vector<A> 
// where 
// 	S: Div<S, Output=S> + Copy,
// 	A: Array1<Scalar=S>,
// {
// 	type Output = Vector<A::Owned>;
// 	fn div(self, other: &S) -> Self::Output {		
// 		Self::Output::from_field(|index|  self[index] / *other)
// 	}
// }
// 
// 
// 
// impl<S,A> Vector<A>
// where
// 	S: Mul<S, Output=S> + Add<S, Output=S> + Sum<S> + Sub<S, Output=S> + Copy,
// 	A: Array1<Scalar=S>,
// {
// 	pub fn dot(&self, other: &Self) -> S {
// 		(self * other).iter().cloned().sum()
// 	}
// 	
// 	pub fn cross(&self, other: &Self) -> Vector<A::Owned> {
// 		assert_eq!(self.len(), 3, "the cross product only exists in dimension 3");
// 		let mut result = Vector::empty();
// 		result[0] = self[1]*other[2] - self[2]*other[1];
// 		result[1] = self[2]*other[0] - self[0]*other[2];
// 		result[2] = self[0]*other[1] - self[1]*other[0];
// 		result
// 	}
// }
// impl<S,A> Vector<A>
// where
// 	S: Float + Div<S, Output=S> + Add<S, Output=S> + Sum<S>,
// 	A: Array1<Scalar=S>,
// {
// 	
// 	/// return a vector with the same direction but length 1
// 	pub fn normalize(&self) -> Vector<A::Owned> {
// 		self / &self.length()
// 	}
// 	
// 	/// squared length of the vector
// 	pub fn length2(&self) -> S {
// 		self.dot(self)
// 	}
// 	
// 	/// lenght of the vector, this is the same as `normL1`
// 	pub fn length(&self) -> S {
// 		self.length2().sqrt()
// 	}
// 	
// 	/// L2 (euclidian) norm
// 	pub fn norm_l2(&self) -> S {
// 		self.dot(self).sqrt()
// 	}
// 	
// 	/// L1 norm
// 	pub fn norm_l1(&self) -> S {
// 		self.iter().cloned().map(Float::abs).reduce(Add::add)
// 			.expect("null dimension vector")
// 	}
// 	
// 	/// L infinite norm
// 	pub fn norm_lx(&self) -> S {
// 		self.iter().cloned().map(Float::abs).reduce(Float::max)
// 			.expect("null dimension vector")
// 	}
// }
// 
// struct VectorIterator<'a, A> {
// 	array: &'a A,
// 	index: usize,
// }
// impl<'a, S: 'a, A> Iterator for VectorIterator<'a, A>
// where
// 	A: Array1<Scalar=S>,
// {
// 	type Item = &'a S;
// 	fn next(&mut self) -> Option<Self::Item> {
// 		if self.index < self.array.len() {
// 			let i = self.index;
// 			self.index += 1;
// 			Some(&self.array[i])
// 		}
// 		else {
// 			None
// 		}
// 	}
// }
// impl<A: Array1> Vector<A> {
// 	fn iter(&self) -> VectorIterator<A>   {VectorIterator{
// 		array: &self.array, 
// 		index: 0,
// 	}}
// }
// 
