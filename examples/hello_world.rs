use preway::*;

fn main() -> anyhow::Result<()> {
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
    map.insert("/api/v1", "/api/v1")?;
    map.insert("/api/v1/user", "/api/v1/user")?;
    map.insert("/api/v2", "/api/v2")?;
    map.insert("/api/v2/user", "/api/v2/user")?;
    map.insert("/api/v2/user/12345", "/api/v2/user/12345")?;
    map.insert("/api", "/api")?;

    // search the tree and find the data
    assert_eq!(map.get("/api"), Some(&"/api"));
    assert_eq!(map.get("/api/v1"), Some(&"/api/v1"));

    // iterate the tree with a prefix path
    // for node in map.iter().with_prefix("/api") {
    //     println!("{}", node.item.pattern);
    // }

    Ok(())
}