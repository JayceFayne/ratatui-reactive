#![doc = include_str!("../README.md")]

mod backend;
mod core;
#[cfg(any(feature = "tokio", feature = "async-io"))]
mod delay;
mod futures;
#[cfg(any(feature = "tokio", feature = "async-io"))]
mod router;
mod runtime;
#[cfg(any(feature = "tokio", feature = "async-io"))]
#[cfg(test)]
mod tests;
#[cfg(any(feature = "tokio", feature = "async-io"))]
mod timer;

#[cfg(any(feature = "tokio", feature = "async-io"))]
pub use backend::*;
pub use core::*;
#[cfg(any(feature = "tokio", feature = "async-io"))]
pub use futures::*;
#[cfg(any(feature = "tokio", feature = "async-io"))]
pub use router::{Route, Router, provide_router};
pub use sycamore_reactive::*;
#[cfg(any(feature = "tokio", feature = "async-io"))]
pub use timer::*;
