pub enum RadixItem<'a> {
    Plain {
        pattern: &'a str,
    },

    Glob {
        pattern: glob::Pattern,
    },

    Regex {
        pattern: regex::Regex,
    },
}

impl<'a> Default for RadixItem<'a> {
    fn default() -> Self {
        Self::Plain { pattern: "" }
    }
}

impl<'a> RadixItem<'a> {
    pub fn new(frag: &'a str) -> Self {
        // todo
        Self::Plain { pattern: frag }
    }

    pub fn longest(&self, path: &'a str) -> &'a str {
        match self {
            RadixItem::Plain { pattern } => {
                let min = std::cmp::min(pattern.len(), path.len());
                let mut len = 0;

                while len < min && pattern.as_bytes()[len] == path.as_bytes()[len] {
                    len += 1;
                }

                &path[..len]
            }
            RadixItem::Glob { .. } => { todo!() }
            RadixItem::Regex { .. } => { todo!() }
        }
    }
}