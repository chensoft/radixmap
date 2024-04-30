//! Rule represents a match
use super::defs::*;

/// An enum representing various matching patterns
#[derive(Clone)]
pub enum RadixRule {
    /// Plain rule that accepts arbitrary strings
    ///
    /// # Syntax
    ///
    /// - /
    /// - /api
    ///
    Plain {
        /// fragment
        frag: Bytes
    },

    /// Named param matches a segment of the route
    ///
    /// # Syntax
    ///
    /// - :
    /// - :id
    ///
    Param {
        /// fragment
        frag: Bytes,

        /// param's name
        name: Bytes,
    },

    /// Unix glob style matcher, note that it must be the last component of a route
    ///
    /// # Syntax
    ///
    /// - *
    ///
    Glob {
        /// fragment
        frag: Bytes,

        /// glob pattern
        glob: glob::Pattern
    },

    /// Perl-like regular expressions
    ///
    /// # Syntax
    ///
    /// - {}
    /// - {:}
    /// - {\d+}
    /// - {:\d+}
    /// - {id:\d+}
    ///
    Regex {
        /// fragment
        frag: Bytes,

        /// regex's name
        name: Bytes,

        /// the regex
        expr: Regex,
    },
}

impl RadixRule {
    /// Create a plain text rule
    ///
    /// ```
    /// use radixmap::{rule::RadixRule};
    ///
    /// assert!(RadixRule::from_plain(r"").is_ok());
    /// assert!(RadixRule::from_plain(r"id").is_ok());
    /// ```
    #[inline]
    pub fn from_plain(frag: impl Into<Bytes>) -> RadixResult<Self> {
        Ok(Self::Plain { frag: frag.into() })
    }

    /// Create a named param rule
    ///
    /// ```
    /// use radixmap::{rule::RadixRule};
    ///
    /// assert!(RadixRule::from_param(r":").is_ok());   // segment placeholder
    /// assert!(RadixRule::from_param(r":id").is_ok()); // param with a name
    /// assert!(RadixRule::from_param(r"").is_err());   // missing :
    /// assert!(RadixRule::from_param(r"id").is_err()); // missing :
    /// ```
    #[inline]
    pub fn from_param(frag: impl Into<Bytes>) -> RadixResult<Self> {
        let frag = frag.into();

        if !frag.starts_with(b":") {
            return Err(RadixError::PathMalformed("param lack of colon"));
        }

        let name = frag.slice(1..);
        Ok(Self::Param { frag, name })
    }

    /// Create a unix glob style rule
    ///
    /// ```
    /// use radixmap::{rule::RadixRule};
    ///
    /// assert!(RadixRule::from_glob(r"*").is_ok());      // match entire string
    /// assert!(RadixRule::from_glob(r"*id").is_ok());    // match strings ending with 'id'
    /// assert!(RadixRule::from_glob(r"").is_err());      // missing rule chars
    /// assert!(RadixRule::from_glob(r"id").is_err());    // missing rule chars
    /// ```
    #[inline]
    pub fn from_glob(frag: impl Into<Bytes>) -> RadixResult<Self> {
        let frag = frag.into();

        if !frag.starts_with(b"*") {
            return Err(RadixError::PathMalformed("glob lack of asterisk"));
        }

        let glob = glob::Pattern::new(std::str::from_utf8(frag.as_ref())?)?;
        Ok(Self::Glob { frag, glob })
    }

