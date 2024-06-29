/*!
    implementation of most matrix operations and their traits
*/

use super::{
    prelude::*,
    matrix::*,
    };
use core::{
    ops::*,
    fmt, cmp::max,
    };


/// addition like [Add] but specifying output object
pub trait AddTo<T,O> {
    fn add_to<'o>(&self, right: &T, out: &'o mut O) -> &'o mut O;
}
/// difference like [Sub] but specifying output object
pub trait SubTo<T,O> {
    fn sub_to<'o>(&self, right: &T, out: &'o mut O) -> &'o mut O;
}
/// product like [Mul] but specifying output object
pub trait MulTo<T,O> {
    fn mul_to<'o>(&self, right: &T, out: &'o mut O) -> &'o mut O;
}
/// division like [Div] but specifying output object
pub trait DivTo<T,O> {
    fn div_to<'o>(&self, right: &T, out: &'o mut O) -> &'o mut O;
}




impl<L,R,RO,CO> 
	Mul<&Matrix<R>> for &Matrix<L>
where 
	L: Array<R=RO> + Compatible<RO, CO>,
	R: Array<Element=L::Element, R=L::C, C=CO>,
	L::Element: Scalar,
{
	type Output = Matrix<L::Owned>;
	/// matrix product
	fn mul(self, right: &Matrix<R>) -> Self::Output {
		let mut new = Matrix::new([self.shape()[0], right.shape()[1]]);
		self.mul_to(right, &mut new);
		new
	}
}
impl<L,R,O>
    MulTo<Matrix<R>,Matrix<O>> for Matrix<L>
where 
    L: Array<Element=O::Element, R=O::R>,
    R: Array<Element=O::Element, C=O::C>,
    O: ArrayMut,
    O::Element: Scalar,
{
    /// matrix product without dynamic allocation
	fn mul_to<'o>(&self, right: &Matrix<R>, out: &'o mut Matrix<O>) -> &'o mut Matrix<O> {
		assert_eq!(self.shape()[1], right.shape()[0]);
		out.set_field(|i|  
				(0 .. self.shape()[1])
				.map(|d|  self[[i[0], d]].clone() * right[[d, i[1]]].clone())
				.reduce(Add::add).unwrap()
				)
	}
}

macro_rules! elementwise_binop {
    ($trait:ident, $method:ident, $traitto:ident, $methodto:ident, $traitassign:ident, $methodassign:ident) => {
        impl<L,R>
            $trait<&Matrix<R>> for &Matrix<L>
        where 
            L: Array<Element=R::Element, R=R::R, C=R::C> + Compatible<R::R,R::C>,
            R: Array,
            R::Element: Scalar,
        {
            type Output = Matrix<L::Owned>;
            /// elementwise operation
            fn $method(self, right: &Matrix<R>) -> Self::Output {
                let mut new = Matrix::new(self.shape());
                self.$methodto(right, &mut new);
                new
            }
        }
        impl<L,R,O>
            $traitto<Matrix<R>,Matrix<O>> for Matrix<L>
        where
            L: Array<Element=O::Element, R=O::R, C=O::C>,
            R: Array<Element=O::Element, R=O::R, C=O::C>,
            O: ArrayMut,
            O::Element: Scalar,
        {
            /// elementwise operation without dynamic allocation
            fn $methodto<'o>(&self, right: &Matrix<R>, out: &'o mut Matrix<O>) -> &'o mut Matrix<O>
            {
                assert_eq!(self.shape(), right.shape());
                out.set_field(|i|  self[i].clone().$method(right[i].clone()))
            }
        }
        impl<L,R>
            $traitassign<&Matrix<R>> for Matrix<L>
        where
            L: ArrayMut,
            R: Array<Element=L::Element, R=L::R, C=L::C>,
            L::Element: Scalar,
        {
            /// inplace operation without dynamic allocation
            fn $methodassign(&mut self, right: &Matrix<R>) {
                assert_eq!(self.shape(), right.shape());
                let out = unsafe {core::mem::transmute::<&mut Self, &mut Self>(self)};
                self.$methodto(right, out);
            }
        }
    }
}
elementwise_binop!(Add, add, AddTo, add_to, AddAssign, add_assign);
elementwise_binop!(Sub, sub, SubTo, sub_to, SubAssign, sub_assign);

macro_rules! scalar_binop {
    ($trait:ident, $method:ident, $traitto:ident, $methodto:ident, $traitassign:ident, $methodassign:ident) => {
        impl<L,R,RO,CO>
            $trait<R> for &Matrix<L>
        where 
            L: Array<Element=R, R=RO, C=CO> + Compatible<RO,CO>,
            R: Scalar,
        {
            type Output = Matrix<L::Owned>;
            /// scalar operation
            fn $method(self, right: R) -> Self::Output {
                let mut new = Matrix::new(self.shape());
                self.$methodto(&right, &mut new);
                new
            }
        }
        impl<L,R,O>
            $traitto<R,Matrix<O>> for Matrix<L>
        where
            L: Array<Element=R, R=O::R, C=O::C>,
            R: Scalar,
            O: ArrayMut<Element=R>,
        {
            /// scalar operation without dynamic allocation
            fn $methodto<'o>(&self, right: &R, out: &'o mut Matrix<O>) -> &'o mut Matrix<O> {
                out.set_field(|i|  self[i].clone().$method(right.clone()))
            }
        }
        impl<L,R>
            $traitassign<R> for Matrix<L>
        where
            L: ArrayMut<Element=R>,
            R: Scalar,
        {
            /// inplace operation without dynamic allocation
            fn $methodassign(&mut self, right: R) {
                let out = unsafe {core::mem::transmute::<&mut Self, &mut Self>(self)};
                self.$methodto(&right, out);
            }
        }
    }
}
scalar_binop!(Add, add, AddTo, add_to, AddAssign, add_assign);
scalar_binop!(Sub, sub, SubTo, sub_to, SubAssign, sub_assign);
scalar_binop!(Mul, mul, MulTo, mul_to, MulAssign, mul_assign);
scalar_binop!(Div, div, DivTo, div_to, DivAssign, div_assign);




