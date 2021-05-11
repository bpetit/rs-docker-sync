//! Docker
#![doc(html_root_url = "https://ghmlee.github.io/rust-docker/doc")]
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

// import external libraries

extern crate futures;
extern crate hyper;
extern crate hyperlocal;
extern crate tokio_core;

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
