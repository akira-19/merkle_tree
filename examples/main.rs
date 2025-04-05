use hex::encode;
use merkle_tree::MerkleTree;

fn main() {
    let leaves = vec!["a", "b", "c"];
    let tree = MerkleTree::new(&leaves);
    println!("Merkle root: {}", encode(tree.root()));
}