#[test]
fn test_operators_static() {
    use super::matrices::*;
    
    let a = SMatrix::<f32,3,4>::identity();
    let b = SMatrix::<f32,4,2>::identity();
    assert!((&a * &b).as_slice() == Some(&[1.,0.,0.,  0.,1.,0.]));
    let b = SVector::<f32,4>::from([1.,2.,3.,4.]);
    assert!((&a * &b).as_slice() == Some(&[1.0, 2.0, 3.0]));
    let c = SVector::<f32,3>::from([5.,6.,7.]);
    assert!((&(&a * &b) + &c).as_slice() == Some(&[6.,8.,10.]));
}
#[test]
fn test_operators_dynamic() {
    use super::matrices::*;
    
    let a = DMatrix::<f32>::identity([3,4]);
    let b = DMatrix::<f32,Dyn,Stat<2>>::identity(4);
    assert!((&a * &b).as_slice() == Some(&[1.,0.,0.,  0.,1.,0.]));
    let b = DVector::<f32>::from(vec![1.,2.,3.,4.]);
    assert!((&a * &b).as_slice() == Some(&[1.0, 2.0, 3.0]));
    let c = DVector::<f32>::from(vec![5.,6.,7.]);
    assert!((&(&a * &b) + &c).as_slice() == Some(&[6.,8.,10.]));
}


impl<A:Array>
    fmt::Debug for Matrix<A>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "<Matrix at {:?} shape:{:?} strides:{:?}>", 
            self.as_ptr(), self.shape(), self.strides())
    }
}

// original code from [nalgebra](https://docs.rs/nalgebra/latest/src/nalgebra/base/matrix.rs.html#1959)
macro_rules! impl_fmt {
    ($trait: path, $fmt_str_without_precision: expr, $fmt_str_with_precision: expr) => {
        impl<A:Array> $trait for Matrix<A>
        where
            A::Element: Scalar + $trait
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fn val_width<T: Scalar + $trait>(val: &T, f: &mut fmt::Formatter<'_>) -> usize {
                    match f.precision() {
                        Some(precision) => format!($fmt_str_with_precision, val, precision)
                            .chars()
                            .count(),
                        None => format!($fmt_str_without_precision, val).chars().count(),
                    }
                }

                let [nrows, ncols] = self.shape();

                if nrows == 0 || ncols == 0 {
                    return write!(f, "[ ]");
                }

                let mut max_length = 0;

                for i in 0..nrows {
                    for j in 0..ncols {
                        max_length = max(max_length, val_width(&self[[i, j]], f));
                    }
                }

                let max_length_with_space = max_length + 1;

                writeln!(f)?;
                writeln!(
                    f,
                    "  ┌ {:>width$} ┐",
                    "",
                    width = max_length_with_space * ncols - 1
                )?;

                for i in 0..nrows {
                    write!(f, "  │")?;
                    for j in 0..ncols {
                        let number_length = val_width(&self[[i, j]], f) + 1;
                        let pad = max_length_with_space - number_length;
                        write!(f, " {:>thepad$}", "", thepad = pad)?;
                        match f.precision() {
                            Some(precision) => {
                                write!(f, $fmt_str_with_precision, (*self)[[i, j]], precision)?
                            }
                            None => write!(f, $fmt_str_without_precision, (*self)[[i, j]])?,
                        }
                    }
                    writeln!(f, " │")?;
                }

                writeln!(
                    f,
                    "  └ {:>width$} ┘",
                    "",
                    width = max_length_with_space * ncols - 1
                )?;
                writeln!(f)
            }
        }
    };
}
impl_fmt!(fmt::Display, "{}", "{:.1$}");
impl_fmt!(fmt::LowerExp, "{:e}", "{:.1$e}");
impl_fmt!(fmt::UpperExp, "{:E}", "{:.1$E}");
impl_fmt!(fmt::Octal, "{:o}", "{:1$o}");
impl_fmt!(fmt::LowerHex, "{:x}", "{:1$x}");
impl_fmt!(fmt::UpperHex, "{:X}", "{:1$X}");
impl_fmt!(fmt::Binary, "{:b}", "{:.1$b}");
impl_fmt!(fmt::Pointer, "{:p}", "{:.1$p}");


#[test]
fn test_formats() {
    use super::matrices::*;
    let a = Matrix::<Dynamic<f32>>::identity([5, 6]);
    let b = Matrix::<Static<f32, 5, 6>>::identity();
    dbg!(&a, &b);
    println!("a = {}b = {}", &a, &b);
}
