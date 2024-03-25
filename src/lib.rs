/// Internal Use
pub(crate) use thiserror::Error;
pub(crate) use std::cmp::Ordering;
pub(crate) use sparseset::SparseSet;
pub(crate) type Result<T> = anyhow::Result<T>;

/// Error Codes
#[derive(Debug, Clone, Error, Eq, PartialEq, Ord, PartialOrd)]
pub enum Error {
    #[error("{0}")]
    Empty(i32)
}

/// Export Types
mod item;
mod node;
mod tree;

pub use item::*;
pub use node::*;
pub use tree::*;

/// Handy Macro
#[macro_export]
macro_rules! preway {
    () => {{
        todo!()
    }};

    ($($key:expr),+) => {{
        $crate::preway!($($key => ()),+)
    }};

    ($($key:expr => $val:expr),+) => {{
        todo!()
    }};
}