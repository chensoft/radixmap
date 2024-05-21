//! Rule represents a match
use super::defs::*;
use std::str::from_utf8;
use std::str::from_utf8_unchecked;

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
    /// # Examples
    ///
    /// ```
    /// use radixmap::{rule::RadixRule};
    ///
    /// assert!(RadixRule::from_plain("").is_ok());
    /// assert!(RadixRule::from_plain("id").is_ok());
    /// ```
    #[inline]
    pub fn from_plain(frag: impl Into<Bytes>) -> RadixResult<Self> {
        Ok(Self::Plain { frag: frag.into() })
    }

    /// Create a named param rule
    ///
    /// # Examples
    ///
    /// ```
    /// use radixmap::{rule::RadixRule};
    ///
    /// assert!(RadixRule::from_param(":").is_ok());   // segment placeholder
    /// assert!(RadixRule::from_param(":id").is_ok()); // param with a name
    /// assert!(RadixRule::from_param("").is_err());   // missing :
    /// assert!(RadixRule::from_param("id").is_err()); // missing :
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
    /// # Examples
    ///
    /// ```
    /// use radixmap::{rule::RadixRule};
    ///
    /// assert!(RadixRule::from_glob("*").is_ok());      // match entire string
    /// assert!(RadixRule::from_glob("*id").is_ok());    // match strings ending with 'id'
    /// assert!(RadixRule::from_glob("").is_err());      // missing rule chars
    /// assert!(RadixRule::from_glob("id").is_err());    // missing rule chars
    /// ```
    #[inline]
    pub fn from_glob(frag: impl Into<Bytes>) -> RadixResult<Self> {
        let frag = frag.into();

        if !frag.starts_with(b"*") {
            return Err(RadixError::PathMalformed("glob lack of asterisk"));
        }

        let glob = glob::Pattern::new(from_utf8(frag.as_ref())?)?;
        Ok(Self::Glob { frag, glob })
    }

    /// Create a regular expression rule
    ///
    /// # Examples
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
    ///
    /// #[allow(invalid_from_utf8_unchecked)]
    /// let invalid = format!("{{{}}}", unsafe { std::str::from_utf8_unchecked(&[0xffu8, 0xfe, 0x65]) });
    /// assert!(RadixRule::from_regex(invalid).is_err());
    /// ```
    #[inline]
    pub fn from_regex(frag: impl Into<Bytes>) -> RadixResult<Self> {
        let frag = frag.into();

        if !frag.starts_with(b"{") || !frag.ends_with(b"}") {
            return Err(RadixError::PathMalformed("regex lack of curly braces"));
        }

        let data = frag.slice(1..frag.len() - 1);
        let find = match memchr::memchr(b':', data.as_ref()) {
            Some(pos) => (data.slice(..pos), from_utf8(&data[pos + 1..])?),
            None => (Bytes::new(), from_utf8(data.as_ref())?)
        };

        // regex must match from the beginning, add ^ if needed
        let (name, expr) = match find.1.as_bytes().first() {
            Some(b'^') => (find.0, Regex::new(find.1)?),
            _ => (find.0, Regex::new(('^'.to_string() + find.1).as_str())?)
        };

        Ok(Self::Regex { frag, name, expr })
    }

    /// Check if the rule is plain text
    ///
    /// # Examples
    ///
    /// ```
    /// use radixmap::{rule::RadixRule, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     assert_eq!(RadixRule::from_plain("")?.is_plain(), true);
    ///     assert_eq!(RadixRule::from_param(":id")?.is_plain(), false);
    ///     assert_eq!(RadixRule::from_glob("*")?.is_plain(), false);
    ///     assert_eq!(RadixRule::from_regex(r"{id:\d+}")?.is_plain(), false);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn is_plain(&self) -> bool {
        matches!(self, RadixRule::Plain { .. })
    }

    /// Check if the rule is special
    ///
    /// # Examples
    ///
    /// ```
    /// use radixmap::{rule::RadixRule, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     assert_eq!(RadixRule::from_plain("")?.is_special(), false);
    ///     assert_eq!(RadixRule::from_param(":id")?.is_special(), true);
    ///     assert_eq!(RadixRule::from_glob("*")?.is_special(), true);
    ///     assert_eq!(RadixRule::from_regex(r"{id:\d+}")?.is_special(), true);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn is_special(&self) -> bool {
        !self.is_plain()
    }

    /// Match the path to find the longest shared segment
    ///
    /// # Examples
    ///
    /// ```
    /// use radixmap::{rule::RadixRule, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     assert_eq!(RadixRule::from_plain("")?.longest(b"", false), Some("".as_bytes()));
    ///     assert_eq!(RadixRule::from_plain("")?.longest(b"api", false), Some("".as_bytes()));
    ///     assert_eq!(RadixRule::from_plain("api")?.longest(b"api", false), Some("api".as_bytes()));
    ///     assert_eq!(RadixRule::from_plain("api/v1")?.longest(b"api", false), Some("api".as_bytes()));
    ///     assert_eq!(RadixRule::from_plain("api/v1")?.longest(b"api/v2", false), Some("api/v".as_bytes()));
    ///     assert_eq!(RadixRule::from_plain("roadmap/issues/events/6430295168")?.longest(b"roadmap/issues/events/6635165802", false), Some("roadmap/issues/events/6".as_bytes()));
    ///
    ///     assert_eq!(RadixRule::from_param(":")?.longest(b"12345/rest", false), Some("12345".as_bytes()));
    ///     assert_eq!(RadixRule::from_param(":id")?.longest(b"12345/rest", false), Some("12345".as_bytes()));
    ///     assert_eq!(RadixRule::from_param(":id")?.longest(b"12345/rest", true), Some("".as_bytes()));
    ///     assert_eq!(RadixRule::from_param(":id")?.longest(b":id", true), Some(":id".as_bytes()));
    ///
    ///     assert_eq!(RadixRule::from_glob("*")?.longest(b"12345/rest", false), Some("12345/rest".as_bytes()));
    ///     assert_eq!(RadixRule::from_glob("*id")?.longest(b"12345/rest", false), None);
    ///     assert_eq!(RadixRule::from_glob("*id")?.longest(b"12345/rest", true), Some("".as_bytes()));
    ///     assert_eq!(RadixRule::from_glob("*id")?.longest(b"*id", true), Some("*id".as_bytes()));
    ///
    ///     assert_eq!(RadixRule::from_regex(r"{}")?.longest(b"12345/rest", false), Some(r"".as_bytes()));
    ///     assert_eq!(RadixRule::from_regex(r"{:}")?.longest(b"12345/rest", false), Some(r"".as_bytes()));
    ///     assert_eq!(RadixRule::from_regex(r"{\d+}")?.longest(b"12345/rest", false), Some(r"12345".as_bytes()));
    ///     assert_eq!(RadixRule::from_regex(r"{:\d+}")?.longest(b"12345/rest", false), Some(r"12345".as_bytes()));
    ///     assert_eq!(RadixRule::from_regex(r"{id:\d+}")?.longest(b"12345/update", false), Some(r"12345".as_bytes()));
    ///     assert_eq!(RadixRule::from_regex(r"{id:\d+}")?.longest(b"abcde", false), None);
    ///     assert_eq!(RadixRule::from_regex(r"{id:\d+}")?.longest(b"abcde", true), Some(r"".as_bytes()));
    ///     assert_eq!(RadixRule::from_regex(r"{id:\d+}")?.longest(br"{id:\d+}", true), Some(r"{id:\d+}".as_bytes()));
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn longest<'u>(&self, path: &'u [u8], raw: bool) -> Option<&'u [u8]> {
        if matches!(self, RadixRule::Plain { .. }) || raw {
            let frag = match self {
                RadixRule::Plain { frag, .. } => frag,
                RadixRule::Param { frag, .. } => frag,
                RadixRule::Glob { frag, .. } => frag,
                RadixRule::Regex { frag, .. } => frag,
            };

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
            while len < min && frag[len] == path[len] {
                len += 1;
            }

            return Some(&path[..len]);
        }

        match self {
            RadixRule::Plain { .. } => unreachable!(),
            RadixRule::Param { .. } => match memchr::memchr(b'/', path) {
                Some(p) => Some(&path[..p]),
                None => Some(path)
            }
            RadixRule::Glob { glob, .. } => {
                let utf8 = match from_utf8(path) {
                    Ok(p) => p,
                    Err(_) => return None,
                };

                match glob.matches(utf8) {
                    true => Some(path),
                    false => None
                }
            }
            RadixRule::Regex { expr, .. } => {
                let utf8 = match from_utf8(path) {
                    Ok(p) => p,
                    Err(_) => return None,
                };

                match expr.find(utf8) {
                    Some(m) => Some(&path[..m.len()]),
                    None => None
                }
            }
        }
    }

    /// Divide the rule into two parts
    ///
    /// # Examples
    ///
    /// ```
    /// use radixmap::{rule::RadixRule, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut rule = RadixRule::from_plain("/api")?;
    ///
    ///     assert_eq!(rule.divide(1)?, "api");
    ///     assert_eq!(rule, "/");
    ///
    ///     assert!(RadixRule::from_param(":id")?.divide(1).is_err());
    ///     assert!(RadixRule::from_glob("*")?.divide(1).is_err());
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
    /// # Examples
    ///
    /// ```
    /// use radixmap::{rule::RadixRule, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     assert_eq!(RadixRule::from_plain("/api")?.origin(), "/api");
    ///     assert_eq!(RadixRule::from_param(":id")?.origin(), ":id");
    ///     assert_eq!(RadixRule::from_glob("*")?.origin(), "*");
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
    /// # Examples
    ///
    /// ```
    /// use radixmap::{rule::RadixRule, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     assert_eq!(RadixRule::from_param(":id")?.identity(), "id");
    ///     assert_eq!(RadixRule::from_regex(r"{id:\d+}")?.identity(), r"id");
    ///
    ///     assert_eq!(RadixRule::from_plain("/api")?.identity(), "");
    ///     assert_eq!(RadixRule::from_param(":")?.identity(), "");
    ///     assert_eq!(RadixRule::from_glob("*")?.identity(), "*");
    ///     assert_eq!(RadixRule::from_regex(r"{\d+}")?.identity(), r"");
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn identity(&self) -> &Bytes {
        static EMPTY: Bytes = Bytes::new();
        static GLOB: Bytes = Bytes::from_static(b"*");

        match self {
            RadixRule::Plain { .. } => &EMPTY,
            RadixRule::Param { name, .. } => name,
            RadixRule::Glob { .. } => &GLOB,
            RadixRule::Regex { name, .. } => name,
        }
    }
}

