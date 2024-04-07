//! Common defines
pub(crate) use std::hash::Hash;
pub(crate) use std::hash::Hasher;
pub(crate) use std::fmt::Debug;
pub(crate) use std::fmt::Formatter;
pub(crate) use std::borrow::Cow;
pub(crate) use std::cmp::Ordering;
pub(crate) use std::iter::Peekable;
pub(crate) use std::collections::VecDeque;

pub(crate) use regex::Regex;
pub(crate) use indexmap::IndexMap;
pub(crate) use sparseset::SparseSet;

/// Error Codes
#[allow(missing_docs)]
#[derive(Debug, Error)]
pub enum RadixError {
    #[error("path is empty")]
    PathEmpty,

    #[error("path not found")]
    PathNotFound,

    #[error("{0}")]
    PathMalformed(Cow<'static, str>),

    #[error("rule can't be split")]
    RuleIndivisible,

    #[error("{0}")]
    RegexInvalid(#[from] regex::Error),

    #[error("{0}")]
    GlobInvalid(#[from] glob::PatternError),
}

/// Custom Result
pub type RadixResult<T> = Result<T, RadixError>;

/// Macros to create RadixMap or RadixSet
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