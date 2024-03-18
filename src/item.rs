use crate::*;

pub enum RadixItem<'a> {
    None,

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
        Self::None
    }
}