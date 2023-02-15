pub trait Matrix<T, R, C>: 
	Add<Matrix<T,R,C>, Output=Matrix<T,R,C>> 
	+ Mul<Matrix<T,R2,C2>, Output=Matrix<T,R2,C>>
	+ Add<T, Output=Matrix<T,R,C>>
	+ Mul<T, Output=Matrix<T,R,C>>
	+ Index<[usize; 2], Item=T>
	+ IndexMut<[usize; 2]>
	
pub trait AllocateProduct<T, R, C> {
	type Output = Matrix<T, R2, C>;
	fn allocate_product(&self, Matrix<T, R2, C2>) -> Self::Output;
}

pub trait AllocateSame<T, R, C> {
	type Output = Matrix<T, R, C>;
	fn allocate_same(&self) -> Self::Output;
}

pub trait Matrix<T, R, C>: 
	+ Index<[usize; 2], Item=T>
	+ IndexMut<[usize; 2]>
	+ PreProduct<T, R, C>
	+ ComponentWise<T, R, C>
{
	type Owned = Matrix<T, R, C>;
	type Product = Matrix<T, R2, C>;
	
	fn shape(&self) -> [usize; 2];
	fn allocate_owned(&self) -> Self::Owned;
	fn allocate_product(&self, Matrix<T, R2, C2>) -> Self::Product;
	
	fn field<F>(self, field: F) -> Self {
		for i in 0 .. self.rows() {
			for j in 0 .. self.columns() {
				let index = [i,j];
				self[index] = field(index);
			}
		}
		self
	}
	
	fn zeros(self) -> Self {
		self.field(|_| T::Zero)
	}
	
	fn one(self) -> Self {
		self.field(|_| T::One}
	}
	
	fn full(self, value: T) -> Self {
		self.field(|_| value}
	}

	fn identity(self) -> Self {
		self.field(|[i,j]|  if i==j {T::One} else {T::Zero})
	}
}

pub trait Vector<T, D>:
	Add<Vector<T,D>, Output=Vector<T,D>>
	+ Add<T, Output=Vector<T,D>>
	+ Mul<T, Output=Vector<T,D>>
	+ Index<usize, Item=T>
	+ IndexMut<usize>
{}

impl Matrix<T, na::Const<1>, na::Const<1>>  for f64 {}
impl Matrix<T, na::Const<1>, na::Const<1>>  for f32 {}
impl<N> Matrix<T, N, na::Const<1>>  for Vector<T,N,1> {}




fn main() {

	let mat = SMatrix<f32, 4, 4>::new().identity();
	let grid = SMatrix<f32, 4,3>::new().full(4.);
	let view = mat.view();
	let dmat = DMatrix<f32>::new(12,10).identity();
}
