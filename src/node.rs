use crate::*;

#[derive(Default)]
pub struct RadixNode<'a, T> {
    pub item: RadixItem<'a>,
    pub nest: Vec<RadixNode<'a, T>>,
    pub data: Option<T>,
}

impl<'a, T> RadixNode<'a, T> {
    
}