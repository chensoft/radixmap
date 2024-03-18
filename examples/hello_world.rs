use preway::*;

fn main() -> anyhow::Result<()> {
    let mut tree = RadixTree::default();
    tree.insert("/abc", ());

    Ok(())
}