    /// Create a regular expression rule
    ///
    /// ```
    /// use radixmap::{rule::RadixRule};
    ///
    /// assert!(RadixRule::from_regex(r"{}").is_ok());       // useless but valid
    /// assert!(RadixRule::from_regex(r"{:}").is_ok());      // same as above
    /// assert!(RadixRule::from_regex(r"{\d+}").is_ok());    // name is empty
    /// assert!(RadixRule::from_regex(r"{:\d+}").is_ok());   // same as above
    /// assert!(RadixRule::from_regex(r"{id:\d+}").is_ok()); // regex with a name
    /// assert!(RadixRule::from_regex(r"").is_err());        // missing {}
    /// assert!(RadixRule::from_regex(r"\d+").is_err());     // missing {}
    /// assert!(RadixRule::from_regex(r"{").is_err());       // missing }
    /// assert!(RadixRule::from_regex(r"{[0-9}").is_err());  // missing ]
    /// assert!(RadixRule::from_regex(r"{:(0}").is_err());   // missing )
    /// assert!(RadixRule::from_regex(r"{id:(0}").is_err()); // missing )
    /// ```
    #[inline]
    pub fn from_regex(frag: impl Into<Bytes>) -> RadixResult<Self> {
        let frag = frag.into();

        if !frag.starts_with(b"{") || !frag.ends_with(b"}") {
            return Err(RadixError::PathMalformed("regex lack of curly braces"));
        }

        let data = frag.slice(1..frag.len() - 1);
        let find = match memchr::memchr(b':', data.as_ref()) {
            Some(pos) => (data.slice(..pos), data.slice(pos + 1..)),
            None => (Bytes::new(), data)
        };

        // regex must match from the beginning, add ^ if needed
        match find.1.first() {
            Some(b'^') => Ok(Self::Regex { frag: frag.into(), name: find.0, expr: Regex::new(std::str::from_utf8(find.1.as_ref())?)? }),
            _ => Ok(Self::Regex { frag: frag.into(), name: find.0, expr: Regex::new(('^'.to_string() + std::str::from_utf8(find.1.as_ref())?).as_str())? })
        }
    }

