/*!
    Convenient type aliases and functions to mimic the [GLSL API](https://www.khronos.org/opengl/wiki/Core_Language_(GLSL))
*/

use super::matrix::*;
use super::matrices::*;

pub type Vec1<T=f32> = Matrix<Static<T,1,1>>;
pub type Vec2<T=f32> = Matrix<Static<T,2,1>>;
pub type Vec3<T=f32> = Matrix<Static<T,3,1>>;
pub type Vec4<T=f32> = Matrix<Static<T,4,1>>;

pub type Mat1<T=f32> = Matrix<Static<T,1,1>>;
pub type Mat2<T=f32> = Matrix<Static<T,2,2>>;
pub type Mat3<T=f32> = Matrix<Static<T,3,3>>;
pub type Mat4<T=f32> = Matrix<Static<T,4,4>>;

pub type FVec1 = Vec1<f32>;
pub type FVec2 = Vec2<f32>;
pub type FVec3 = Vec3<f32>;
pub type FVec4 = Vec4<f32>;
pub type FMat1 = Mat1<f32>;
pub type FMat2 = Mat2<f32>;
pub type FMat3 = Mat3<f32>;
pub type FMat4 = Mat4<f32>;

pub type DVec1 = Vec1<f64>;
pub type DVec2 = Vec2<f64>;
pub type DVec3 = Vec3<f64>;
pub type DVec4 = Vec4<f64>;
pub type DMat1 = Mat1<f64>;
pub type DMat2 = Mat2<f64>;
pub type DMat3 = Mat3<f64>;
pub type DMat4 = Mat4<f64>;
