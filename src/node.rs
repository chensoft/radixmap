use super::def::*;
use super::item::*;
use super::iter::*;
use super::rule::*;
use super::pack::*;

/// The basic element inside a tree
pub struct RadixNode<'a, V> {
    rule: RadixRule<'a>,
    item: Option<RadixItem<'a, V>>,
    next: RadixPack<'a, V>,
}

impl<'a, V> RadixNode<'a, V> {
    pub fn rule_ref(&self) -> &RadixRule<'a> {
        &self.rule
    }

    pub fn rule_mut(&mut self) -> &mut RadixRule<'a> {
        &mut self.rule
    }

    pub fn item_ref(&self) -> Option<(&'a str, &V)> {
        self.item.as_ref().map(|item| item.as_ref())
    }

    pub fn item_mut(&mut self) -> Option<(&'a str, &mut V)> {
        self.item.as_mut().map(|item| item.as_mut())
    }

    pub fn path_ref(&self) -> Option<&'a str> {
        self.item.as_ref().map(|item| item.as_ref().0)
    }

    pub fn data_ref(&self) -> Option<&V> {
        self.item.as_ref().map(|item| item.as_ref().1)
    }

    pub fn data_mut(&mut self) -> Option<&mut V> {
        self.item.as_mut().map(|item| item.as_mut().1)
    }

    pub fn next_ref(&self) -> &RadixPack<'a, V> {
        &self.next
    }

    pub fn next_mut(&mut self) -> &mut RadixPack<'a, V> {
        &mut self.next
    }

    pub fn is_leaf(&self) -> bool {
        self.next.is_empty()
    }

    pub fn is_empty(&self) -> bool {
        self.item.is_none()
    }

    pub fn iter(&'a self) -> Iter<'a, V> {
        Iter::from(self)
    }

    pub fn deepest(&'a self, path: &'a str) -> Option<&'a RadixNode<'a, V>> {
        match self.iter().with_prefix(path) {
            Ok(mut iter) => iter.next(),
            Err(_) => None
        }
    }

    pub fn deepest_mut(&mut self, path: &str) -> Option<&'a mut RadixNode<'a, V>> {
        todo!()
    }

    pub fn insert(&mut self, path: &'a str, data: V) -> Result<Option<V>> {
        let mut frag = path;

        loop {
            // extract the next path fragment and insert it via pack
            let next = RadixRule::extract(frag)?;
            let slot = self.next.insert(next)?;

            // encountering a leaf node indicates completion of insertion
            if next.len() == frag.len() {
                let prev = std::mem::take(&mut slot.item);
                slot.item = Some(RadixItem::from((path, data)));
                return Ok(prev.map(|item| item.data));
            }

            frag = &frag[next.len()..];
        }
    }

    /// Divide the node into two parts
    ///
    /// ```
    /// use radixmap::{rule::RadixRule, node::RadixNode};
    ///
    /// fn main() -> anyhow::Result<()> {
    ///     let mut node = RadixNode::try_from(("/api", 12345))?;
    ///
    ///     assert_eq!(node.rule_ref(), "/api");
    ///     assert_eq!(node.data_ref(), Some(&12345));
    ///
    ///     let leaf = node.divide(1)?;
    ///
    ///     assert_eq!(node.rule_ref(), "/");
    ///     assert_eq!(node.data_ref(), None);
    ///     assert_eq!(leaf.rule_ref(), "api");
    ///     assert_eq!(leaf.data_ref(), Some(&12345));
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn divide(&mut self, len: usize) -> Result<RadixNode<'a, V>> {
        Ok(RadixNode {
            rule: self.rule.divide(len)?,
            item: std::mem::take(&mut self.item),
            next: std::mem::take(&mut self.next),
        })
    }

    pub fn clear(&mut self) {
        self.rule = RadixRule::default();
        self.item = None;
        self.next.clear();
    }
}

impl<'a, V> From<RadixRule<'a>> for RadixNode<'a, V> {
    fn from(rule: RadixRule<'a>) -> Self {
        Self { rule, item: None, next: Default::default() }
    }
}

impl<'a, V> TryFrom<(&'a str, V)> for RadixNode<'a, V> {
    type Error = anyhow::Error;

    fn try_from((path, data): (&'a str, V)) -> Result<Self> {
        Ok(Self { rule: RadixRule::new(path)?, item: Some(RadixItem::from((path, data))), next: Default::default() })
    }
}

/// Debug trait
///
/// ```
/// use radixmap::{rule::RadixRule, node::RadixNode};
///
/// fn main() -> anyhow::Result<()> {
///     assert_eq!(format!("{:?}", RadixNode::<'_, ()>::from(RadixRule::new_plain(r"/api")?)), r"Plain(/api)".to_string());
///     assert_eq!(format!("{:?}", RadixNode::<'_, ()>::from(RadixRule::new_regex(r"{id:\d+}")?)), r"Regex({id:\d+})".to_string());
///     assert_eq!(format!("{:?}", RadixNode::<'_, ()>::from(RadixRule::new_param(r":id")?)), r"Param(:id)".to_string());
///     assert_eq!(format!("{:?}", RadixNode::<'_, ()>::from(RadixRule::new_glob(r"*")?)), r"Glob(*)".to_string());
///
///     Ok(())
/// }
/// ```
impl<'a, V> Debug for RadixNode<'a, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.rule.fmt(f)
    }
}

/// Create an empty node
impl<'a, V> Default for RadixNode<'a, V> {
    fn default() -> Self {
        Self { rule: RadixRule::default(), item: None, next: RadixPack::default() }
    }
}

impl<'a, V: Clone> Clone for RadixNode<'a, V> {
    fn clone(&self) -> Self {
        Self {
            rule: self.rule.clone(),
            item: self.item.clone(),
            next: self.next.clone(),
        }
    }
}

// -----------------------------------------------------------------------------

pub struct Keys<'a, V> {
    iter: Iter<'a, V>
}

impl<'a, V> From<NodeRef<'a, V>> for Keys<'a, V> {
    fn from(value: NodeRef<'a, V>) -> Self {
        Self { iter: Iter::from(value) }
    }
}

impl<'a, V> Iterator for Keys<'a, V> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|node| node.path_ref()).flatten()
    }
}

// -----------------------------------------------------------------------------

pub struct Values<'a, V> {
    iter: Iter<'a, V>
}

impl<'a, V> From<NodeRef<'a, V>> for Values<'a, V> {
    fn from(value: NodeRef<'a, V>) -> Self {
        Self { iter: Iter::from(value) }
    }
}

impl<'a, V> Iterator for Values<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|node| node.data_ref()).flatten()
    }
}