use super::{
    prelude::*,
    matrix::*,
    };
use core::{
    ops::*,
    fmt, cmp::max,
    };



impl<L:Array> Matrix<L>
where L::Element: Scalar
{
	pub fn add_to<'o, R, Out>(&self, right: &Matrix<R>, out: &'o mut Matrix<Out>) -> &'o mut Matrix<Out> 
	where 
		R: Array<Element=L::Element, R=L::R, C=L::C>,
		Out: ArrayMut<Element=L::Element, R=L::R, C=L::C>,
	{
		assert_eq!(self.shape(), right.shape());
		out.set_field(|i|  self[i].clone() + right[i].clone())
	}
	
	pub fn mul_to<'o, R, Out>(&self, right: &Matrix<R>, out: &'o mut Matrix<Out>) -> &'o mut Matrix<Out>
	where 
		R: Array<Element=L::Element, R=L::C>,
		Out: ArrayMut<Element=L::Element, R=L::R, C=R::C>
	{
		assert_eq!(self.shape()[1], right.shape()[0]);
		out.set_field(|i|  
				(0 .. self.shape()[1])
				.map(|d|  self[[i[0], d]].clone() * right[[d, i[1]]].clone())
				.reduce(Add::add).unwrap()
				)
	}
}


impl<L,R,RO,CO>
	Add<&Matrix<R>> for &Matrix<L>
where 
	L: Array<R=RO, C=CO> + Compatible<RO,CO>,
	R: Array<Element=L::Element, R=RO, C=CO>,
	L::Element: Scalar,
{
	type Output = Matrix<L::Owned>;
	fn add(self, right: &Matrix<R>) -> Self::Output {
		let mut new = Matrix::new(self.shape());
		self.add_to(right, &mut new);
		new
	}
}

impl<L,R,RO,CO> 
	Mul<&Matrix<R>> for &Matrix<L>
where 
	L: Array<R=RO> + Compatible<RO, CO>,
	R: Array<Element=L::Element, R=L::C, C=CO>,
	L::Element: Scalar,
{
	type Output = Matrix<L::Owned>;
	fn mul(self, right: &Matrix<R>) -> Self::Output {
		let mut new = Matrix::new([self.shape()[0], right.shape()[1]]);
		self.mul_to(right, &mut new);
		new
	}
}


#[test]
fn test_operators_static() {
    use super::matrices::*;
    
    let a = SMatrix::<f32,3,4>::identity();
    let b = SMatrix::<f32,4,2>::identity();
    let r = &a * &b;
    let b = SVector::<f32,4>::from([1.,2.,3.,4.]);
    let r = &a * &b;
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
