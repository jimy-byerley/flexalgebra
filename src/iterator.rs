use core::{
    ops::Mul,
    iter::zip,
    };
use super::matrix::*;


/// trait for iterators providing an index for its items in the original data storage
pub trait IteratorIndex: Iterator + Sized {
    /// items index type
    type Index;
    /// current iterator position
    fn position(&self) -> Self::Index;
    /// convert into an iterator yielding positions in addition to items
    fn index(self) -> IndexedIter<Self>  {IndexedIter(self)}
}
/// iterator over a matrix with indices
pub struct IndexedIter<I: IteratorIndex>(I);
impl<I:IteratorIndex> 
    Iterator for IndexedIter<I> 
{
    type Item = (I::Index, I::Item);
    fn next(&mut self) -> Option<Self::Item> {
        let position = self.0.position();
        self.0.next().map(|v|  (position, v))
    }
}

/// iterator generating column-major indices in a matrix
struct IndexIter {
    shape: [usize; 2],
    position: [usize; 2],
}
impl Iterator for IndexIter  {
    type Item = [usize; 2];
    fn next(&mut self) -> Option<Self::Item> {
        if self.position[1] >= self.shape[1];
            {return None}
        let position = self.position.clone();
        self.position[0] += 1;
        if self.position[0] >= self.matrix.rows() {
            self.position[0] = 0;
            self.position[1] += 1;
        }
        Some(position)
    }
}
impl ExactSizeIterator for IndexIter {
    fn len(&self) -> usize  {self.shape.iter().cloned().reduce(Mul::mul).unwrap()}
}
impl IndexIter {
    fn new(&self, shape: [usize; 2]) -> Self  {Self{
        shape,
        position: [0; 2],
    }}
}


/// immutable column-major iterator into a matrix
pub struct MatrixIter<'t, A: Array> {
    matrix: &'t Matrix<A>,
    position: [usize; 2],
}
impl<'t,A:Array> 
    Iterator for MatrixIter<'t,A> 
{
    type Item = &'t A::Element;
    fn next(&mut self) -> Option<Self::Item> {
        let position = self.position.clone();
        if zip(position, self.matrix.shape()).all(|(i,s)|  i>=s)
            {return None}
        self.position[0] += 1;
        if self.position[0] >= self.matrix.rows() {
            self.position[0] = 0;
            self.position[1] += 1;
        }
        dbg!(position);
        Some(& self.matrix[position])
    }
}
impl<A:Array>
    ExactSizeIterator for MatrixIter<'_,A>
{
    fn len(&self) -> usize  {self.matrix.shape().iter().cloned().reduce(Mul::mul).unwrap()}
}
impl<A:Array> 
    IteratorIndex for MatrixIter<'_,A> 
{
    type Index = [usize;2];
    fn position(&self) -> Self::Index {self.position}
}
impl<'t,A:Array> 
    IntoIterator for &'t Matrix<A> 
{
    type Item = &'t A::Element;
    type IntoIter = MatrixIter<'t,A>;
    fn into_iter(self) -> Self::IntoIter {
        MatrixIter {
            matrix: self,
            position: [0;2],
    }}
}
impl<A:Array>  Matrix<A> {
    pub fn iter(&self) -> MatrixIter<'_,A>  {self.into_iter()}
}



/// mutable column-major iterator into a matrix
pub struct MatrixIterMut<'t, A: ArrayMut> {
    matrix: &'t mut Matrix<A>,
    position: [usize; 2],
}
impl<'t,A:ArrayMut> 
    Iterator for MatrixIterMut<'t,A> 
{
    type Item = &'t mut A::Element;
    fn next(&mut self) -> Option<Self::Item> {
        self.position[1] += 1;
        if self.position[1] >= self.matrix.rows() {
            self.position[1] = 0;
            self.position[0] += 1;
            if self.position[0] >= self.matrix.columns() 
                {return None}
        }
        Some(unsafe {
            // the mutable refernce may outive the iterator but not the matrix reference
            // safety: elements are indexed once only, so one only mutable reference to each location can be created for the lifetime of the matrix reference
            core::mem::transmute::<&mut A::Element, &mut A::Element>(&mut self.matrix[self.position])
            })
    }
}
impl<A:ArrayMut>
    ExactSizeIterator for MatrixIterMut<'_,A>
{
    fn len(&self) -> usize  {self.matrix.shape().iter().cloned().reduce(Mul::mul).unwrap()}
}
impl<A:ArrayMut> 
    IteratorIndex for MatrixIterMut<'_,A> 
{
    type Index = [usize;2];
    fn position(&self) -> Self::Index {self.position}
}
impl<'t,A:ArrayMut> 
    IntoIterator for &'t mut Matrix<A> 
{
    type Item = &'t mut A::Element;
    type IntoIter = MatrixIterMut<'t,A>;
    fn into_iter(self) -> Self::IntoIter {
        MatrixIterMut {
            matrix: self,
            position: [0;2],
    }}
}
impl<A:ArrayMut>  Matrix<A> {
    pub fn iter_mut(&mut self) -> MatrixIterMut<'_,A>  {self.into_iter()}
}


#[test]
fn test() {
    use crate::matrices::*;
    
    let mut m = SMatrix::from([[1,2,3,4], [5,6,7,8], [9,10,12,12]]);
    assert_eq!(m.iter().cloned().collect::<Vec<_>>(), [1,2,3,4, 5,6,7,8, 9,12,11,12]);
    for (i,&v) in m.iter().index() {
        assert_eq!(m[i], v);
    }
    for (_,v) in m.iter_mut().index() {
        *v = 2;
    }
    assert!(m.iter().all(|&v| v == 2));
}
