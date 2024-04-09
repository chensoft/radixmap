use radixmap::*;

fn main() -> RadixResult<()> {
    let mut map = RadixMap::default();

    map.insert("/api/v1/user/:id", "user1")?;
    map.insert("/api/v2/user/{id:[^0-9]+}", "user2")?;
    map.insert("/api/v3/user/*cde", "user3")?;
    map.insert("/", "/")?;

    assert_eq!(map.get("/api/v1/user/12345"), Some(&"user1"));
    assert_eq!(map.get("/api/v2/user/12345"), None);
    assert_eq!(map.get("/api/v2/user/abcde"), Some(&"user2"));
    assert_eq!(map.get("/api/v3/user/12345"), None);
    assert_eq!(map.get("/api/v3/user/abcde"), Some(&"user3"));

    let mut iter = map.iter().with_prefix("/api/v", false);

    assert_eq!(iter.next(), Some(("/api/v1/user/:id", &"user1")));
    assert_eq!(iter.next(), Some(("/api/v2/user/{id:[^0-9]+}", &"user2")));
    assert_eq!(iter.next(), Some(("/api/v3/user/*cde", &"user3")));
    assert_eq!(iter.next(), None);

    Ok(())
}