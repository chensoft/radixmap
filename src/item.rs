pub struct RadixItem<'a> {
    pub pattern: &'a str,
}

// pub enum RadixItem<'a> {
//     Plain {
//         pattern: &'a str,
//     },
//
//     Glob {
//         pattern: glob::Pattern,
//     },
//
//     Regex {
//         pattern: regex::Regex,
//     },
// }

impl<'a> Default for RadixItem<'a> {
    fn default() -> Self {
        // Self::Plain { pattern: "" }
        Self { pattern: "" }
    }
}

impl<'a> RadixItem<'a> {
    pub fn new(path: &'a str) -> Self {
        // todo
        // Self::Plain { pattern: path }
        Self { pattern: path }
    }
}