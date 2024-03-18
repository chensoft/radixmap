use crate::*;

pub struct RadixNode<'a, T> {
    hold: Cow<'a, str>,
    item: RadixItem<'a>,
    nest: Vec<RadixNode<'a, T>>,
    data: T,
}

impl<'a, T> Default for RadixNode<'a, T> {
    fn default() -> Self {
        todo!()
    }
}