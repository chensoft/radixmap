use crate::*;

pub struct RadixItem<'a> {
    pub pattern: &'a str,
}

// pub enum RadixItem<'a> {
//     Plain {
//         pub pattern: &'a str,
//     },
//
//     Glob {
//         pub pattern: glob::Pattern,
//     },
//
//     Regex {
//         pub pattern: regex::Regex,
//     },
// }

impl<'a> Default for RadixItem<'a> {
    fn default() -> Self {
        Self { pattern: "" }
    }
}

impl<'a> RadixItem<'a> {
    pub fn new(path: &'a str) -> Self {
        Self { pattern: path }
    }
}