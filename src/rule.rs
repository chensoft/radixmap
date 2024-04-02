use super::def::*;

/// An enum representing various matching patterns
#[derive(Clone)]
pub enum RadixRule<'a> {
    /// Plain rule that accepts arbitrary strings
    ///
    /// # Syntax
    ///
    /// - /
    /// - /api
    ///
    Plain {
        text: &'a str
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
        orig: &'a str,
        name: &'a str,
        expr: Regex,
    },

    /// Named param matches a segment of the route
    ///
    /// # Syntax
    ///
    /// - :
    /// - :id
    Param {
        orig: &'a str,
        name: &'a str
    },

    /// Unix glob style matcher, note that it must be the last component of a route
    ///
    /// # Syntax
    ///
    /// - *
    ///
    Glob {
        glob: glob::Pattern
    },
}

impl<'a> RadixRule<'a> {
    /// Analyze the fragment type and create a radix rule
    ///
    /// ```
    /// use radixmap::{rule::RadixRule};
    ///
    /// assert!(RadixRule::new(r"{id:\d+}").is_ok());
    /// assert!(RadixRule::new(r":id").is_ok());
    /// assert!(RadixRule::new(r"*id").is_ok());
    /// assert!(RadixRule::new(r"id").is_ok());
    /// ```
    pub fn new(frag: &'a str) -> RadixResult<Self> {
        match frag.as_bytes().first() {
            Some(b'{') => Self::new_regex(frag),
            Some(b':') => Self::new_param(frag),
            Some(b'*') => Self::new_glob(frag),
            _ => Self::new_plain(frag)
        }
    }

    /// Create a plain text rule
    ///
    /// ```
    /// use radixmap::{rule::RadixRule};
    ///
    /// assert!(RadixRule::new_plain(r"").is_ok());
    /// assert!(RadixRule::new_plain(r"id").is_ok());
    /// ```
    pub fn new_plain(frag: &'a str) -> RadixResult<Self> {
        Ok(Self::Plain { text: frag })
    }

    /// Create a regular expression rule
    ///
    /// ```
    /// use radixmap::{rule::RadixRule};
    ///
    /// assert!(RadixRule::new_regex(r"{}").is_ok());       // useless but valid
    /// assert!(RadixRule::new_regex(r"{:}").is_ok());      // same as above
    /// assert!(RadixRule::new_regex(r"{\d+}").is_ok());    // name is empty
    /// assert!(RadixRule::new_regex(r"{:\d+}").is_ok());   // same as above
    /// assert!(RadixRule::new_regex(r"{id:\d+}").is_ok()); // regex with a name
    /// assert!(RadixRule::new_regex(r"").is_err());        // missing {}
    /// assert!(RadixRule::new_regex(r"\d+").is_err());     // missing {}
    /// assert!(RadixRule::new_regex(r"{").is_err());       // missing }
    /// assert!(RadixRule::new_regex(r"{[0-9}").is_err());  // missing ]
    /// assert!(RadixRule::new_regex(r"{:(0}").is_err());   // missing )
    /// assert!(RadixRule::new_regex(r"{id:(0}").is_err()); // missing )
    /// ```
    pub fn new_regex(frag: &'a str) -> RadixResult<Self> {
        if !frag.starts_with('{') || !frag.ends_with('}') {
            return Err(RadixError::PathMalformed("regex lack of curly braces".into()).into());
        }

        let data = &frag[1..frag.len() - 1];
        let find = match data.find(':') {
            Some(pos) => (&data[..pos], &data[pos + 1..]),
            None => ("", data)
        };

        // regex must match from the beginning, add ^ if needed
        match find.1.as_bytes().first() {
            Some(b'^') => Ok(Self::Regex { orig: frag, name: find.0, expr: Regex::new(find.1)? }),
            _ => Ok(Self::Regex { orig: frag, name: find.0, expr: Regex::new(('^'.to_string() + find.1).as_str())? })
        }
    }

    /// Create a named param rule
    ///
    /// ```
    /// use radixmap::{rule::RadixRule};
    ///
    /// assert!(RadixRule::new_param(r":").is_ok());   // segment placeholder
    /// assert!(RadixRule::new_param(r":id").is_ok()); // param with a name
    /// assert!(RadixRule::new_param(r"").is_err());   // missing :
    /// assert!(RadixRule::new_param(r"id").is_err()); // missing :
    /// ```
    pub fn new_param(frag: &'a str) -> RadixResult<Self> {
        if !frag.starts_with(':') {
            return Err(RadixError::PathMalformed("param lack of colon".into()).into());
        }

        Ok(Self::Param { orig: frag, name: &frag[1..] })
    }

