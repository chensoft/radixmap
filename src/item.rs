pub struct RadixItem<'a, V> {
    pub path: &'a str,
    pub data: V,
}

impl<'a, V> RadixItem<'a, V> {
    pub fn new(path: &'a str, data: V) -> Self {
        Self { path, data }
    }

    pub fn as_ref(&self) -> (&'a str, &V) {
        (self.path, &self.data)
    }

    pub fn as_mut(&mut self) -> (&'a str, &mut V) {
        (self.path, &mut self.data)
    }
}

impl<'a, V: Default> Default for RadixItem<'a, V> {
    fn default() -> Self {
        Self { path: "", data: Default::default() }
    }
}

impl<'a, V: Clone> Clone for RadixItem<'a, V> {
    fn clone(&self) -> Self {
        Self { path: self.path, data: self.data.clone() }
    }
}

impl<'a, V: Eq> Eq for RadixItem<'a, V> {}

impl<'a, V: PartialEq> PartialEq for RadixItem<'a, V> {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path && self.data == other.data
    }
}