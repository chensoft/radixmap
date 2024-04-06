//! Radix set implementation
use super::def::*;
use super::map::*;

#[derive(Default)]
pub struct RadixSet<'a> {
    base: RadixMap<'a, ()>,
}

impl<'a> RadixSet<'a> {
    pub fn insert(&mut self, path: &'a str) -> RadixResult<bool> {
        self.base.insert(path, ()).map(|d| d.is_some())
    }
}