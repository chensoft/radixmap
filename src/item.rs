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
        param: &'a str
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
    const META: [bool; 256] = {
        let mut map = [false; 256];
        map[b'{' as usize] = true;
        map[b':' as usize] = true;
        map[b'*' as usize] = true;
        map[b'?' as usize] = true;
        map[b'[' as usize] = true;
        map
    };

    // pub fn new(frag: &'a str) -> Result<Self> {
    //     match frag.as_bytes().first() {
    //         Some(b'{') => Ok(Self::Regex { ident: &frag.split(':').next().unwrap_or("")[1..], regex: regex::Regex::new(&frag[..-1])? }),
    //         Some(b':') => Ok(Self::Param { param: &frag[1..] }),
    //         Some(b'*') | Some(b'?') | Some(b'[') => Ok(Self::Glob { glob: glob::Pattern::new(frag)? }),
    //         _ => Ok(Self::Plain { plain: frag })
    //     }
    // }

    /// ```
    /// use preway::{item::RadixItem};
    ///
    /// assert!(matches!(RadixItem::segment(r""), Ok(r"")));
    /// assert!(matches!(RadixItem::segment(r"api"), Ok(r"api")));
    /// assert!(matches!(RadixItem::segment(r"api/v1"), Ok(r"api/v1")));
    /// assert!(matches!(RadixItem::segment(r"/api/v1"), Ok(r"/api/v1")));
    ///
    /// assert!(matches!(RadixItem::segment(r"{id:\d+}"), Ok(r"{id:\d+}")));
    /// assert!(matches!(RadixItem::segment(r"{id:\d+}/rest"), Ok(r"{id:\d+}")));
    /// assert!(matches!(RadixItem::segment(r"{id:\d+"), Err(_)));
    /// assert!(matches!(RadixItem::segment(r"{id:\d+/rest"), Err(_)));
    ///
    /// assert!(matches!(RadixItem::segment(r":id"), Ok(r":id")));
    /// assert!(matches!(RadixItem::segment(r":id/rest"), Ok(r":id")));
    ///
    /// assert!(matches!(RadixItem::segment(r"*"), Ok(r"*")));
    /// assert!(matches!(RadixItem::segment(r"*rest"), Ok(r"*rest")));
    /// assert!(matches!(RadixItem::segment(r"*/rest"), Ok(r"*")));
    /// assert!(matches!(RadixItem::segment(r"**"), Ok(r"**")));
    /// assert!(matches!(RadixItem::segment(r"**/rest"), Ok(r"**")));
    ///
    /// assert!(matches!(RadixItem::segment(r"?"), Ok(r"?")));
    /// assert!(matches!(RadixItem::segment(r"?rest"), Ok(r"?rest")));
    /// assert!(matches!(RadixItem::segment(r"?/rest"), Ok(r"?")));
    /// assert!(matches!(RadixItem::segment(r"??"), Ok(r"??")));
    /// assert!(matches!(RadixItem::segment(r"??/rest"), Ok(r"??")));
    ///
    /// assert!(matches!(RadixItem::segment(r"[0..9]"), Ok(r"[0..9]")));
    /// assert!(matches!(RadixItem::segment(r"[0..9]/rest"), Ok(r"[0..9]")));
    /// ```
    pub fn segment(path: &'a str) -> Result<&'a str> {
        let raw = path.as_bytes();
        let end = match raw.first() {
            Some(b'{') => match raw.iter().position(|c| *c == b'}') {
                Some(pos) => pos + 1,
                _ => return Err(Error::PathMalformed("missing closing sign '}'".into()).into())
            }
            Some(b':') | Some(b'*') | Some(b'?') | Some(b'[') => match raw.iter().position(|c| *c == b'/') {
                Some(pos) => pos,
                _ if !raw.is_empty() => raw.len(),
                _ => return Err(Error::PathMalformed("missing closing sign '/'".into()).into())
            }
            _ => match raw.iter().position(|c| Self::META[*c as usize]) {
                Some(pos) => pos,
                None => raw.len(),
            }
        };

        Ok(&path[..end])
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