    /// Create a unix glob style rule
    ///
    /// ```
    /// use radixmap::{rule::RadixRule};
    ///
    /// assert!(RadixRule::new_glob(r"*").is_ok());      // match entire string
    /// assert!(RadixRule::new_glob(r"*id").is_ok());    // match strings ending with 'id'
    /// assert!(RadixRule::new_glob(r"").is_err());      // missing rule chars
    /// assert!(RadixRule::new_glob(r"id").is_err());    // missing rule chars
    /// ```
    pub fn new_glob(frag: &'a str) -> RadixResult<Self> {
        match frag.as_bytes().first() {
            Some(b'*') => Ok(Self::Glob { glob: glob::Pattern::new(frag)? }),
            _ => Err(RadixError::PathMalformed("glob lack of rule chars".into()).into())
        }
    }

    /// Extract the path to find the next fragment
    ///
    /// ```
    /// use radixmap::{rule::RadixRule};
    ///
    /// fn main() -> RadixResult<()> {
    ///     assert_eq!(RadixRule::extract(r"api")?, r"api");
    ///     assert_eq!(RadixRule::extract(r"api/v1")?, r"api/v1");
    ///     assert_eq!(RadixRule::extract(r"/api/v1")?, r"/api/v1");
    ///     assert!(RadixRule::extract(r"").is_err());
    ///
    ///     assert_eq!(RadixRule::extract(r"{id:\d+}")?, r"{id:\d+}");
    ///     assert_eq!(RadixRule::extract(r"{id:\d+}/rest")?, r"{id:\d+}");
    ///     assert!(RadixRule::extract(r"{id:\d+").is_err());
    ///     assert!(RadixRule::extract(r"{id:\d+/rest").is_err());
    ///
    ///     assert_eq!(RadixRule::extract(r":")?, r":");
    ///     assert_eq!(RadixRule::extract(r":id")?, r":id");
    ///     assert_eq!(RadixRule::extract(r":id/rest")?, r":id");
    ///
    ///     assert_eq!(RadixRule::extract(r"*")?, r"*");
    ///     assert_eq!(RadixRule::extract(r"*rest")?, r"*rest");
    ///     assert_eq!(RadixRule::extract(r"*/rest")?, r"*/rest");
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn extract(path: &'a str) -> RadixResult<&'a str> {
        if path.is_empty() {
            return Err(RadixError::PathEmpty.into());
        }

        const MAP: [bool; 256] = {
            let mut map = [false; 256];
            map[b'{' as usize] = true;
            map[b':' as usize] = true;
            map[b'*' as usize] = true;
            map
        };

        let raw = path.as_bytes();
        let len = match raw.first() {
            Some(b'{') => match raw.iter().position(|c| *c == b'}') {
                Some(pos) => pos + 1,
                _ => return Err(RadixError::PathMalformed("missing closing sign '}'".into()).into())
            }
            Some(b':') => match raw.iter().position(|c| *c == b'/') {
                Some(pos) => pos,
                _ => raw.len(),
            }
            Some(b'*') => raw.len(),
            _ => match raw.iter().position(|c| MAP[*c as usize]) {
                Some(pos) => pos,
                None => raw.len(),
            }
        };

        Ok(&path[..len])
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        match self {
            RadixRule::Plain { text } => text.is_empty(),
            RadixRule::Regex { orig, .. } => orig.is_empty(),
            RadixRule::Param { orig, .. } => orig.is_empty(),
            RadixRule::Glob { glob } => glob.as_str().is_empty(),
        }
    }

    /// Origin fragment of the rule
    ///
    /// ```
    /// use radixmap::{rule::RadixRule};
    ///
    /// fn main() -> RadixResult<()> {
    ///     assert_eq!(RadixRule::new_plain(r"/api")?.origin(), r"/api");
    ///     assert_eq!(RadixRule::new_regex(r"{id:\d+}")?.origin(), r"{id:\d+}");
    ///     assert_eq!(RadixRule::new_param(r":id")?.origin(), r":id");
    ///     assert_eq!(RadixRule::new_glob(r"*")?.origin(), r"*");
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn origin(&'a self) -> &'a str {
        match self {
            RadixRule::Plain { text } => text,
            RadixRule::Regex { orig, .. } => orig,
            RadixRule::Param { orig, .. } => orig,
            RadixRule::Glob { glob } => glob.as_str(),
        }
    }

    /// Match the path to find the longest shared segment
    ///
    /// ```
    /// use std::cmp::Ordering;
    /// use radixmap::{rule::RadixRule};
    ///
    /// fn main() -> RadixResult<()> {
    ///     assert_eq!(RadixRule::new_plain(r"")?.longest(""), (r"", Ordering::Equal));
    ///     assert_eq!(RadixRule::new_plain(r"")?.longest("api"), (r"", Ordering::Equal));
    ///     assert_eq!(RadixRule::new_plain(r"api")?.longest("api"), (r"api", Ordering::Equal));
    ///     assert_eq!(RadixRule::new_plain(r"api/v1")?.longest("api"), (r"api", Ordering::Greater));
    ///     assert_eq!(RadixRule::new_plain(r"api/v1")?.longest("api/v2"), (r"api/v", Ordering::Greater));
    ///
    ///     assert_eq!(RadixRule::new_regex(r"{}")?.longest("12345/rest"), (r"", Ordering::Equal));
    ///     assert_eq!(RadixRule::new_regex(r"{:}")?.longest("12345/rest"), (r"", Ordering::Equal));
    ///     assert_eq!(RadixRule::new_regex(r"{\d+}")?.longest("12345/rest"), (r"12345", Ordering::Equal));
    ///     assert_eq!(RadixRule::new_regex(r"{:\d+}")?.longest("12345/rest"), (r"12345", Ordering::Equal));
    ///     assert_eq!(RadixRule::new_regex(r"{id:\d+}")?.longest("12345/update"), (r"12345", Ordering::Equal));
    ///
    ///     assert_eq!(RadixRule::new_param(r":")?.longest("12345/rest"), (r"12345", Ordering::Equal));
    ///     assert_eq!(RadixRule::new_param(r":id")?.longest("12345/rest"), (r"12345", Ordering::Equal));
    ///
    ///     assert_eq!(RadixRule::new_glob(r"*")?.longest("12345/rest"), (r"12345/rest", Ordering::Equal));
    ///     assert_eq!(RadixRule::new_glob(r"*id")?.longest("12345/rest"), (r"", Ordering::Equal));
    ///     Ok(())
    /// }
    /// ```
    pub fn longest(&self, path: &'a str) -> (&'a str, Ordering) {
        match self {
            RadixRule::Plain { text } => {
                let min = std::cmp::min(text.len(), path.len());
                let mut len = 0;

                while len < min && text.as_bytes()[len] == path.as_bytes()[len] {
                    len += 1;
                }

                (&path[..len], text.len().cmp(&len))
            }
            RadixRule::Regex { expr, .. } => match expr.find(path) {
                Some(m) => (&path[..m.len()], Ordering::Equal),
                None => ("", Ordering::Equal)
            }
            RadixRule::Param { .. } => match path.find('/') {
                Some(p) => (&path[..p], Ordering::Equal),
                None => ("", Ordering::Equal)
            }
            RadixRule::Glob { glob } => match glob.matches(path) {
                true => (path, Ordering::Equal),
                false => ("", Ordering::Equal)
            }
        }
    }

    /// Divide the rule into two parts
    ///
    /// ```
    /// use radixmap::{rule::RadixRule};
    ///
    /// fn main() -> RadixResult<()> {
    ///     assert_eq!(RadixRule::new_plain(r"/api")?.divide(1)?, RadixRule::new_plain(r"api")?);
    ///     assert!(RadixRule::new_regex(r"{id:\d+}")?.divide(1).is_err());
    ///     assert!(RadixRule::new_param(r":id")?.divide(1).is_err());
    ///     assert!(RadixRule::new_glob(r"*")?.divide(1).is_err());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn divide(&mut self, len: usize) -> RadixResult<RadixRule<'a>> {
        match self {
            RadixRule::Plain { text } if text.len() > len => {
                let rule = RadixRule::new_plain(&text[len..]);
                *text = &text[..len];
                rule
            }
            _ => Err(RadixError::RuleIndivisible.into())
        }
    }
}

/// Debug trait
///
/// ```
/// use radixmap::{rule::RadixRule};
///
/// fn main() -> RadixResult<()> {
///     assert_eq!(format!("{:?}", RadixRule::new_plain(r"/api")?), r"Plain(/api)".to_string());
///     assert_eq!(format!("{:?}", RadixRule::new_regex(r"{id:\d+}")?), r"Regex({id:\d+})".to_string());
///     assert_eq!(format!("{:?}", RadixRule::new_param(r":id")?), r"Param(:id)".to_string());
///     assert_eq!(format!("{:?}", RadixRule::new_glob(r"*")?), r"Glob(*)".to_string());
///
///     Ok(())
/// }
/// ```
impl<'a> Debug for RadixRule<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RadixRule::Plain { text } => write!(f, "Plain({text})"),
            RadixRule::Regex { orig, .. } => write!(f, "Regex({orig})"),
            RadixRule::Param { orig, .. } => write!(f, "Param({orig})"),
            RadixRule::Glob { glob } => write!(f, "Glob({glob})"),
        }
    }
}

