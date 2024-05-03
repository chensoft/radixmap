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
    /// # Examples
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
    /// # Examples
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
    /// # Examples
    ///
    /// ```
    /// use bytes::Bytes;
    /// use radixmap::{RadixSet, RadixResult};
    ///
    /// macro_rules! verify {
    ///     ($set:expr, $path:expr, $data:expr, $capt:expr) => {{
    ///         let mut vec = vec![];
    ///         assert_eq!($set.capture($path, &mut vec), $data);
    ///         assert_eq!(vec, $capt);
    ///     }};
    /// }
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut set = RadixSet::new();
    ///     set.insert("/api/v1/user/12345")?;
    ///     set.insert("/api/v2/user/:id")?;
    ///     set.insert("/api/v3/user/{id:[0-9]+}")?;
    ///     set.insert("/api/v4/user/{id:[^0-9]+}")?;
    ///     set.insert("/api/v5/user/*345")?;
    ///
    ///     verify!(set, b"/api/v1/user/12345", true, vec![]);
    ///     verify!(set, b"/api/v2/user/12345", true, vec![(Bytes::from("id"), "12345".as_bytes())]);
    ///     verify!(set, b"/api/v3/user/12345", true, vec![(Bytes::from("id"), "12345".as_bytes())]);
    ///     verify!(set, b"/api/v4/user/12345", false, vec![]);
    ///     verify!(set, b"/api/v5/user/12345", true, vec![(Bytes::from("*"), "12345".as_bytes())]);
    ///     verify!(set, b"/api/v6", false, vec![]);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn capture<'u>(&self, path: &'u [u8], capt: &mut Vec<(Bytes, &'u [u8])>) -> bool {
        let data = self.base.capture(path, capt);
        data.is_some()
    }

    /// Check if the tree contains specific path
    ///
    /// # Examples
    ///
    /// ```
    /// use radixmap::{RadixSet, RadixResult};
    ///
    /// fn main() -> RadixResult<()> {
    ///     let mut set = RadixSet::new();
    ///     set.insert("/api/v1")?;
    ///     set.insert("/api/v2")?;
    ///
    ///     assert_eq!(set.contains(b"/api/v1"), true);
    ///     assert_eq!(set.contains(b"/api/v2"), true);
    ///     assert_eq!(set.contains(b"/api/v3"), false);
    ///     assert_eq!(set.contains(b"/api/v"), false);
    ///     assert_eq!(set.contains(b"/api"), false);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn contains(&self, path: &[u8]) -> bool {
        self.base.contains_key(path)
    }

    /// Iterate over the set to retrieve nodes' path
    ///
    /// # Examples
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
    /// # Examples
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
    /// # Examples
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
    ///     assert_eq!(set.remove(b"/"), false);      // non-data node
    ///     assert_eq!(set.remove(b"/api"), true);    // len - 1
    ///     assert_eq!(set.remove(b"/api/v2"), true); // len - 1
    ///     assert_eq!(set.len(), 1);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn remove(&mut self, path: &[u8]) -> bool {
        self.base.remove(path).is_some()
    }

    /// Clear the radix set but preserve its capacity
    ///
    /// # Examples
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
/// # Examples
///
/// ```
/// use radixmap::{RadixSet, RadixResult};
///
/// fn main() -> RadixResult<()> {
///     let set = RadixSet::try_from(["/api/v1", "/api/v2"])?;
///
///     assert_eq!(set.len(), 2);
///     assert_eq!(set.contains(b"/api/v1"), true);
///     assert_eq!(set.contains(b"/api/v2"), true);
///     assert_eq!(set.contains(b"/api/v3"), false);
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

/// Construct from an array of tuples
impl<const N: usize> TryFrom<[&'static [u8]; N]> for RadixSet {
    type Error = RadixError;

    #[inline]
    fn try_from(value: [&'static [u8]; N]) -> Result<Self, Self::Error> {
        value.map(Bytes::from).try_into()
    }
}

/// Construct from an array of tuples
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
/// # Examples
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
/// # Examples
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