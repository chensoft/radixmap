#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::let_underscore_future)]

pub mod map;
pub mod set;

pub mod defs;
pub mod node;
pub mod pack;
pub mod rule;

pub use map::{RadixMap};
pub use set::{RadixSet};
pub use defs::{RadixError, RadixResult};