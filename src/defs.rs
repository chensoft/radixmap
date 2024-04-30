//! Common defines
pub(crate) use std::hash::Hash;
pub(crate) use std::hash::Hasher;
pub(crate) use std::fmt::Debug;
pub(crate) use std::fmt::Formatter;
pub(crate) use std::cmp::Ordering;
pub(crate) use std::iter::Peekable;
pub(crate) use std::str::Utf8Error;
pub(crate) use std::collections::VecDeque;

pub(crate) use bytes::Bytes;
pub(crate) use regex::Regex;
pub(crate) use vec_map::VecMap;
pub(crate) use thiserror::Error;
pub(crate) use indexmap::IndexMap;

/// Error Codes
#[allow(missing_docs)]
#[derive(Debug, Error)]
pub enum RadixError {
    #[error("path is empty")]
    PathEmpty,

    #[error("path not found")]
    PathNotFound,

    #[error("{0}")]
    PathInvalid(#[from] Utf8Error),

    #[error("{0}")]
    PathMalformed(&'static str),

    #[error("rule can't be split")]
    RuleIndivisible,

    #[error("{0}")]
    GlobInvalid(#[from] glob::PatternError),

    #[error("{0}")]
    RegexInvalid(#[from] regex::Error),
}

/// Custom Result
pub type RadixResult<T> = Result<T, RadixError>;

/// Macros to create RadixMap or RadixSet
/// 
/// # Examples
///
/// ```
/// #[macro_use] extern crate radixmap;
///
/// let map = radix!{
///     "/" => "/",
///     "/api" => "api",
///     "/api/v1" => "v1",
///     "/api/v1/user" => "user1",
///     "/api/v2" => "v2",
///     "/api/v2/user" => "user2",
/// };
///
/// assert_eq!(map.get("/"), Some(&"/"));
/// assert_eq!(map.get("/api"), Some(&"api"));
/// assert_eq!(map.get("/api/v1"), Some(&"v1"));
/// assert_eq!(map.get("/api/v1/user"), Some(&"user1"));
/// assert_eq!(map.get("/api/v2"), Some(&"v2"));
/// assert_eq!(map.get("/api/v2/user"), Some(&"user2"));
/// assert_eq!(map.get("/api/v3"), None);
/// assert_eq!(map.get("/api/v3/user"), None);
///
/// let set = radix!{
///     "/",
///     "/api",
///     "/api/v1",
///     "/api/v1/user",
///     "/api/v2",
///     "/api/v2/user",
/// };
///
/// assert_eq!(set.contains("/"), true);
/// assert_eq!(set.contains("/api"), true);
/// assert_eq!(set.contains("/api/v1"), true);
/// assert_eq!(set.contains("/api/v1/user"), true);
/// assert_eq!(set.contains("/api/v2"), true);
/// assert_eq!(set.contains("/api/v2/user"), true);
/// assert_eq!(set.contains("/api/v3"), false);
/// assert_eq!(set.contains("/api/v3/user"), false);
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