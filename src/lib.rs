// #![warn(missing_docs)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::let_underscore_future)]

#[macro_use] extern crate thiserror;

pub mod def;
pub mod map;
pub mod set;

pub mod item;
pub mod iter;
pub mod node;
pub mod pack;
pub mod rule;

pub use def::{RadixError, RadixResult};
pub use map::{RadixMap};
pub use set::{RadixSet};