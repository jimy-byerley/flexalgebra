use std::ops::*;
use std::iter::Sum;


pub trait Matrix<T, R, C>: 
	Clone
	+ Index<[usize; 2], Output=T>
	+ IndexMut<[usize; 2]>
// 	+ Add<impl Matrix<T,R,C>, Output=Self::Owned<R,C>>
	+ Add<T, Output=Self::Owned<R,C>>
	+ Mul<T, Output=Self::Owned<R,C>>
	where T: Clone
{
	type Owned<R2, C2>: Matrix<T,R2,C2> + Default;
	
	fn shape(&self) -> [usize; 2];
	fn uninit<R2, C2>(shape: [usize; 2]) -> Self::Owned<R2, C2>;
	
	fn rows(&self) -> usize  {self.shape()[0]}
	fn columns(&self) -> usize  {self.shape()[1]}
	
	fn field<F>(mut self, field: F) -> Self 
		where F: Fn([usize; 2]) -> T
	{
		for i in 0 .. self.rows() {
			for j in 0 .. self.columns() {
				let index = [i,j];
				self[index] = field(index);
			}
		}
		self
	}
	
	fn full(self, value: T) -> Self {
		self.field(|_| value.clone())
	}

// 	fn zeros(self) -> Self {
// 		self.field(|_| T::Zero)
// 	}
// 	
// 	fn one(self) -> Self {
// 		self.field(|_| T::One)
// 	}
// 	
// 	fn identity(self) -> Self {
// 		self.field(|[i,j]|  if i==j {T::One} else {T::Zero})
// 	}
	
	fn matadd<'s, M>(&'s self, other: &'s M) -> Self::Owned<R,C>
		where 
			M: Matrix<T,R,C>,
			T: Add<Output=T>,
	{
		assert_eq!(self.shape(), other.shape());
		
		Self::uninit(self.shape())
			.field(|i|  self[i].clone() + other[i].clone())
	}
	
	fn matmul<'s,D,M>(&'s self, other: &'s M) -> Self::Owned<R,D>   
		where 
			M: Matrix<T,C,D>,
			T: Add<Output=T> + Mul<Output=T> + Sum<T>,
	{
		assert_eq!(self.shape()[1], other.shape()[0]);
		
		Self::uninit([self.shape()[0], other.shape()[1]])
			.field(|i|  
				(0 .. self.shape()[1])
				.map(|d|  self[[i[0], d]].clone() * other[[d, i[1]]].clone())
				.sum()
				)
	}
}

/*
impl<T,R,C, A,B> Add<B> for A
	where 
		T: Add<Output=T>,
		A: Matrix<T,R,C>,
		B: Matrix<T,R,C>,
{
	type Output = A::Owned<R,C>;
	fn add(&self, other: &B) -> Self::Output   {self.matadd(other)}
}
*/
/*
impl<T,R,C, B> Add<B> for dyn Matrix<T,R,C>
	where 
		T: Add<Output=T>,
		B: Matrix<T,R,C>,
{
	type Output = Self::Owned<R,C>;
	fn add(&self, other: &B) -> Self::Output 	{self.matadd(other)}
}
*/
/*
impl<T,R,C, A,B> Add<dyn Matrix<T,R,C>> for A
	where 
		T: Add,
		A: Matrix<T,R,C>,
{
	type Output = A::Owned<R,C>;
	fn add(&self, other: &dyn Matrix<T,R,C>) -> Self::Output {self.matadd(self, other)}
}
*/
/*
trait MatAdd<T,R,C, Rhs: Matrix<T,R,C>>: Matrix<T,R,C> 
	where T: Add<Output=T>
{
	fn matadd(&self, other: &Rhs) -> Self::Owned<R,C> {
		assert_eq!(self.shape(), other.shape());
		
		Self::Owned::default().field(|i|  self[i] + other[i])
	}
}

trait MatMul<T,R,D,C, Rhs: Matrix<T,D,C>>: Matrix<T,R,D> 
	where T: Add<Output=T> + Mul<Output=T> + Sum<T>
{
	fn matmul(&self, other: &Rhs) -> Self::Owned<R,C>   {
		assert_eq!(self.shape()[1], other.shape()[0]);
		
		Self::Owned::default().field(|i|  
			(0 .. self.shape()[1])
			.map(|d|  self[[i[0],d]] * other[[d, i[1]]])
			.sum()
			)
	}
}

use_matrix_operators!(SMatrix);
*/

pub trait Vector<T, D>:
	Clone + ToOwned
// 	+ Add<Vector<T,D>, Output=Vector<T,D>>
// 	+ Add<T, Output=Vector<T,D>>
// 	+ Mul<T, Output=Vector<T,D>>
	+ Index<usize, Output=T>
	+ IndexMut<usize>
{}

// impl Matrix<T, na::Const<1>, na::Const<1>>  for f64 {}
// impl Matrix<T, na::Const<1>, na::Const<1>>  for f32 {}
// impl<N> Matrix<T, N, na::Const<1>>  for Vector<T,N,1> {}




// fn main() {
// 
// 	let mat = SMatrix::<f32, 4, 4>::new().identity();
// 	let grid = SMatrix::<f32, 4, 3>::new().full(4.);
// 	let view = mat.view();
// 	let dmat = DMatrix::<f32>::new(12,10).identity();
// }




/*
trait Dim {
}

pub trait Matrix: 
	Clone
	+ Index<[usize; 2], Output=Self::T>
	+ IndexMut<[usize; 2]>
	+ Add<Matrix, Output=Self::Owned<Self::R, Self::C>>
// 	+ Add<Self::T, Output=Self::Owned<Self::R, Self::C>>
// 	+ Mul<Self::T, Output=Self::Owned<Self::R, Self::C>>
{
	type T: Clone;
	type R: Dim;
	type C: Dim;
	type Owned<R2, C2>: Matrix<T=Self::T, R=R2, C=C2> + Default;
}
*/