    /// Match the path to find the longest shared segment
    ///
    /// ```
    /// use std::cmp::Ordering;
    /// use radixmap::{rule::RadixRule, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     assert_eq!(RadixRule::from_plain(r"")?.longest(""), (r"", Ordering::Equal));
    ///     assert_eq!(RadixRule::from_plain(r"")?.longest("api"), (r"", Ordering::Equal));
    ///     assert_eq!(RadixRule::from_plain(r"api")?.longest("api"), (r"api", Ordering::Equal));
    ///     assert_eq!(RadixRule::from_plain(r"api/v1")?.longest("api"), (r"api", Ordering::Greater));
    ///     assert_eq!(RadixRule::from_plain(r"api/v1")?.longest("api/v2"), (r"api/v", Ordering::Greater));
    ///     assert_eq!(RadixRule::from_plain(r"roadmap/issues/events/6430295168")?.longest("roadmap/issues/events/6635165802"), (r"roadmap/issues/events/6", Ordering::Greater));
    ///
    ///     assert_eq!(RadixRule::from_param(r":")?.longest("12345/rest"), (r"12345", Ordering::Equal));
    ///     assert_eq!(RadixRule::from_param(r":id")?.longest("12345/rest"), (r"12345", Ordering::Equal));
    ///
    ///     assert_eq!(RadixRule::from_glob(r"*")?.longest("12345/rest"), (r"12345/rest", Ordering::Equal));
    ///     assert_eq!(RadixRule::from_glob(r"*id")?.longest("12345/rest"), (r"", Ordering::Equal));
    ///
    ///     assert_eq!(RadixRule::from_regex(r"{}")?.longest("12345/rest"), (r"", Ordering::Equal));
    ///     assert_eq!(RadixRule::from_regex(r"{:}")?.longest("12345/rest"), (r"", Ordering::Equal));
    ///     assert_eq!(RadixRule::from_regex(r"{\d+}")?.longest("12345/rest"), (r"12345", Ordering::Equal));
    ///     assert_eq!(RadixRule::from_regex(r"{:\d+}")?.longest("12345/rest"), (r"12345", Ordering::Equal));
    ///     assert_eq!(RadixRule::from_regex(r"{id:\d+}")?.longest("12345/update"), (r"12345", Ordering::Equal));
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn longest<'u>(&self, path: &'u str) -> (&'u str, Ordering) {
        match self {
            RadixRule::Plain { frag } => {
                // accelerating string comparison using numbers
                let min = std::cmp::min(frag.len(), path.len());
                let mut len = 0;

                const BLK: usize = std::mem::size_of::<usize>();

                while len + BLK <= min {
                    let frag_chunk: &usize = unsafe { &*(frag.as_ptr().add(len) as *const usize) };
                    let path_chunk: &usize = unsafe { &*(path.as_ptr().add(len) as *const usize) };

                    match frag_chunk == path_chunk {
                        true => len += BLK,
                        false => break,
                    }
                }

                // process the leftover unmatched substring
                while len < min && frag[len] == path.as_bytes()[len] {
                    len += 1;
                }

                (&path[..len], frag.len().cmp(&len))
            }
            RadixRule::Param { .. } => match memchr::memchr(b'/', path.as_bytes()) {
                Some(p) => (&path[..p], Ordering::Equal),
                None => (path, Ordering::Equal)
            }
            RadixRule::Glob { glob, .. } => match glob.matches(path) {
                true => (path, Ordering::Equal),
                false => ("", Ordering::Equal)
            }
            RadixRule::Regex { expr, .. } => match expr.find(path) {
                Some(m) => (&path[..m.len()], Ordering::Equal),
                None => ("", Ordering::Equal)
            }
        }
    }

    /// Divide the rule into two parts
    ///
    /// ```
    /// use radixmap::{rule::RadixRule, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut rule = RadixRule::from_plain(r"/api")?;
    ///
    ///     assert_eq!(rule.divide(1)?, r"api");
    ///     assert_eq!(rule, r"/");
    ///
    ///     assert!(RadixRule::from_param(r":id")?.divide(1).is_err());
    ///     assert!(RadixRule::from_glob(r"*")?.divide(1).is_err());
    ///     assert!(RadixRule::from_regex(r"{id:\d+}")?.divide(1).is_err());
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn divide(&mut self, len: usize) -> RadixResult<RadixRule> {
        match self {
            RadixRule::Plain { frag } if frag.len() > len => {
                let rule = RadixRule::from_plain(frag.slice(len..));
                *frag = frag.slice(..len);
                rule
            }
            _ => Err(RadixError::RuleIndivisible)
        }
    }

    /// Origin fragment of the rule
    ///
    /// ```
    /// use radixmap::{rule::RadixRule, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     assert_eq!(RadixRule::from_plain(r"/api")?.origin(), r"/api");
    ///     assert_eq!(RadixRule::from_param(r":id")?.origin(), r":id");
    ///     assert_eq!(RadixRule::from_glob(r"*")?.origin(), r"*");
    ///     assert_eq!(RadixRule::from_regex(r"{id:\d+}")?.origin(), r"{id:\d+}");
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn origin(&self) -> &Bytes {
        match self {
            RadixRule::Plain { frag } => frag,
            RadixRule::Param { frag, .. } => frag,
            RadixRule::Glob { frag, .. } => frag,
            RadixRule::Regex { frag, .. } => frag,
        }
    }

    /// The name of the named param and regex
    ///
    /// ```
    /// use radixmap::{rule::RadixRule, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     assert_eq!(RadixRule::from_param(r":id")?.identity(), r"id");
    ///     assert_eq!(RadixRule::from_regex(r"{id:\d+}")?.identity(), r"id");
    ///
    ///     assert_eq!(RadixRule::from_plain(r"/api")?.identity(), r"");
    ///     assert_eq!(RadixRule::from_param(r":")?.identity(), r"");
    ///     assert_eq!(RadixRule::from_glob(r"*")?.identity(), r"");
    ///     assert_eq!(RadixRule::from_regex(r"{\d+}")?.identity(), r"");
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn identity(&self) -> &Bytes {
        static EMPTY: Bytes = Bytes::new();
        match self {
            RadixRule::Param { name, .. } => name,
            RadixRule::Regex { name, .. } => name,
            _ => &EMPTY,
        }
    }
}

