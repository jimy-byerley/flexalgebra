/**
	## Architecture
	
	- copy-based operations (like math operations) are implemented in the common matrix structure
	- storage-based operations (like slicing and referencing) are implemented in specializations
*/

pub mod prelude;
pub mod matrix;
pub mod operators;
pub mod matrices;
pub mod glm;

pub use matrix::*;
pub use matrices::*;
