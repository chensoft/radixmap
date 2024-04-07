//! Radix set implementation
use super::defs::*;
use super::map::*;

/// Radix set build on top of map
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