/// Analyze a path as long as possible and construct a rule
///
/// ```
/// use radixmap::{rule::RadixRule, RadixResult};
///
/// fn main() -> RadixResult<()> {
///     assert!(RadixRule::try_from(r"").is_err());
///
///     assert_eq!(RadixRule::try_from(r"api")?, r"api");
///     assert_eq!(RadixRule::try_from(r"api/v1")?, r"api/v1");
///     assert_eq!(RadixRule::try_from(r"/api/v1")?, r"/api/v1");
///
///     assert_eq!(RadixRule::try_from(r":")?, r":");
///     assert_eq!(RadixRule::try_from(r":id")?, r":id");
///     assert_eq!(RadixRule::try_from(r":id/rest")?, r":id");
///
///     assert_eq!(RadixRule::try_from(r"{id:\d+}")?, r"{id:\d+}");
///     assert_eq!(RadixRule::try_from(r"{id:\d+}/rest")?, r"{id:\d+}");
///     assert!(RadixRule::try_from(r"{id:\d+").is_err());
///     assert!(RadixRule::try_from(r"{id:\d+/rest").is_err());
///
///     assert_eq!(RadixRule::try_from(r"*")?, r"*");
///     assert_eq!(RadixRule::try_from(r"*rest")?, r"*rest");
///     assert_eq!(RadixRule::try_from(r"*/rest")?, r"*/rest");
///
///     Ok(())
/// }
/// ```
impl TryFrom<Bytes> for RadixRule {
    type Error = RadixError;

    fn try_from(path: Bytes) -> Result<Self, Self::Error> {
        let init = path.first().ok_or(RadixError::PathEmpty)?;

        match *init {
            b':' => match memchr::memchr(b'/', path.as_ref()) {
                Some(pos) => Self::from_param(path.slice(..pos)),
                _ => Self::from_param(path),
            }
            b'*' => {
                Self::from_glob(path)
            }
            b'{' => match memchr::memchr(b'}', path.as_ref()) {
                Some(pos) => Self::from_regex(path.slice(..pos + 1)),
                _ => Err(RadixError::PathMalformed("missing closing sign '}'"))
            }
            _ => match memchr::memchr3(b'{', b':', b'*', path.as_ref()) {
                Some(pos) => Self::from_plain(path.slice(..pos)),
                None => Self::from_plain(path),
            }
        }
    }
}

/// Analyze a path as long as possible and construct a rule
impl TryFrom<&'static str> for RadixRule {
    type Error = RadixError;

    fn try_from(path: &'static str) -> Result<Self, Self::Error> {
        Bytes::from(path).try_into()
    }
}

/// Default trait
///
/// ```
/// use radixmap::{rule::RadixRule};
///
/// assert_eq!(RadixRule::default(), "");
/// ```
impl Default for RadixRule {
    #[inline]
    fn default() -> Self {
        Self::Plain { frag: Bytes::new() }
    }
}

/// Debug trait
///
/// ```
/// use radixmap::{rule::RadixRule, RadixResult};
///
/// fn main() -> RadixResult<()> {
///     assert_eq!(format!("{:?}", RadixRule::from_plain(r"/api")?).as_str(), r"Plain(/api)");
///     assert_eq!(format!("{:?}", RadixRule::from_param(r":id")?).as_str(), r"Param(:id)");
///     assert_eq!(format!("{:?}", RadixRule::from_glob(r"*")?).as_str(), r"Glob(*)");
///     assert_eq!(format!("{:?}", RadixRule::from_regex(r"{id:\d+}")?).as_str(), r"Regex({id:\d+})");
///
///     Ok(())
/// }
/// ```
impl Debug for RadixRule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RadixRule::Plain { frag } => write!(f, "Plain({})", unsafe { std::str::from_utf8_unchecked(frag.as_ref()) }),
            RadixRule::Param { frag, .. } => write!(f, "Param({})", unsafe { std::str::from_utf8_unchecked(frag.as_ref()) }),
            RadixRule::Glob { frag, .. } => write!(f, "Glob({})", unsafe { std::str::from_utf8_unchecked(frag.as_ref()) }),
            RadixRule::Regex { frag, .. } => write!(f, "Regex({})", unsafe { std::str::from_utf8_unchecked(frag.as_ref()) }),
        }
    }
}

