use radixmap::*;
use bytes::Bytes;

fn main() -> RadixResult<()> {
    let mut map = RadixMap::default();

    // the final radix tree looks like this
    // /
    // └── api
    //     └── /v
    //         ├── 1
    //         │   └── /user
    //         └── 2
    //             └── /user
    //                 └── /12345
    map.insert("/", "/")?;
    map.insert("/api/v1", "v1")?;
    map.insert("/api/v1/user", "user1")?;
    map.insert("/api/v2", "v2")?;
    map.insert("/api/v2/user", "user2")?;
    map.insert("/api/v2/user/12345", "user2-12345")?;
    map.insert("/api", "api")?;

    // search the tree and find the data
    assert_eq!(map.get(b"/api"), Some(&"api"));
    assert_eq!(map.get(b"/api/v1"), Some(&"v1"));
    assert_eq!(map.get(b"/api/v2/user/12345"), Some(&"user2-12345"));

    // iterate the tree with a prefix path
    let mut iter = map.iter().with_prefix(b"/api/v2", true);

    assert_eq!(iter.next(), Some((&Bytes::from("/api/v2"), &"v2")));
    assert_eq!(iter.next(), Some((&Bytes::from("/api/v2/user"), &"user2")));
    assert_eq!(iter.next(), Some((&Bytes::from("/api/v2/user/12345"), &"user2-12345")));
    assert_eq!(iter.next(), None);

    Ok(())
}