use preway::*;

fn main() -> anyhow::Result<()> {
    let mut tree = RadixTree::default();

    // the final radix tree looks like this
    // /
    // └── api
    //     └── /v
    //         ├── 1
    //         │   └── /user
    //         └── 2
    //             └── /user
    //                 └── /12345
    tree.insert("/", "/");
    tree.insert("/api/v1", "/api/v1");
    tree.insert("/api/v1/user", "/api/v1/user");
    tree.insert("/api/v2", "/api/v2");
    tree.insert("/api/v2/user", "/api/v2/user");
    tree.insert("/api/v2/user/12345", "/api/v2/user/12345");
    tree.insert("/api", "/api");

    // search the tree and find the data
    assert_eq!(tree.search("/api"), Some(&"/api"));
    assert_eq!(tree.search("/api/v1"), Some(&"/api/v1"));

    // iterate the tree with a prefix path
    for node in tree.prefix("") {
        println!("{}", node.item.pattern);
    }

    Ok(())
}