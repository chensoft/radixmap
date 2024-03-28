use super::def::*;

/// An enum representing various matching patterns
#[derive(Clone)]
pub enum RadixItem<'a> {
    /// Plain item that accepts arbitrary strings
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

/// Create a plain text item
impl<'a> Default for RadixItem<'a> {
    fn default() -> Self {
        Self::Plain { text: "" }
    }
}

impl<'a> RadixItem<'a> {
    /// Analyze the fragment type and create a radix item
    ///
    /// ```
    /// use radixmap::{item::RadixItem};
    ///
    /// assert!(RadixItem::new(r"{id:\d+}").is_ok());
    /// assert!(RadixItem::new(r":id").is_ok());
    /// assert!(RadixItem::new(r"*id").is_ok());
    /// assert!(RadixItem::new(r"id").is_ok());
    pub fn new(frag: &'a str) -> Result<Self> {
        match frag.as_bytes().first() {
            Some(b'{') => Self::new_regex(frag),
            Some(b':') => Self::new_param(frag),
            Some(b'*') => Self::new_glob(frag),
            _ => Self::new_plain(frag)
        }
    }

    /// Create a plain text item
    ///
    /// ```
    /// use radixmap::{item::RadixItem};
    ///
    /// assert!(RadixItem::new_plain(r"").is_ok());
    /// assert!(RadixItem::new_plain(r"id").is_ok());
    /// ```
    pub fn new_plain(frag: &'a str) -> Result<Self> {
        Ok(Self::Plain { text: frag })
    }

