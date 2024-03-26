/// Internal Use
pub(crate) use std::ops::Index;
pub(crate) use std::ops::IndexMut;
pub(crate) use std::fmt::Debug;
pub(crate) use std::fmt::Formatter;
pub(crate) use std::borrow::Cow;
pub(crate) use std::cmp::Ordering;

pub(crate) use thiserror::Error;
pub(crate) use sparseset::SparseSet;
pub(crate) type Result<T> = anyhow::Result<T>;

/// Error Codes
#[derive(Debug, Clone, Error, Eq, PartialEq, Ord, PartialOrd)]
pub enum Error {
    #[error("path is empty")]
    PathEmpty,

    #[error("{0}")]
    PathMalformed(Cow<'static, str>),
}

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