/// Default trait
impl<'a> Default for RadixRule<'a> {
    fn default() -> Self {
        Self::Plain { text: "" }
    }
}

/// Hash trait
///
/// ```
/// use std::collections::HashMap;
/// use radixmap::{rule::RadixRule};
///
/// fn main() -> RadixResult<()> {
///     let mut map = HashMap::new();
///     map.insert(RadixRule::new_plain(r"/api")?, r"/api");
///     map.insert(RadixRule::new_regex(r"{id:\d+}")?, r"{id:\d+}");
///     map.insert(RadixRule::new_param(r":id")?, r":id");
///     map.insert(RadixRule::new_glob(r"*")?, r"*");
///
///     assert_eq!(map[&RadixRule::new_plain(r"/api")?], r"/api");
///     assert_eq!(map[&RadixRule::new_regex(r"{id:\d+}")?], r"{id:\d+}");
///     assert_eq!(map[&RadixRule::new_param(r":id")?], r":id");
///     assert_eq!(map[&RadixRule::new_glob(r"*")?], r"*");
///
///     Ok(())
/// }
/// ```
impl<'a> Hash for RadixRule<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            RadixRule::Plain { text } => {
                "Plain".hash(state);
                text.hash(state);
            },
            RadixRule::Regex { orig, .. } => {
                "Regex".hash(state);
                orig.hash(state);
            },
            RadixRule::Param { orig, .. } => {
                "Param".hash(state);
                orig.hash(state);
            },
            RadixRule::Glob { glob } => {
                "Glob".hash(state);
                glob.as_str().hash(state);
            },
        }
    }
}

