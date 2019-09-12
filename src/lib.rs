//! Docker
#![doc(html_root_url="https://ghmlee.github.io/rust-docker/doc")]
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

// import external libraries

extern crate hyper;
extern crate hyperlocal;
extern crate futures;
extern crate tokio_core;

// declare modules
mod test;
mod docker;
pub mod container;
pub mod network;
pub mod stats;
pub mod system;
pub mod image;
pub mod process;
pub mod filesystem;
pub mod version;

// publicly re-export
pub use docker::Docker;