/// Hash trait
///
/// ```
/// use std::collections::HashMap;
/// use radixmap::{rule::RadixRule, RadixResult};
///
/// fn main() -> RadixResult<()> {
///     let mut map = HashMap::new();
///     map.insert(RadixRule::from_plain(r"/api")?, r"/api");
///     map.insert(RadixRule::from_param(r":id")?, r":id");
///     map.insert(RadixRule::from_glob(r"*")?, r"*");
///     map.insert(RadixRule::from_regex(r"{id:\d+}")?, r"{id:\d+}");
///
///     assert_eq!(map[&RadixRule::from_plain(r"/api")?], r"/api");
///     assert_eq!(map[&RadixRule::from_param(r":id")?], r":id");
///     assert_eq!(map[&RadixRule::from_glob(r"*")?], r"*");
///     assert_eq!(map[&RadixRule::from_regex(r"{id:\d+}")?], r"{id:\d+}");
///
///     Ok(())
/// }
/// ```
impl Hash for RadixRule {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            RadixRule::Plain { frag } => {
                "Plain".hash(state);
                frag.hash(state);
            }
            RadixRule::Param { frag, .. } => {
                "Param".hash(state);
                frag.hash(state);
            }
            RadixRule::Glob { frag, .. } => {
                "Glob".hash(state);
                frag.hash(state);
            }
            RadixRule::Regex { frag, .. } => {
                "Regex".hash(state);
                frag.hash(state);
            }
        }
    }
}

/// == & !=
impl Eq for RadixRule {}

/// == & !=
///
/// ```
/// use radixmap::{rule::RadixRule, RadixResult};
///
/// fn main() -> RadixResult<()> {
///     assert_eq!(RadixRule::from_plain(r"/api")?, RadixRule::from_plain(r"/api")?);
///     assert_eq!(RadixRule::from_param(r":id")?, RadixRule::from_param(r":id")?);
///     assert_eq!(RadixRule::from_glob(r"*")?, RadixRule::from_glob(r"*")?);
///     assert_eq!(RadixRule::from_regex(r"{id:\d+}")?, RadixRule::from_regex(r"{id:\d+}")?);
///
///     assert_ne!(RadixRule::from_plain(r"/api")?, RadixRule::from_plain(r"")?);
///     assert_ne!(RadixRule::from_param(r":id")?, RadixRule::from_param(r":")?);
///     assert_ne!(RadixRule::from_glob(r"*")?, RadixRule::from_glob(r"**")?);
///     assert_ne!(RadixRule::from_regex(r"{id:\d+}")?, RadixRule::from_regex(r"{}")?);
///
///     // type mismatch
///     assert_ne!(RadixRule::from_plain(r"{}")?, RadixRule::from_regex(r"{}")?);
///
///     Ok(())
/// }
/// ```
impl PartialEq for RadixRule {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RadixRule::Plain { frag: a }, RadixRule::Plain { frag: b }) => a == b,
            (RadixRule::Param { frag: a, .. }, RadixRule::Param { frag: b, .. }) => a == b,
            (RadixRule::Glob { frag: a, .. }, RadixRule::Glob { frag: b, .. }) => a == b,
            (RadixRule::Regex { frag: a, .. }, RadixRule::Regex { frag: b, .. }) => a == b,
            _ => false
        }
    }
}

/// == & !=
///
/// ```
/// use radixmap::{rule::RadixRule, RadixResult};
///
/// fn main() -> RadixResult<()> {
///     assert_eq!(RadixRule::from_plain(r"/api")?, r"/api");
///     assert_eq!(RadixRule::from_param(r":id")?, r":id");
///     assert_eq!(RadixRule::from_glob(r"*")?, r"*");
///     assert_eq!(RadixRule::from_regex(r"{id:\d+}")?, r"{id:\d+}");
///
///     assert_ne!(RadixRule::from_plain(r"/api")?, r"");
///     assert_ne!(RadixRule::from_param(r":id")?, r":");
///     assert_ne!(RadixRule::from_glob(r"*")?, r"**");
///     assert_ne!(RadixRule::from_regex(r"{id:\d+}")?, r"{}");
///
///     Ok(())
/// }
/// ```
impl PartialEq<&str> for RadixRule {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        self.origin() == *other
    }
}