/// == & !=
impl<'a> Eq for RadixRule<'a> {}

/// == & !=
///
/// ```
/// use radixmap::{rule::RadixRule};
///
/// fn main() -> RadixResult<()> {
///     assert_eq!(RadixRule::new_plain(r"/api")?, RadixRule::new_plain(r"/api")?);
///     assert_eq!(RadixRule::new_regex(r"{id:\d+}")?, RadixRule::new_regex(r"{id:\d+}")?);
///     assert_eq!(RadixRule::new_param(r":id")?, RadixRule::new_param(r":id")?);
///     assert_eq!(RadixRule::new_glob(r"*")?, RadixRule::new_glob(r"*")?);
///
///     assert_ne!(RadixRule::new_plain(r"/api")?, RadixRule::new_plain(r"")?);
///     assert_ne!(RadixRule::new_regex(r"{id:\d+}")?, RadixRule::new_regex(r"{}")?);
///     assert_ne!(RadixRule::new_param(r":id")?, RadixRule::new_param(r":")?);
///     assert_ne!(RadixRule::new_glob(r"*")?, RadixRule::new_glob(r"**")?);
///
///     Ok(())
/// }
/// ```
impl<'a> PartialEq for RadixRule<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RadixRule::Plain { text: a }, RadixRule::Plain { text: b }) => a == b,
            (RadixRule::Regex { orig: a, .. }, RadixRule::Regex { orig: b, .. }) => a == b,
            (RadixRule::Param { orig: a, .. }, RadixRule::Param { orig: b, .. }) => a == b,
            (RadixRule::Glob { glob: a }, RadixRule::Glob { glob: b }) => a == b,
            _ => false
        }
    }
}

impl<'a> PartialEq<str> for RadixRule<'a> {
    fn eq(&self, other: &str) -> bool {
        match (self, other) {
            (RadixRule::Plain { text: a }, b) => *a == b,
            (RadixRule::Regex { orig: a, .. }, b) => *a == b,
            (RadixRule::Param { orig: a, .. }, b) => *a == b,
            (RadixRule::Glob { glob: a }, b) => a.as_str() == b,
        }
    }
}