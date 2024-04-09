//! Rule represents a match
use super::defs::*;

/// An enum representing various matching patterns
#[derive(Clone)]
pub enum RadixRule<'k> {
    /// Plain rule that accepts arbitrary strings
    ///
    /// # Syntax
    ///
    /// - /
    /// - /api
    ///
    Plain {
        /// fragment
        frag: &'k str
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
        frag: &'k str,

        /// param's name
        name: &'k str
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
        frag: &'k str,

        /// regex's name
        name: &'k str,

        /// the regex
        expr: Regex,
    },

    /// Unix glob style matcher, note that it must be the last component of a route
    ///
    /// # Syntax
    ///
    /// - *
    ///
    Glob {
        /// fragment
        frag: &'k str,

        /// glob pattern
        glob: glob::Pattern
    },
}

impl<'k> RadixRule<'k> {
    /// Create a plain text rule
    ///
    /// ```
    /// use radixmap::{rule::RadixRule};
    ///
    /// assert!(RadixRule::from_plain(r"").is_ok());
    /// assert!(RadixRule::from_plain(r"id").is_ok());
    /// ```
    #[inline]
    pub fn from_plain(frag: &'k str) -> RadixResult<Self> {
        Ok(Self::Plain { frag })
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
    pub fn from_param(frag: &'k str) -> RadixResult<Self> {
        if !frag.starts_with(':') {
            return Err(RadixError::PathMalformed("param lack of colon".into()));
        }

        Ok(Self::Param { frag, name: &frag[1..] })
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
    pub fn from_regex(frag: &'k str) -> RadixResult<Self> {
        if !frag.starts_with('{') || !frag.ends_with('}') {
            return Err(RadixError::PathMalformed("regex lack of curly braces".into()));
        }

        let data = &frag[1..frag.len() - 1];
        let find = match data.find(':') {
            Some(pos) => (&data[..pos], &data[pos + 1..]),
            None => ("", data)
        };

        // regex must match from the beginning, add ^ if needed
        match find.1.as_bytes().first() {
            Some(b'^') => Ok(Self::Regex { frag, name: find.0, expr: Regex::new(find.1)? }),
            _ => Ok(Self::Regex { frag, name: find.0, expr: Regex::new(('^'.to_string() + find.1).as_str())? })
        }
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
    pub fn from_glob(frag: &'k str) -> RadixResult<Self> {
        match frag.as_bytes().first() {
            Some(b'*') => Ok(Self::Glob { frag, glob: glob::Pattern::new(frag)? }),
            _ => Err(RadixError::PathMalformed("glob lack of asterisk".into()))
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
    ///
    ///     assert_eq!(RadixRule::from_param(r":")?.longest("12345/rest"), (r"12345", Ordering::Equal));
    ///     assert_eq!(RadixRule::from_param(r":id")?.longest("12345/rest"), (r"12345", Ordering::Equal));
    ///
    ///     assert_eq!(RadixRule::from_regex(r"{}")?.longest("12345/rest"), (r"", Ordering::Equal));
    ///     assert_eq!(RadixRule::from_regex(r"{:}")?.longest("12345/rest"), (r"", Ordering::Equal));
    ///     assert_eq!(RadixRule::from_regex(r"{\d+}")?.longest("12345/rest"), (r"12345", Ordering::Equal));
    ///     assert_eq!(RadixRule::from_regex(r"{:\d+}")?.longest("12345/rest"), (r"12345", Ordering::Equal));
    ///     assert_eq!(RadixRule::from_regex(r"{id:\d+}")?.longest("12345/update"), (r"12345", Ordering::Equal));
    ///
    ///     assert_eq!(RadixRule::from_glob(r"*")?.longest("12345/rest"), (r"12345/rest", Ordering::Equal));
    ///     assert_eq!(RadixRule::from_glob(r"*id")?.longest("12345/rest"), (r"", Ordering::Equal));
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn longest<'u>(&self, path: &'u str) -> (&'u str, Ordering) {
        match self {
            RadixRule::Plain { frag } => {
                let min = std::cmp::min(frag.len(), path.len());
                let mut len = 0;

                while len < min && frag.as_bytes()[len] == path.as_bytes()[len] {
                    len += 1;
                }

                (&path[..len], frag.len().cmp(&len))
            }
            RadixRule::Param { .. } => match path.find('/') {
                Some(p) => (&path[..p], Ordering::Equal),
                None => (path, Ordering::Equal)
            }
            RadixRule::Regex { expr, .. } => match expr.find(path) {
                Some(m) => (&path[..m.len()], Ordering::Equal),
                None => ("", Ordering::Equal)
            }
            RadixRule::Glob { glob, .. } => match glob.matches(path) {
                true => (path, Ordering::Equal),
                false => ("", Ordering::Equal)
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
    ///     assert!(RadixRule::from_regex(r"{id:\d+}")?.divide(1).is_err());
    ///     assert!(RadixRule::from_glob(r"*")?.divide(1).is_err());
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn divide(&mut self, len: usize) -> RadixResult<RadixRule<'k>> {
        match self {
            RadixRule::Plain { frag } if frag.len() > len => {
                let rule = RadixRule::from_plain(&frag[len..]);
                *frag = &frag[..len];
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
    ///     assert_eq!(RadixRule::from_regex(r"{id:\d+}")?.origin(), r"{id:\d+}");
    ///     assert_eq!(RadixRule::from_glob(r"*")?.origin(), r"*");
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn origin(&self) -> &'k str {
        match self {
            RadixRule::Plain { frag } => frag,
            RadixRule::Param { frag, .. } => frag,
            RadixRule::Regex { frag, .. } => frag,
            RadixRule::Glob { frag, .. } => frag,
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
    ///     assert_eq!(RadixRule::from_regex(r"{\d+}")?.identity(), r"");
    ///     assert_eq!(RadixRule::from_glob(r"*")?.identity(), r"");
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn identity(&self) -> &'k str {
        match self {
            RadixRule::Param { name, .. } => name,
            RadixRule::Regex { name, .. } => name,
            _ => "",
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
impl<'k> TryFrom<&'k str> for RadixRule<'k> {
    type Error = RadixError;

    fn try_from(path: &'k str) -> Result<Self, Self::Error> {
        if path.is_empty() {
            return Err(RadixError::PathEmpty);
        }

        const MAP: [bool; 256] = {
            let mut map = [false; 256];
            map[b'{' as usize] = true;
            map[b':' as usize] = true;
            map[b'*' as usize] = true;
            map
        };

        let raw = path.as_bytes();
        match raw.first() {
            Some(b':') => match raw.iter().position(|c| *c == b'/') {
                Some(pos) => Self::from_param(&path[..pos]),
                _ => Self::from_param(path),
            }
            Some(b'{') => match raw.iter().position(|c| *c == b'}') {
                Some(pos) => Self::from_regex(&path[..pos + 1]),
                _ => Err(RadixError::PathMalformed("missing closing sign '}'".into()))
            }
            Some(b'*') => {
                Self::from_glob(path)
            }
            _ => match raw.iter().position(|c| MAP[*c as usize]) {
                Some(pos) => Self::from_plain(&path[..pos]),
                None => Self::from_plain(path),
            }
        }
    }
}

/// Default trait
///
/// ```
/// use radixmap::{rule::RadixRule};
///
/// assert_eq!(RadixRule::default(), "");
/// ```
impl<'k> Default for RadixRule<'k> {
    #[inline]
    fn default() -> Self {
        Self::Plain { frag: "" }
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
///     assert_eq!(format!("{:?}", RadixRule::from_regex(r"{id:\d+}")?).as_str(), r"Regex({id:\d+})");
///     assert_eq!(format!("{:?}", RadixRule::from_glob(r"*")?).as_str(), r"Glob(*)");
///
///     Ok(())
/// }
/// ```
impl<'k> Debug for RadixRule<'k> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RadixRule::Plain { frag } => write!(f, "Plain({frag})"),
            RadixRule::Param { frag, .. } => write!(f, "Param({frag})"),
            RadixRule::Regex { frag, .. } => write!(f, "Regex({frag})"),
            RadixRule::Glob { frag, .. } => write!(f, "Glob({frag})"),
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
///     map.insert(RadixRule::from_regex(r"{id:\d+}")?, r"{id:\d+}");
///     map.insert(RadixRule::from_glob(r"*")?, r"*");
///
///     assert_eq!(map[&RadixRule::from_plain(r"/api")?], r"/api");
///     assert_eq!(map[&RadixRule::from_param(r":id")?], r":id");
///     assert_eq!(map[&RadixRule::from_regex(r"{id:\d+}")?], r"{id:\d+}");
///     assert_eq!(map[&RadixRule::from_glob(r"*")?], r"*");
///
///     Ok(())
/// }
/// ```
impl<'k> Hash for RadixRule<'k> {
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
            RadixRule::Regex { frag, .. } => {
                "Regex".hash(state);
                frag.hash(state);
            }
            RadixRule::Glob { frag, .. } => {
                "Glob".hash(state);
                frag.hash(state);
            }
        }
    }
}

/// == & !=
impl<'k> Eq for RadixRule<'k> {}

/// == & !=
///
/// ```
/// use radixmap::{rule::RadixRule, RadixResult};
///
/// fn main() -> RadixResult<()> {
///     assert_eq!(RadixRule::from_plain(r"/api")?, RadixRule::from_plain(r"/api")?);
///     assert_eq!(RadixRule::from_param(r":id")?, RadixRule::from_param(r":id")?);
///     assert_eq!(RadixRule::from_regex(r"{id:\d+}")?, RadixRule::from_regex(r"{id:\d+}")?);
///     assert_eq!(RadixRule::from_glob(r"*")?, RadixRule::from_glob(r"*")?);
///
///     assert_ne!(RadixRule::from_plain(r"/api")?, RadixRule::from_plain(r"")?);
///     assert_ne!(RadixRule::from_param(r":id")?, RadixRule::from_param(r":")?);
///     assert_ne!(RadixRule::from_regex(r"{id:\d+}")?, RadixRule::from_regex(r"{}")?);
///     assert_ne!(RadixRule::from_glob(r"*")?, RadixRule::from_glob(r"**")?);
///
///     // type mismatch
///     assert_ne!(RadixRule::from_plain(r"{}")?, RadixRule::from_regex(r"{}")?);
///
///     Ok(())
/// }
/// ```
impl<'k> PartialEq for RadixRule<'k> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RadixRule::Plain { frag: a }, RadixRule::Plain { frag: b }) => a == b,
            (RadixRule::Param { frag: a, .. }, RadixRule::Param { frag: b, .. }) => a == b,
            (RadixRule::Regex { frag: a, .. }, RadixRule::Regex { frag: b, .. }) => a == b,
            (RadixRule::Glob { frag: a, .. }, RadixRule::Glob { frag: b, .. }) => a == b,
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
///     assert_eq!(RadixRule::from_regex(r"{id:\d+}")?, r"{id:\d+}");
///     assert_eq!(RadixRule::from_glob(r"*")?, r"*");
///
///     assert_ne!(RadixRule::from_plain(r"/api")?, r"");
///     assert_ne!(RadixRule::from_param(r":id")?, r":");
///     assert_ne!(RadixRule::from_regex(r"{id:\d+}")?, r"{}");
///     assert_ne!(RadixRule::from_glob(r"*")?, r"**");
///
///     Ok(())
/// }
/// ```
impl<'k> PartialEq<&str> for RadixRule<'k> {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        self.origin() == *other
    }
}