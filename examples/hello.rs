// use radixmap::*;
// use bytes::Bytes;
// 
// fn main() -> RadixResult<()> {
//     let mut map = RadixMap::default();
// 
//     // the final radix tree looks like this
//     // /
//     // └── api
//     //     └── /v
//     //         ├── 1
//     //         │   └── /user
//     //         └── 2
//     //             └── /user
//     //                 └── /12345
//     map.insert("/", "/")?;
//     map.insert("/api/v1", "v1")?;
//     map.insert("/api/v1/user", "user1")?;
//     map.insert("/api/v2", "v2")?;
//     map.insert("/api/v2/user", "user2")?;
//     map.insert("/api/v2/user/12345", "user2-12345")?;
//     map.insert("/api", "api")?;
// 
//     // search the tree and find the data
//     assert_eq!(map.get("/api"), Some(&"api"));
//     assert_eq!(map.get("/api/v1"), Some(&"v1"));
//     assert_eq!(map.get("/api/v2/user/12345"), Some(&"user2-12345"));
// 
//     // iterate the tree with a prefix path
//     let mut iter = map.iter().with_prefix("/api/v2", true);
// 
//     assert_eq!(iter.next(), Some((&Bytes::from("/api/v2"), &"v2")));
//     assert_eq!(iter.next(), Some((&Bytes::from("/api/v2/user"), &"user2")));
//     assert_eq!(iter.next(), Some((&Bytes::from("/api/v2/user/12345"), &"user2-12345")));
//     assert_eq!(iter.next(), None);
// 
//     Ok(())
// }


use radixmap::{RadixMap, RadixResult};

fn main() -> RadixResult<()> {
    let mut map = RadixMap::new();
    map.insert("/api/v1/user/12345", "user1")?;
    map.insert("/api/v2/user/:id", "user2")?;
    map.insert("/api/v3/user/{id:[0-9]+}", "user3")?;
    map.insert("/api/v4/user/{id:[^0-9]+}", "user4")?;
    map.insert("/api/v5/user/*345", "user5")?;
    map.insert("/blog/:date/{author:[^/]+}/*.html", "blog")?;
    map.insert("/blog/:date/{author:[^/]+}/:date/*.html", "blog")?;

    assert_eq!(map.capture("/api/v1/user/12345"), (Some(&"user1"), vec![]));
    assert_eq!(map.capture("/api/v2/user/12345"), (Some(&"user2"), vec![("id".to_string(), "12345")]));
    assert_eq!(map.capture("/api/v3/user/12345"), (Some(&"user3"), vec![("id".to_string(), "12345")]));
    assert_eq!(map.capture("/api/v4/user/12345"), (None, vec![]));
    assert_eq!(map.capture("/api/v5/user/12345"), (Some(&"user5"), vec![("*".to_string(), "12345")]));
    assert_eq!(map.capture("/api/v6"), (None, vec![]));
    assert_eq!(map.capture("/blog/2024-04-10/chensoft/index.php"), (None, vec![("date".to_string(), "2024-04-10"), ("author".to_string(), "chensoft")]));
    assert_eq!(map.capture("/blog/2024-04-10/chensoft/index.html"), (Some(&"blog"), vec![("date".to_string(), "2024-04-10"), ("author".to_string(), "chensoft")]));
    assert_eq!(map.capture("/blog/2024-04-10/chensoft/2024-05-01/index.html"), (Some(&"blog"), vec![("date".to_string(), "2024-04-10"), ("author".to_string(), "chensoft"), ("date".to_string(), "2024-05-01")]));

    Ok(())
}
