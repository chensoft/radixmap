use preway::*;

fn main() -> anyhow::Result<()> {
    let mut tree = RadixTree::default();

    // here is the final radix tree
    // /
    // └── api
    //     └── /v
    //         ├── 1
    //         │   └── /user
    //         └── 2
    //             └── /user
    //                 └── /12345
    tree.insert("/", ());
    tree.insert("/api", ());
    tree.insert("/api/v1", ());
    tree.insert("/api/v1/user", ());
    tree.insert("/api/v2", ());
    tree.insert("/api/v2/user", ());
    tree.insert("/api/v2/user/12345", ());

    // search the tree and return data
    tree.search();

    Ok(())
}