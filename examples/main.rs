use hex::encode;
use merkle_tree::MerkleTree;

fn main() {
    let leaves = vec!["a", "b", "c"];
    let tree = MerkleTree::new(
        &leaves,
        "ProofOfReserve_Leaf".to_string(),
        "ProofOfReserve_Branch".to_string(),
    );
    println!("Merkle root: {}", encode(tree.root()));
}
