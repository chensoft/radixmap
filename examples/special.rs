use radixmap::*;
use bytes::Bytes;

fn main() -> RadixResult<()> {
    let mut map = RadixMap::default();

    map.insert("/api/v1/user/:id", "user1")?;
    map.insert("/api/v2/user/{id:[^0-9]+}", "user2")?;
    map.insert("/api/v3/user/*cde", "user3")?;
    map.insert("/", "/")?;

    assert_eq!(map.capture(b"/api/v1/user/12345"), (Some(&"user1"), vec![(Bytes::from("id"), "12345".as_bytes())]));
    assert_eq!(map.capture(b"/api/v2/user/12345"), (None, vec![]));
    assert_eq!(map.capture(b"/api/v2/user/abcde"), (Some(&"user2"), vec![(Bytes::from("id"), "abcde".as_bytes())]));
    assert_eq!(map.capture(b"/api/v3/user/12345"), (None, vec![]));
    assert_eq!(map.capture(b"/api/v3/user/abcde"), (Some(&"user3"), vec![]));

    let mut iter = map.iter().with_prefix(b"/api/v", false);

    assert_eq!(iter.next(), Some((&Bytes::from("/api/v1/user/:id"), &"user1")));
    assert_eq!(iter.next(), Some((&Bytes::from("/api/v2/user/{id:[^0-9]+}"), &"user2")));
    assert_eq!(iter.next(), Some((&Bytes::from("/api/v3/user/*cde"), &"user3")));
    assert_eq!(iter.next(), None);

    Ok(())
}