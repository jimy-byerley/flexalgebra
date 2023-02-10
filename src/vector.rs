use std::ops::*;
use num_traits::{Zero, One};

use crate::prelude::*;


/** Array1 interface, this trait allows the use of any data structure as a vector */
pub trait Array1:
	Index<usize, Output=<Self as Array1>::Scalar>
	+IndexMut<usize, Output=<Self as Array1>::Scalar>
{
	type Scalar: Sized + Copy;
	type Owned: Array1<Scalar=Self::Scalar> + Owned;
	
	fn len(&self) -> usize;
}

/** wraps an object exposing the Array1 interface to perform vector operations */
pub struct Vector<A: Array1> {
	pub array: A,
}

// basic initializer

impl<A: Array1>  From<A> for Vector<A> {
	fn from(value: A) -> Self   {Self{array: value}}
}
impl<T, A1, A2>  From<&Vector<A2>> for Vector<A1> 
where
	T: Clone,
	A1: Array1<Scalar=T> + Owned + Default,
	A2: Array1<Scalar=T>,
{
	fn from(value: &Vector<A2>) -> Self  {Self::from_field(|i|  value[i].clone())}
}

// items access

impl<A: Array1> Index<usize> for Vector<A> {
	type Output = A::Scalar;
	fn index(&self, i: usize) -> &Self::Output  {&self.array[i]}
}
impl<A: Array1> IndexMut<usize> for Vector<A> {
	fn index_mut(&mut self, i: usize) -> &mut Self::Output  {&mut self.array[i]}
}

// general methods

impl<A: Array1> Vector<A> {
 	pub fn x<'a>(&'a self) -> &'a A::Scalar   {&self[0]}
 	pub fn y<'a>(&'a self) -> &'a A::Scalar   {&self[1]}
 	pub fn z<'a>(&'a self) -> &'a A::Scalar   {&self[2]}
 	pub fn w<'a>(&'a self) -> &'a A::Scalar   {&self[3]}
}

impl<A: Array1> Vector<A> {
	pub fn len(&self) -> usize  {self.array.len()}
}

impl<A: Array1 + Owned> Vector<A> {
	/// build a vector from a function returning a value each index
	pub fn from_field<F>(field: F) -> Self
	where F: Fn(usize) -> A::Scalar
	{
		let mut new = Self::from(A::empty());
		for i in 0..new.len() {
			new[i] = field(i);
		}
		new
	}
	
	/// build a vector full of the given value
	pub fn full(value: &A::Scalar) -> Self {
		Self::from_field(|_| *value)
	}
}

// custom initializers

impl<S, A> Vector<A> 
where 
	S: Zero + One,
	A: Array1<Scalar=S> + Owned,
{
	/// unit vector in the ith direction
 	pub fn ith(i: usize) -> Self {
		Self::from_field(|j| 
			if i==j  {A::Scalar::one()} 
			else {A::Scalar::zero()}
			)
 	}
 	/// unit X vector (1st dimension)
 	#[allow(non_snake_case)]
 	pub fn X() -> Self   {Self::ith(0)}
 	/// unit Y vector (2nd dimension)
 	#[allow(non_snake_case)]
 	pub fn Y() -> Self   {Self::ith(1)}
 	/// unit Z vector (3rd dimension)
 	#[allow(non_snake_case)]
 	pub fn Z() -> Self   {Self::ith(2)}
 	/// unit W vector (4rd dimension)
 	#[allow(non_snake_case)]
 	pub fn W() -> Self   {Self::ith(3)}
}

// vector operators

impl<S,A> Add<&Vector<A>> for Vector<A>
where 
	S: Add<S, Output=S> + Copy,
	A: Array1<Scalar=S>,
{
	type Output = Vector<A::Owned>;
	fn add(self, other: &Self) -> Self::Output {
		assert_eq!(self.len(), other.len(), "vectors dimensions mismatch in addition");
		Self::Output::from_field(|index|  self[index] + other[index])
	}
}
impl<S,A> Add<S> for Vector<A> 
where 
	S: Add<S, Output=S> + Copy,
	A: Array1<Scalar=S>,
{
	type Output = Vector<A::Owned>;
	fn add(self, other: S) -> Self::Output {		
		Self::Output::from_field(|index|  self[index] + other)
	}
}

impl<S,A> Mul<&Vector<A>> for Vector<A>
where 
	S: Mul<S, Output=S> + Copy,
	A: Array1<Scalar=S>,
{
	type Output = Vector<A::Owned>;
	fn mul(self, other: &Self) -> Self::Output {
		assert_eq!(self.len(), other.len(), "vectors dimensions mismatch in product");
		Self::Output::from_field(|index|  self[index] * other[index])
	}
}
impl<S,A> Mul<S> for Vector<A> 
where 
	S: Mul<S, Output=S> + Copy,
	A: Array1<Scalar=S>,
{
	type Output = Vector<A::Owned>;
	fn mul(self, other: S) -> Self::Output {		
		Self::Output::from_field(|index|  self[index] * other)
	}
}
/*
impl<S,A> Vector<A>
where
	S: Mul<S, Output=S> + Add<S, Output=S> + Copy,
	A: Array1<Scalar=S>,
{
	pub fn dot(&self, other: &Self) -> S {
		(self * other).iter().sum()
	}
	
	pub fn cross(&self) -> Self {
		assert_eq!(self.len(), 3, "the cross product only exists in dimension 3");
	}
	
	pub fn normalize(&self) -> Self {
		self / self.length()
	}
	
	pub fn length2(&self) -> S {
		self.dot(self).iter().sum()
	}
	
	pub fn length(&self) -> S {
		self.length2().sqrt()
	}
	
	pub fn normL2(&self) -> S {
		self.dot(self).iter().sum().sqrt()
	}
	
	pub fn normL1(&self) -> S {
		self.iter().map(S::Scalar::abs).sum()
	}
	
	pub fn normLX(&self) -> S {
		self.iter().map(S::Scalar::abs).partial_max()
	}
}*/