/// Analyze a path as long as possible and construct a rule
///
/// # Examples
///
/// ```
/// use radixmap::{rule::RadixRule, RadixResult};
///
/// fn main() -> RadixResult<()> {
///     assert!(RadixRule::try_from("").is_err());
///
///     assert_eq!(RadixRule::try_from("api")?, "api");
///     assert_eq!(RadixRule::try_from("api/v1")?, "api/v1");
///     assert_eq!(RadixRule::try_from("/api/v1")?, "/api/v1");
///
///     assert_eq!(RadixRule::try_from(":")?, ":");
///     assert_eq!(RadixRule::try_from(":id")?, ":id");
///     assert_eq!(RadixRule::try_from(":id/rest")?, ":id");
///
///     assert_eq!(RadixRule::try_from("*")?, "*");
///     assert_eq!(RadixRule::try_from("*rest")?, "*rest");
///     assert_eq!(RadixRule::try_from("*/rest")?, "*/rest");
///
///     assert_eq!(RadixRule::try_from(r"{id:\d+}")?, r"{id:\d+}");
///     assert_eq!(RadixRule::try_from(r"{id:\d+}/rest")?, r"{id:\d+}");
///     assert!(RadixRule::try_from(r"{id:\d+").is_err());
///     assert!(RadixRule::try_from(r"{id:\d+/rest").is_err());
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
impl TryFrom<&'static [u8]> for RadixRule {
    type Error = RadixError;

    fn try_from(path: &'static [u8]) -> Result<Self, Self::Error> {
        Bytes::from(path).try_into()
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
/// # Examples
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
/// # Examples
///
/// ```
/// use radixmap::{rule::RadixRule, RadixResult};
///
/// fn main() -> RadixResult<()> {
///     assert_eq!(format!("{:?}", RadixRule::from_plain("/api")?).as_str(), "Plain(/api)");
///     assert_eq!(format!("{:?}", RadixRule::from_param(":id")?).as_str(), "Param(:id)");
///     assert_eq!(format!("{:?}", RadixRule::from_glob("*")?).as_str(), "Glob(*)");
///     assert_eq!(format!("{:?}", RadixRule::from_regex(r"{id:\d+}")?).as_str(), r"Regex({id:\d+})");
///
///     Ok(())
/// }
/// ```
impl Debug for RadixRule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (kind, frag) = match self {
            RadixRule::Plain { frag } => ("Plain", frag),
            RadixRule::Param { frag, .. } => ("Param", frag),
            RadixRule::Glob { frag, .. } => ("Glob", frag),
            RadixRule::Regex { frag, .. } => ("Regex", frag),
        };

        write!(f, "{}({})", kind, unsafe { from_utf8_unchecked(frag.as_ref()) })
    }
}

