use super::def::*;

pub enum RadixItem<'a> {
    /// /api
    Plain {
        plain: &'a str
    },

    /// /{id:\d+}
    Regex {
        ident: &'a str,
        regex: regex::Regex,
    },

    /// /:id
    Param {
        ident: &'a str
    },

    /// /*
    Glob {
        glob: glob::Pattern
    },
}

impl<'a> Default for RadixItem<'a> {
    fn default() -> Self {
        Self::Plain { plain: "" }
    }
}

impl<'a> RadixItem<'a> {
    /// ```
    /// use preway::{item::RadixItem};
    ///
    /// assert!(RadixItem::new(r"{id:\d+}").is_ok());
    /// assert!(RadixItem::new(r":id").is_ok());
    /// assert!(RadixItem::new(r"*id").is_ok());
    /// assert!(RadixItem::new(r"?id").is_ok());
    /// assert!(RadixItem::new(r"[0..9]").is_ok());
    /// assert!(RadixItem::new(r"id").is_ok());
    pub fn new(part: &'a str) -> Result<Self> {
        match part.as_bytes().first() {
            Some(b'{') => Self::new_regex(part),
            Some(b':') => Self::new_param(part),
            Some(b'*') | Some(b'?') | Some(b'[') => Self::new_glob(part),
            _ => Self::new_plain(part)
        }
    }

    /// ```
    /// use preway::{item::RadixItem};
    ///
    /// assert!(RadixItem::new_plain(r"").is_ok());
    /// assert!(RadixItem::new_plain(r"id").is_ok());
    /// ```
    pub fn new_plain(part: &'a str) -> Result<Self> {
        Ok(Self::Plain { plain: part })
    }

    /// ```
    /// use preway::{item::RadixItem};
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
    pub fn new_regex(mut part: &'a str) -> Result<Self> {
        if !part.starts_with('{') || !part.ends_with('}') {
            return Err(Error::PathMalformed("regex lack of curly braces".into()).into());
        }

        part = &part[1..part.len() - 1];

        match part.find(':') {
            Some(pos) => Ok(Self::Regex { ident: &part[..pos], regex: regex::Regex::new(&part[pos + 1..])? }),
            None => Ok(Self::Regex { ident: "", regex: regex::Regex::new(part)? })
        }
    }

    /// ```
    /// use preway::{item::RadixItem};
    ///
    /// assert!(RadixItem::new_param(r":").is_ok());   // segment placeholder
    /// assert!(RadixItem::new_param(r":id").is_ok()); // param with a name
    /// assert!(RadixItem::new_param(r"").is_err());   // missing :
    /// assert!(RadixItem::new_param(r"id").is_err()); // missing :
    pub fn new_param(part: &'a str) -> Result<Self> {
        if !part.starts_with(':') {
            return Err(Error::PathMalformed("param lack of colon".into()).into());
        }

        Ok(Self::Param { ident: &part[1..] })
    }

    /// ```
    /// use preway::{item::RadixItem};
    ///
    /// assert!(RadixItem::new_glob(r"*").is_ok());      // match entire string
    /// assert!(RadixItem::new_glob(r"*id").is_ok());    // match strings ending with 'id'
    /// assert!(RadixItem::new_glob(r"?").is_ok());      // match single char
    /// assert!(RadixItem::new_glob(r"?id").is_ok());    // match single char and ending with 'id'
    /// assert!(RadixItem::new_glob(r"[0..9]").is_ok()); // match a range of chars
    /// assert!(RadixItem::new_glob(r"").is_err());      // missing meta chars
    /// assert!(RadixItem::new_glob(r"id").is_err());    // missing meta chars
    pub fn new_glob(part: &'a str) -> Result<Self> {
        match part.as_bytes().first() {
            Some(b'*') | Some(b'?') | Some(b'[') => {}
            _ => return Err(Error::PathMalformed("glob lack of meta chars".into()).into())
        }

        Ok(Self::Glob { glob: glob::Pattern::new(part)? })
    }

    /// ```
    /// use preway::{item::RadixItem};
    ///
    /// assert_eq!(RadixItem::extract(r"api").unwrap(), r"api");
    /// assert_eq!(RadixItem::extract(r"api/v1").unwrap(), r"api/v1");
    /// assert_eq!(RadixItem::extract(r"/api/v1").unwrap(), r"/api/v1");
    /// assert!(RadixItem::extract(r"").is_err());
    ///
    /// assert_eq!(RadixItem::extract(r"{id:\d+}").unwrap(), r"{id:\d+}");
    /// assert_eq!(RadixItem::extract(r"{id:\d+}/rest").unwrap(), r"{id:\d+}");
    /// assert!(RadixItem::extract(r"{id:\d+").is_err());
    /// assert!(RadixItem::extract(r"{id:\d+/rest").is_err());
    ///
    /// assert_eq!(RadixItem::extract(r":").unwrap(), r":");
    /// assert_eq!(RadixItem::extract(r":id").unwrap(), r":id");
    /// assert_eq!(RadixItem::extract(r":id/rest").unwrap(), r":id");
    ///
    /// assert_eq!(RadixItem::extract(r"*").unwrap(), r"*");
    /// assert_eq!(RadixItem::extract(r"*rest").unwrap(), r"*rest");
    /// assert_eq!(RadixItem::extract(r"*/rest").unwrap(), r"*");
    /// assert_eq!(RadixItem::extract(r"**").unwrap(), r"**");
    /// assert_eq!(RadixItem::extract(r"**/rest").unwrap(), r"**");
    ///
    /// assert_eq!(RadixItem::extract(r"?").unwrap(), r"?");
    /// assert_eq!(RadixItem::extract(r"?rest").unwrap(), r"?rest");
    /// assert_eq!(RadixItem::extract(r"?/rest").unwrap(), r"?");
    /// assert_eq!(RadixItem::extract(r"??").unwrap(), r"??");
    /// assert_eq!(RadixItem::extract(r"??/rest").unwrap(), r"??");
    ///
    /// assert_eq!(RadixItem::extract(r"[0..9]").unwrap(), r"[0..9]");
    /// assert_eq!(RadixItem::extract(r"[0..9]/rest").unwrap(), r"[0..9]");
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
            map[b'?' as usize] = true;
            map[b'[' as usize] = true;
            map
        };

        let raw = path.as_bytes();
        let len = match raw.first() {
            Some(b'{') => match raw.iter().position(|c| *c == b'}') {
                Some(pos) => pos + 1,
                _ => return Err(Error::PathMalformed("missing closing sign '}'".into()).into())
            }
            Some(b':') | Some(b'*') | Some(b'?') | Some(b'[') => match raw.iter().position(|c| *c == b'/') {
                Some(pos) => pos,
                _ if !raw.is_empty() => raw.len(),
                _ => return Err(Error::PathMalformed("missing closing sign '/'".into()).into())
            }
            _ => match raw.iter().position(|c| MAP[*c as usize]) {
                Some(pos) => pos,
                None => raw.len(),
            }
        };

        Ok(&path[..len])
    }

    pub fn longest(&self, path: &'a str) -> &'a str {
        // match self {
        //     RadixItem::Plain { pattern } => {
        //         let min = std::cmp::min(pattern.len(), path.len());
        //         let mut len = 0;
        //
        //         while len < min && pattern.as_bytes()[len] == path.as_bytes()[len] {
        //             len += 1;
        //         }
        //
        //         &path[..len]
        //     }
        //     RadixItem::Glob { .. } => { todo!() }
        //     RadixItem::Regex { .. } => { todo!() }
        // }
        todo!()
    }
}