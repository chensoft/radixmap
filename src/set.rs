//! Radix set implementation
use super::defs::*;
use super::map::{self, RadixMap};

/// Radix set build on top of map
pub struct RadixSet {
    /// The internal map
    base: RadixMap<()>,
}

impl RadixSet {
    /// For consistency with the standard library, we provide this fn to create an empty set
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// The size of the set
    ///
    /// ```
    /// use radixmap::{RadixSet, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut set = RadixSet::new();
    ///     set.insert("/api/v1")?;
    ///     set.insert("/api/v2")?;
    ///
    ///     assert_eq!(set.len(), 2);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.base.len()
    }

    /// Check if the set has no data nodes
    ///
    /// ```
    /// use radixmap::{RadixSet, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut set = RadixSet::new();
    ///
    ///     assert_eq!(set.is_empty(), true);
    ///
    ///     set.insert("/api/v1")?;
    ///     set.insert("/api/v2")?;
    ///
    ///     assert_eq!(set.is_empty(), false);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.base.is_empty()
    }

    /// Retrieve the corresponding data and collect named captures
    ///
    /// ```
    /// use radixmap::{RadixSet, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut set = RadixSet::new();
    ///     set.insert("/api/v1/user/12345")?;
    ///     set.insert("/api/v2/user/:id")?;
    ///     set.insert("/api/v3/user/{id:[0-9]+}")?;
    ///     set.insert("/api/v4/user/{id:[^0-9]+}")?;
    ///     set.insert("/api/v5/user/*345")?;
    ///
    ///     assert_eq!(set.capture("/api/v1/user/12345"), (true, vec![]));
    ///     assert_eq!(set.capture("/api/v2/user/12345"), (true, vec![("id".to_string(), "12345")]));
    ///     assert_eq!(set.capture("/api/v3/user/12345"), (true, vec![("id".to_string(), "12345")]));
    ///     assert_eq!(set.capture("/api/v4/user/12345"), (false, vec![]));
    ///     assert_eq!(set.capture("/api/v5/user/12345"), (true, vec![]));
    ///     assert_eq!(set.capture("/api/v6"), (false, vec![]));
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn capture<'u>(&self, path: &'u str) -> (bool, Vec<(String, &'u str)>) {
        let (data, capt) = self.base.capture(path);
        (data.is_some(), capt)
    }

    /// Check if the tree contains specific path
    ///
    /// ```
    /// use radixmap::{RadixSet, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut set = RadixSet::new();
    ///     set.insert("/api/v1")?;
    ///     set.insert("/api/v2")?;
    ///
    ///     assert_eq!(set.contains("/api/v1"), true);
    ///     assert_eq!(set.contains("/api/v2"), true);
    ///     assert_eq!(set.contains("/api/v3"), false);
    ///     assert_eq!(set.contains("/api/v"), false);
    ///     assert_eq!(set.contains("/api"), false);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn contains(&self, path: &str) -> bool {
        self.base.contains_key(path)
    }

    /// Iterate over the set to retrieve nodes' path
    ///
    /// ```
    /// use bytes::Bytes;
    /// use radixmap::{RadixSet, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut set = RadixSet::new();
    ///     set.insert("/api/v1")?;
    ///     set.insert("/api/v1/user")?;
    ///     set.insert("/api/v2")?;
    ///     set.insert("/api/v2/user")?;
    ///     set.insert("/api")?;
    ///
    ///     let mut iter = set.iter();
    ///
    ///     assert_eq!(iter.next(), Some(&Bytes::from("/api")));
    ///     assert_eq!(iter.next(), Some(&Bytes::from("/api/v1")));
    ///     assert_eq!(iter.next(), Some(&Bytes::from("/api/v1/user")));
    ///     assert_eq!(iter.next(), Some(&Bytes::from("/api/v2")));
    ///     assert_eq!(iter.next(), Some(&Bytes::from("/api/v2/user")));
    ///     assert_eq!(iter.next(), None);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn iter(&self) -> Iter<'_> {
        Iter::from(&self.base)
    }

    /// Insert into new data and return true if exist
    ///
    /// ```
    /// use radixmap::{RadixSet, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut set = RadixSet::new();
    ///
    ///     assert_eq!(set.insert("/api/v1")?, false);
    ///     assert_eq!(set.insert("/api/v2")?, false);
    ///     assert_eq!(set.insert("/api/v1")?, true);
    ///     assert_eq!(set.insert("/api/v2")?, true);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn insert(&mut self, path: impl Into<Bytes>) -> RadixResult<bool> {
        self.base.insert(path, ()).map(|data| data.is_some())
    }

    /// Remove the nodes along the path, affecting data nodes only
    ///
    /// ```
    /// use radixmap::{RadixSet, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut set = RadixSet::new();
    ///     set.insert("/api/v1")?;
    ///     set.insert("/api/v2")?;
    ///     set.insert("/api")?;
    ///
    ///     assert_eq!(set.len(), 3);
    ///     assert_eq!(set.remove("/"), false);      // non-data node
    ///     assert_eq!(set.remove("/api"), true);    // len - 1
    ///     assert_eq!(set.remove("/api/v2"), true); // len - 1
    ///     assert_eq!(set.len(), 1);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn remove(&mut self, path: &str) -> bool {
        self.base.remove(path).is_some()
    }

    /// Clear the radix set but preserve its capacity
    ///
    /// ```
    /// use radixmap::{RadixSet, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut set = RadixSet::new();
    ///     set.insert("/api/v1")?;
    ///     set.insert("/api/v2")?;
    ///
    ///     assert_eq!(set.len(), 2);
    ///
    ///     set.clear();
    ///
    ///     assert_eq!(set.is_empty(), true);
    ///     assert_eq!(set.len(), 0);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        self.base.clear();
    }
}

