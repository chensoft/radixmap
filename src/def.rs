/// Internal Use
// pub(crate) use std::ops::Index;
// pub(crate) use std::ops::IndexMut;
pub(crate) use std::hash::Hash;
pub(crate) use std::hash::Hasher;
pub(crate) use std::fmt::Debug;
pub(crate) use std::fmt::Formatter;
pub(crate) use std::borrow::Cow;
pub(crate) use std::cmp::Ordering;
pub(crate) use std::collections::VecDeque;

pub(crate) use regex::Regex;
pub(crate) use thiserror::Error;
pub(crate) use indexmap::IndexMap;
pub(crate) use sparseset::SparseSet;
pub(crate) type Result<T> = anyhow::Result<T>;

/// Error Codes
#[derive(Debug, Clone, Error, Eq, PartialEq, Ord, PartialOrd)]
pub enum Error {
    #[error("path is empty")]
    PathEmpty,

    #[error("{0}")]
    PathMalformed(Cow<'static, str>),

    #[error("item can't be split")]
    ItemIndivisible
}

/// Create RadixMap or RadixSet
///
/// ```
/// #[macro_use] extern crate radixmap;
///
/// let map = radix!{
///     "/" => "/",
///     "/api" => "/api",
///     "/api/v1" => "/api/v1",
///     "/api/v1/user" => "/api/v1/user",
///     "/api/v2" => "/api/v2",
///     "/api/v2/user" => "/api/v2/user",
///     "/api/v2/user/12345" => "/api/v2/user/12345"
/// };
///
/// let set = radix!{
///     "/",
///     "/api",
///     "/api/v1",
///     "/api/v1/user",
///     "/api/v2",
///     "/api/v2/user",
///     "/api/v2/user/12345",
/// };
///
/// // todo assert
/// ```
#[macro_export]
macro_rules! radix {
    ($($path:expr => $data:expr),+ $(,)?) => {{
        let mut map = $crate::RadixMap::default();
        $(map.insert($path, $data);)+
        map
    }};

    ($($path:expr),+ $(,)?) => {{
        let mut set = $crate::RadixSet::default();
        $(set.insert($path);)+
        set
    }};
}