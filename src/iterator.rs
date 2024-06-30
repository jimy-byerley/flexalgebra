use core::ops::Mul;
use super::matrix::*;


/// iterator generating column-major indices in a matrix
pub struct IndexIter {
    shape: [usize; 2],
    position: [usize; 2],
}
impl Iterator for IndexIter  {
    type Item = [usize; 2];
    fn next(&mut self) -> Option<Self::Item> {
        if self.position[1] >= self.shape[1]
            {return None}
        let position = self.position.clone();
        self.position[0] += 1;
        if self.position[0] >= self.shape[0] {
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
    pub fn new(shape: [usize; 2]) -> Self  {Self{
        shape,
        position: [0; 2],
    }}
}
impl<A:Array> Matrix<A> {
    pub fn index(&self) -> IndexIter {IndexIter::new(self.shape())}
}


/// immutable column-major iterator into a matrix with elements indices
pub struct IndexedIter<'t, A: Array> {
    matrix: &'t Matrix<A>,
    index: IndexIter,
}
impl<'t,A:Array> 
    Iterator for IndexedIter<'t,A> 
{
    type Item = ([usize; 2], &'t A::Element);
    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index.next()?;
        Some((index, & self.matrix[index]))
    }
}
impl<A:Array>
    ExactSizeIterator for IndexedIter<'_,A>
{
    fn len(&self) -> usize  {self.index.len()}
}
impl<'t,A:Array> IndexedIter<'t,A> {
    pub fn values(self) -> MatrixIter<'t,A>  {MatrixIter(self)}
}


/// immutable column-major iterator into a matrix
pub struct MatrixIter<'t, A: Array>  (IndexedIter<'t, A>);
impl<'t,A:Array> 
    Iterator for MatrixIter<'t,A> 
{
    type Item = &'t A::Element;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|item|  item.1)
    }
}
impl<A:Array>
    ExactSizeIterator for MatrixIter<'_,A>
{
    fn len(&self) -> usize  {self.0.len()}
}
impl<'t,A:Array> 
    IntoIterator for &'t Matrix<A> 
{
    type Item = &'t A::Element;
    type IntoIter = MatrixIter<'t,A>;
    fn into_iter(self) -> Self::IntoIter {
        MatrixIter(IndexedIter {
            index: self.index(),
            matrix: self,
        })
    }
}
impl<'t,A:Array> MatrixIter<'t,A> {
    pub fn index(self) -> IndexedIter<'t,A>  {self.0}
}
impl<A:Array>  Matrix<A> {
    pub fn iter(&self) -> MatrixIter<'_,A>  {self.into_iter()}
}



/// mutable column-major iterator into a matrix with elements indices
pub struct IndexedIterMut<'t, A: ArrayMut> {
    matrix: &'t mut Matrix<A>,
    index: IndexIter,
}
impl<'t,A:ArrayMut> 
    Iterator for IndexedIterMut<'t,A> 
{
    type Item = ([usize; 2], &'t mut A::Element);
    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index.next()?;
        Some((index, unsafe {
            // the mutable refernce may outive the iterator but not the matrix reference
            // safety: elements are indexed once only, so one only mutable reference to each location can be created for the lifetime of the matrix reference
            core::mem::transmute::<&mut A::Element, &mut A::Element>(&mut self.matrix[index])
            }))
    }
}
impl<A:ArrayMut>
    ExactSizeIterator for IndexedIterMut<'_,A>
{
    fn len(&self) -> usize  {self.index.len()}
}
impl<'t,A:ArrayMut> IndexedIterMut<'t,A> {
    pub fn values(self) -> MatrixIterMut<'t,A>  {MatrixIterMut(self)}
}


/// mutable column-major iterator into a matrix
pub struct MatrixIterMut<'t, A: ArrayMut> (IndexedIterMut<'t, A>);
impl<'t,A:ArrayMut> 
    Iterator for MatrixIterMut<'t,A> 
{
    type Item = &'t mut A::Element;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|item|  item.1)
    }
}
impl<A:ArrayMut>
    ExactSizeIterator for MatrixIterMut<'_,A>
{
    fn len(&self) -> usize  {self.0.len()}
}
impl<'t,A:ArrayMut> 
    IntoIterator for &'t mut Matrix<A> 
{
    type Item = &'t mut A::Element;
    type IntoIter = MatrixIterMut<'t,A>;
    fn into_iter(self) -> Self::IntoIter {
        MatrixIterMut(IndexedIterMut {
            index: self.index(),
            matrix: self,
        })
    }
}
impl<'t,A:ArrayMut> MatrixIterMut<'t,A> {
    pub fn index(self) -> IndexedIterMut<'t,A>  {self.0}
}
impl<A:ArrayMut>  Matrix<A> {
    pub fn iter_mut(&mut self) -> MatrixIterMut<'_,A>  {self.into_iter()}
}




#[test]
fn test_iterators() {
    use crate::matrices::*;
    
    let mut m = SMatrix::from([[1,2,3,4], [5,6,7,8], [9,10,11,12]]);
    assert_eq!(m.iter().cloned().collect::<Vec<_>>(), [1,2,3,4, 5,6,7,8, 9,10,11,12]);
    for (i,&v) in m.iter().index() {
        assert_eq!(m[i], v);
    }
    for (_,v) in m.iter_mut().index() {
        *v = 2;
    }
    assert!(m.iter().all(|&v| v == 2));
}
#[test]
fn test_constructors() {
    use crate::matrices::*;
    use core::iter::zip;
    
    let a = SMatrix::from([[1,2,3,4], [5,6,7,8], [9,10,11,12]]);
    assert!(a.diagonal().is_none());
    let mut b = SMatrix::<u32,4,3>::zeros();
    b.set_iter(a.iter().cloned());
    for (&a,&b) in zip(&b, &a) {
        assert_eq!(a, b);
    }
    let mut b = SMatrix::<u32,4,4>::zeros();
    b.set_iter(a.iter().cloned());
    for (i,&a) in a.iter().index() {
        assert_eq!(b[i], a);
    }
    for (i,v) in b.diagonal_mut().unwrap().iter_mut().enumerate() {
        *v = i as _;
    }
    for i in 0 .. 3 {
        assert_eq!(b[[i,i]], i as _);
    }
}