// -----------------------------------------------------------------------------

/// Construct from an array of tuples
///
/// ```
/// use radixmap::{RadixSet, RadixResult};
///
/// fn main() -> RadixResult<()> {
///     let set = RadixSet::try_from(["/api/v1", "/api/v2"])?;
///
///     assert_eq!(set.len(), 2);
///     assert_eq!(set.contains("/api/v1"), true);
///     assert_eq!(set.contains("/api/v2"), true);
///     assert_eq!(set.contains("/api/v3"), false);
///
///     Ok(())
/// }
/// ```
impl<const N: usize> TryFrom<[Bytes; N]> for RadixSet {
    type Error = RadixError;

    #[inline]
    fn try_from(value: [Bytes; N]) -> Result<Self, Self::Error> {
        let mut set = RadixSet::default();

        for path in value {
            set.insert(path)?;
        }

        Ok(set)
    }
}

impl<const N: usize> TryFrom<[&'static str; N]> for RadixSet {
    type Error = RadixError;

    #[inline]
    fn try_from(value: [&'static str; N]) -> Result<Self, Self::Error> {
        value.map(Bytes::from).try_into()
    }
}

/// Default trait
impl Default for RadixSet {
    #[inline]
    fn default() -> Self {
        Self { base: Default::default() }
    }
}

/// Clone trait
///
/// ```
/// use radixmap::{RadixSet, RadixResult};
///
/// fn main() -> RadixResult<()> {
///     let set_a = RadixSet::try_from(["/api/v1", "/api/v2"])?;
///     let set_b = set_a.clone();
///
///     assert_eq!(set_a, set_b);
///
///     Ok(())
/// }
/// ```
impl Clone for RadixSet {
    #[inline]
    fn clone(&self) -> Self {
        Self { base: self.base.clone() }
    }
}

/// Debug trait
///
/// ```
/// use radixmap::{RadixSet, RadixResult};
///
/// fn main() -> RadixResult<()> {
///     let set = RadixSet::try_from(["/api/v1", "/api/v2"])?;
///
///     assert_eq!(format!("{:?}", set).as_str(), r#"{b"/api/v1", b"/api/v2"}"#);
///
///     Ok(())
/// }
/// ```
impl Debug for RadixSet {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

/// == & !=
impl Eq for RadixSet {}

/// == & !=
impl PartialEq for RadixSet {
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base
    }
}

// -----------------------------------------------------------------------------

/// Re-import Order
pub type Order = map::Order;

// -----------------------------------------------------------------------------

/// Re-import Iterator
pub type Iter<'n> = map::Keys<'n, ()>;