    /// Create a regular expression item
    ///
    /// ```
    /// use radixmap::{item::RadixItem};
    ///
    /// assert!(RadixItem::new_regex(r"{}").is_ok());       // useless but valid
    /// assert!(RadixItem::new_regex(r"{:}").is_ok());      // same as above
    /// assert!(RadixItem::new_regex(r"{\d+}").is_ok());    // name is empty
    /// assert!(RadixItem::new_regex(r"{:\d+}").is_ok());   // same as above
    /// assert!(RadixItem::new_regex(r"{id:\d+}").is_ok()); // regex with a name
    /// assert!(RadixItem::new_regex(r"").is_err());        // missing {}
    /// assert!(RadixItem::new_regex(r"\d+").is_err());     // missing {}
    /// assert!(RadixItem::new_regex(r"{").is_err());       // missing }
    /// assert!(RadixItem::new_regex(r"{[0-9}").is_err());  // missing ]
    /// assert!(RadixItem::new_regex(r"{:(0}").is_err());   // missing )
    /// assert!(RadixItem::new_regex(r"{id:(0}").is_err()); // missing )
    /// ```
    pub fn new_regex(frag: &'a str) -> Result<Self> {
        if !frag.starts_with('{') || !frag.ends_with('}') {
            return Err(Error::PathMalformed("regex lack of curly braces".into()).into());
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

    /// Create a named param item
    ///
    /// ```
    /// use radixmap::{item::RadixItem};
    ///
    /// assert!(RadixItem::new_param(r":").is_ok());   // segment placeholder
    /// assert!(RadixItem::new_param(r":id").is_ok()); // param with a name
    /// assert!(RadixItem::new_param(r"").is_err());   // missing :
    /// assert!(RadixItem::new_param(r"id").is_err()); // missing :
    pub fn new_param(frag: &'a str) -> Result<Self> {
        if !frag.starts_with(':') {
            return Err(Error::PathMalformed("param lack of colon".into()).into());
        }

        Ok(Self::Param { orig: frag, name: &frag[1..] })
    }

    /// Create a unix glob style item
    ///
    /// ```
    /// use radixmap::{item::RadixItem};
    ///
    /// assert!(RadixItem::new_glob(r"*").is_ok());      // match entire string
    /// assert!(RadixItem::new_glob(r"*id").is_ok());    // match strings ending with 'id'
    /// assert!(RadixItem::new_glob(r"").is_err());      // missing meta chars
    /// assert!(RadixItem::new_glob(r"id").is_err());    // missing meta chars
    pub fn new_glob(frag: &'a str) -> Result<Self> {
        match frag.as_bytes().first() {
            Some(b'*') => Ok(Self::Glob { glob: glob::Pattern::new(frag)? }),
            _ => Err(Error::PathMalformed("glob lack of meta chars".into()).into())
        }
    }

    /// Extract the path to find the next fragment
    ///
    /// ```
    /// use radixmap::{item::RadixItem};
    ///
    /// fn main() -> anyhow::Result<()> {
    ///     assert_eq!(RadixItem::extract(r"api")?, r"api");
    ///     assert_eq!(RadixItem::extract(r"api/v1")?, r"api/v1");
    ///     assert_eq!(RadixItem::extract(r"/api/v1")?, r"/api/v1");
    ///     assert!(RadixItem::extract(r"").is_err());
    ///
    ///     assert_eq!(RadixItem::extract(r"{id:\d+}")?, r"{id:\d+}");
    ///     assert_eq!(RadixItem::extract(r"{id:\d+}/rest")?, r"{id:\d+}");
    ///     assert!(RadixItem::extract(r"{id:\d+").is_err());
    ///     assert!(RadixItem::extract(r"{id:\d+/rest").is_err());
    ///
    ///     assert_eq!(RadixItem::extract(r":")?, r":");
    ///     assert_eq!(RadixItem::extract(r":id")?, r":id");
    ///     assert_eq!(RadixItem::extract(r":id/rest")?, r":id");
    ///
    ///     assert_eq!(RadixItem::extract(r"*")?, r"*");
    ///     assert_eq!(RadixItem::extract(r"*rest")?, r"*rest");
    ///     assert_eq!(RadixItem::extract(r"*/rest")?, r"*/rest");
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn extract(path: &'a str) -> Result<&'a str> {
        if path.is_empty() {
            return Err(Error::PathEmpty.into());
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
                _ => return Err(Error::PathMalformed("missing closing sign '}'".into()).into())
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

    /// Origin fragment of the item
    ///
    /// ```
    /// use radixmap::{item::RadixItem};
    ///
    /// fn main() -> anyhow::Result<()> {
    ///     assert_eq!(RadixItem::new_plain(r"/api")?.origin(), r"/api");
    ///     assert_eq!(RadixItem::new_regex(r"{id:\d+}")?.origin(), r"{id:\d+}");
    ///     assert_eq!(RadixItem::new_param(r":id")?.origin(), r":id");
    ///     assert_eq!(RadixItem::new_glob(r"*")?.origin(), r"*");
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn origin(&'a self) -> &'a str {
        match self {
            RadixItem::Plain { text } => text,
            RadixItem::Regex { orig, .. } => orig,
            RadixItem::Param { orig, .. } => orig,
            RadixItem::Glob { glob } => glob.as_str(),
        }
    }

    /// Match the path to find the longest shared segment
    ///
    /// ```
    /// use std::cmp::Ordering;
    /// use radixmap::{item::RadixItem};
    ///
    /// fn main() -> anyhow::Result<()> {
    ///     assert_eq!(RadixItem::new_plain(r"")?.longest(""), (r"", Ordering::Equal));
    ///     assert_eq!(RadixItem::new_plain(r"")?.longest("api"), (r"", Ordering::Equal));
    ///     assert_eq!(RadixItem::new_plain(r"api")?.longest("api"), (r"api", Ordering::Equal));
    ///     assert_eq!(RadixItem::new_plain(r"api/v1")?.longest("api"), (r"api", Ordering::Greater));
    ///     assert_eq!(RadixItem::new_plain(r"api/v1")?.longest("api/v2"), (r"api/v", Ordering::Greater));
    ///
    ///     assert_eq!(RadixItem::new_regex(r"{}")?.longest("12345/rest"), (r"", Ordering::Equal));
    ///     assert_eq!(RadixItem::new_regex(r"{:}")?.longest("12345/rest"), (r"", Ordering::Equal));
    ///     assert_eq!(RadixItem::new_regex(r"{\d+}")?.longest("12345/rest"), (r"12345", Ordering::Equal));
    ///     assert_eq!(RadixItem::new_regex(r"{:\d+}")?.longest("12345/rest"), (r"12345", Ordering::Equal));
    ///     assert_eq!(RadixItem::new_regex(r"{id:\d+}")?.longest("12345/update"), (r"12345", Ordering::Equal));
    ///
    ///     assert_eq!(RadixItem::new_param(r":")?.longest("12345/rest"), (r"12345", Ordering::Equal));
    ///     assert_eq!(RadixItem::new_param(r":id")?.longest("12345/rest"), (r"12345", Ordering::Equal));
    ///
    ///     assert_eq!(RadixItem::new_glob(r"*")?.longest("12345/rest"), (r"12345/rest", Ordering::Equal));
    ///     assert_eq!(RadixItem::new_glob(r"*id")?.longest("12345/rest"), (r"", Ordering::Equal));
    ///     Ok(())
    /// }
    /// ```
    pub fn longest(&self, path: &'a str) -> (&'a str, Ordering) {
        match self {
            RadixItem::Plain { text } => {
                let min = std::cmp::min(text.len(), path.len());
                let mut len = 0;

                while len < min && text.as_bytes()[len] == path.as_bytes()[len] {
                    len += 1;
                }

                (&path[..len], text.len().cmp(&len))
            }
            RadixItem::Regex { expr, .. } => match expr.find(path) {
                Some(m) => (&path[..m.len()], Ordering::Equal),
                None => ("", Ordering::Equal)
            }
            RadixItem::Param { .. } => match path.find('/') {
                Some(p) => (&path[..p], Ordering::Equal),
                None => ("", Ordering::Equal)
            }
            RadixItem::Glob { glob } => match glob.matches(path) {
                true => (path, Ordering::Equal),
                false => ("", Ordering::Equal)
            }
        }
    }

    /// Divide the item into two parts
    ///
    /// ```
    /// use radixmap::{item::RadixItem};
    ///
    /// fn main() -> anyhow::Result<()> {
    ///     assert_eq!(RadixItem::new_plain(r"/api")?.divide(1)?, RadixItem::new_plain(r"api")?);
    ///     assert!(RadixItem::new_regex(r"{id:\d+}")?.divide(1).is_err());
    ///     assert!(RadixItem::new_param(r":id")?.divide(1).is_err());
    ///     assert!(RadixItem::new_glob(r"*")?.divide(1).is_err());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn divide(&mut self, len: usize) -> Result<RadixItem<'a>> {
        match self {
            RadixItem::Plain { text } if text.len() > len => {
                let item = RadixItem::new_plain(&text[len..]);
                *text = &text[..len];
                item
            }
            _ => Err(Error::ItemIndivisible.into())
        }
    }
}

/// Debug trait
///
/// ```
/// use radixmap::{item::RadixItem};
///
/// fn main() -> anyhow::Result<()> {
///     assert_eq!(format!("{:?}", RadixItem::new_plain(r"/api")?), r"Plain(/api)".to_string());
///     assert_eq!(format!("{:?}", RadixItem::new_regex(r"{id:\d+}")?), r"Regex({id:\d+})".to_string());
///     assert_eq!(format!("{:?}", RadixItem::new_param(r":id")?), r"Param(:id)".to_string());
///     assert_eq!(format!("{:?}", RadixItem::new_glob(r"*")?), r"Glob(*)".to_string());
///
///     Ok(())
/// }
/// ```
impl<'a> Debug for RadixItem<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RadixItem::Plain { text } => write!(f, "Plain({text})"),
            RadixItem::Regex { orig, .. } => write!(f, "Regex({orig})"),
            RadixItem::Param { orig, .. } => write!(f, "Param({orig})"),
            RadixItem::Glob { glob } => write!(f, "Glob({glob})"),
        }
    }
}

/// Hash trait
///
/// ```
/// use std::collections::HashMap;
/// use radixmap::{item::RadixItem};
///
/// fn main() -> anyhow::Result<()> {
///     let mut map = HashMap::new();
///     map.insert(RadixItem::new_plain(r"/api")?, r"/api");
///     map.insert(RadixItem::new_regex(r"{id:\d+}")?, r"{id:\d+}");
///     map.insert(RadixItem::new_param(r":id")?, r":id");
///     map.insert(RadixItem::new_glob(r"*")?, r"*");
///
///     assert_eq!(map[&RadixItem::new_plain(r"/api")?], r"/api");
///     assert_eq!(map[&RadixItem::new_regex(r"{id:\d+}")?], r"{id:\d+}");
///     assert_eq!(map[&RadixItem::new_param(r":id")?], r":id");
///     assert_eq!(map[&RadixItem::new_glob(r"*")?], r"*");
///
///     Ok(())
/// }
/// ```
impl<'a> Hash for RadixItem<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            RadixItem::Plain { text } => {
                "Plain".hash(state);
                text.hash(state);
            },
            RadixItem::Regex { orig, .. } => {
                "Regex".hash(state);
                orig.hash(state);
            },
            RadixItem::Param { orig, .. } => {
                "Param".hash(state);
                orig.hash(state);
            },
            RadixItem::Glob { glob } => {
                "Glob".hash(state);
                glob.as_str().hash(state);
            },
        }
    }
}

