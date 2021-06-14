//! Docker
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

// declare modules
pub mod container;
mod docker;
pub mod filesystem;
pub mod image;
pub mod network;
pub mod process;
pub mod stats;
pub mod system;
mod test;
pub mod version;

// publicly re-export
pub use docker::Docker;
