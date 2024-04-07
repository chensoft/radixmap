//! Radix set implementation
use super::defs::*;
use super::map::*;

/// todo
#[derive(Default)]
pub struct RadixSet<'a> {
    base: RadixMap<'a, ()>,
}

impl<'a> RadixSet<'a> {
    /// todo
    pub fn insert(&mut self, path: &'a str) -> RadixResult<bool> {
        self.base.insert(path, ()).map(|d| d.is_some())
    }
}