mod base;
pub mod done;
pub mod pending;

pub mod traits; //this enables our "super" keyword to access the traits.

/*
This enables the files within the module to access
the base file. However, because we only want our base
struct to be used within the module, we do not make
it public
*/