/// Hash trait
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use radixmap::{rule::RadixRule, RadixResult};
///
/// fn main() -> RadixResult<()> {
///     let mut map = HashMap::new();
///     map.insert(RadixRule::from_plain("/api")?, "/api");
///     map.insert(RadixRule::from_param(":id")?, ":id");
///     map.insert(RadixRule::from_glob("*")?, "*");
///     map.insert(RadixRule::from_regex(r"{id:\d+}")?, r"{id:\d+}");
///
///     assert_eq!(map[&RadixRule::from_plain("/api")?], "/api");
///     assert_eq!(map[&RadixRule::from_param(":id")?], ":id");
///     assert_eq!(map[&RadixRule::from_glob("*")?], "*");
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
/// # Examples
///
/// ```
/// use radixmap::{rule::RadixRule, RadixResult};
///
/// fn main() -> RadixResult<()> {
///     assert_eq!(RadixRule::from_plain("/api")?, RadixRule::from_plain("/api")?);
///     assert_eq!(RadixRule::from_param(":id")?, RadixRule::from_param(":id")?);
///     assert_eq!(RadixRule::from_glob("*")?, RadixRule::from_glob("*")?);
///     assert_eq!(RadixRule::from_regex(r"{id:\d+}")?, RadixRule::from_regex(r"{id:\d+}")?);
///
///     assert_ne!(RadixRule::from_plain("/api")?, RadixRule::from_plain("")?);
///     assert_ne!(RadixRule::from_param(":id")?, RadixRule::from_param(":")?);
///     assert_ne!(RadixRule::from_glob("*")?, RadixRule::from_glob("**")?);
///     assert_ne!(RadixRule::from_regex(r"{id:\d+}")?, RadixRule::from_regex(r"{}")?);
///
///     // type mismatch
///     assert_ne!(RadixRule::from_plain("{}")?, RadixRule::from_regex(r"{}")?);
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
/// # Examples
///
/// ```
/// use radixmap::{rule::RadixRule, RadixResult};
///
/// fn main() -> RadixResult<()> {
///     assert_eq!(RadixRule::from_plain("/api")?, "/api");
///     assert_eq!(RadixRule::from_param(":id")?, ":id");
///     assert_eq!(RadixRule::from_glob("*")?, "*");
///     assert_eq!(RadixRule::from_regex(r"{id:\d+}")?, r"{id:\d+}");
///
///     assert_ne!(RadixRule::from_plain("/api")?, "");
///     assert_ne!(RadixRule::from_param(":id")?, ":");
///     assert_ne!(RadixRule::from_glob("*")?, "**");
///     assert_ne!(RadixRule::from_regex(r"{id:\d+}")?, r"{}");
///
///     Ok(())
/// }
/// ```
impl PartialEq<&[u8]> for RadixRule {
    #[inline]
    fn eq(&self, other: &&[u8]) -> bool {
        self.origin() == *other
    }
}

/// == & !=
impl<const N: usize> PartialEq<&[u8; N]> for RadixRule {
    #[inline]
    fn eq(&self, other: &&[u8; N]) -> bool {
        self.origin() == other.as_ref()
    }
}

/// == & !=
impl PartialEq<&str> for RadixRule {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        self.origin() == other.as_bytes()
    }
}