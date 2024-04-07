//! Radix set implementation
use super::defs::*;
use super::map::*;

/// Radix set build on top of map
#[derive(Default)]
pub struct RadixSet<'k> {
    base: RadixMap<'k, ()>,
}

impl<'k> RadixSet<'k> {
    /// todo
    pub fn insert(&mut self, path: &'k str) -> RadixResult<bool> {
        self.base.insert(path, ()).map(|d| d.is_some())
    }
}