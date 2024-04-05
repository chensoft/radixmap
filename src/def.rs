/// Internal Use
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
/// assert_eq!(map.get("/"), Some(&"/"));
/// assert_eq!(map.get("/api"), Some(&"/api"));
/// assert_eq!(map.get("/api/v1"), Some(&"/api/v1"));
/// assert_eq!(map.get("/api/v1/user"), Some(&"/api/v1/user"));
/// assert_eq!(map.get("/api/v2"), Some(&"/api/v2"));
/// assert_eq!(map.get("/api/v2/user"), Some(&"/api/v2/user"));
/// assert_eq!(map.get("/api/v2/user/12345"), Some(&"/api/v2/user/12345"));
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