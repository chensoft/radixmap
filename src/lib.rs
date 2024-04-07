//! radixmap
//!
//! This crate is a rust-based radix tree implementation. Radix tree, also known as Trie, is a
//! space-optimized tree data structure for efficient information retrieval. Its key advantages
//! are space optimization, fast prefix-based searches, and efficient memory usage. Radix trees
//! are widely used, especially in HTTP routers.
//!
//! ## Features
//!
//! - Fast prefix-based lookup
//! - RadixMap and RadixSet support
//! - Standard collection-compatible interfaces  
//! - Named params, regex and glob support
//! - Pre-order, post-order, level-order iterations
//! - Comprehensive unit tests for correctness
//!
//! ## Example
//!
//! ```
//! use radixmap::{RadixMap, RadixResult};
//!
//! fn main() -> RadixResult<()> {
//!     // creation
//!     let mut map = RadixMap::new();
//!     map.insert("/api", "api")?;
//!     map.insert("/api/v1", "v1")?;
//!     map.insert("/api/v1/user", "user1")?;
//!     map.insert("/api/v2", "v2")?;
//!     map.insert("/api/v2/user", "user2")?;
//!
//!     // searching
//!     assert_eq!(map.get("/api/v1/user"), Some(&"user1"));
//!     assert_eq!(map.get("/api/v2/user"), Some(&"user2"));
//!
//!     // iteration
//!     let mut iter = map.iter(); // pre-order by default
//!
//!     assert_eq!(iter.next(), Some(("/api", &"api")));
//!     assert_eq!(iter.next(), Some(("/api/v1", &"v1")));
//!     assert_eq!(iter.next(), Some(("/api/v1/user", &"user1")));
//!     assert_eq!(iter.next(), Some(("/api/v2", &"v2")));
//!     assert_eq!(iter.next(), Some(("/api/v2/user", &"user2")));
//!     assert_eq!(iter.next(), None);
//!
//!     Ok(())
//! }
//! ```
#![warn(missing_docs)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::let_underscore_future)]

#[macro_use] extern crate thiserror;

pub mod map;
pub mod set;

pub mod defs;
pub mod node;
pub mod pack;
pub mod rule;

pub use map::{RadixMap};
pub use set::{RadixSet};
pub use defs::{RadixError, RadixResult};