/// == & !=
impl<'a> Eq for RadixItem<'a> {}

/// == & !=
///
/// ```
/// use radixmap::{item::RadixItem};
///
/// fn main() -> anyhow::Result<()> {
///     assert_eq!(RadixItem::new_plain(r"/api")?, RadixItem::new_plain(r"/api")?);
///     assert_eq!(RadixItem::new_regex(r"{id:\d+}")?, RadixItem::new_regex(r"{id:\d+}")?);
///     assert_eq!(RadixItem::new_param(r":id")?, RadixItem::new_param(r":id")?);
///     assert_eq!(RadixItem::new_glob(r"*")?, RadixItem::new_glob(r"*")?);
///
///     assert_ne!(RadixItem::new_plain(r"/api")?, RadixItem::new_plain(r"")?);
///     assert_ne!(RadixItem::new_regex(r"{id:\d+}")?, RadixItem::new_regex(r"{}")?);
///     assert_ne!(RadixItem::new_param(r":id")?, RadixItem::new_param(r":")?);
///     assert_ne!(RadixItem::new_glob(r"*")?, RadixItem::new_glob(r"**")?);
///
///     Ok(())
/// }
/// ```
impl<'a> PartialEq for RadixItem<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RadixItem::Plain { text: a }, RadixItem::Plain { text: b }) => a == b,
            (RadixItem::Regex { orig: a, .. }, RadixItem::Regex { orig: b, .. }) => a == b,
            (RadixItem::Param { orig: a, .. }, RadixItem::Param { orig: b, .. }) => a == b,
            (RadixItem::Glob { glob: a }, RadixItem::Glob { glob: b }) => a == b,
            _ => false
        }
    }
}