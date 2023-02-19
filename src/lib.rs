/**
	## Architecture
	
	- copy-based operations (like math operations) are implemented in the common matrix structure
	- storage-based operations (like slicing and referencing) are implemented in specializations
*/

pub mod prelude;
pub mod matrix;
pub mod vector;
pub mod matrices;
pub mod vectors;

pub mod idea;
