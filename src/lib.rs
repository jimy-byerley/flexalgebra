/*!
	*Flexible Linear algebra library working in arbitrary dimension*

	This crate propose an alternative strongly inspired from [nalgebra](https://docs.rs/nalgebra/latest/nalgebra/index.html) that aims to reduce the boilerplate its mentor has for working with generics, while keeping its flexiblity.
	The result is as much simpler and comprehensive library with a shorter codebase

	It mainly features the **[Matrix]** struct, which represent a *dense* matrix with any *strided* memory. And providing common matrix operations *as its methods*.
	
	Of course this crate address both the case of small-dimension objects better used as static-allocated objects and the case of big-dimension arrays usually handled with dynamic allocation

	## Goals
	
	- `no-std` feature, but not started yet
	- as feature-complete as [nalgebra](https://docs.rs/nalgebra/latest/nalgebra/index.html) and [numpy](https://numpy.org/doc/stable/reference/routines.linalg.html)
	- as fast as size-specific libraries
	
	## Comparison
		
	At contrary to *nalgebra*, `Matrix` is not a one-size-fit-all struct with tons of generic arguments. It is a wrapper type that can wrap any array storage type (even not from *flexalgebra*). The type and shape parameters only depend on the array storage which can be user provided.

	This means any user-type can be used as a matrix compatible with the rest of the *flexalgebra* types
*/

pub mod prelude;
pub mod matrix;
pub mod operators;
pub mod matrices;
pub mod glm;
pub mod iterator;

pub use prelude::*;
pub use matrix::*;
pub use matrices::*;
