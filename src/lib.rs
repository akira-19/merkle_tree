mod hash;
use hash::tagged_hash;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleTree {
    layers: Vec<Vec<[u8; 32]>>,
    leaf_tag: String,
    branch_tag: String,
}

impl MerkleTree {
    pub fn new(leaves: &[&str], leaf_tag: String, branch_tag: String) -> Self {
        if leaves.is_empty() {
            return MerkleTree {
                layers: vec![vec![tagged_hash(b"", &leaf_tag)]],
                leaf_tag,
                branch_tag,
            };
        }

        let mut current_layer: Vec<[u8; 32]> = leaves
            .iter()
            .map(|s| tagged_hash(s.as_bytes(), &leaf_tag))
            .collect();

        let mut layers = vec![current_layer.clone()];

        while current_layer.len() > 1 {
            if current_layer.len() % 2 != 0 {
                let last = *current_layer.last().unwrap();
                current_layer.push(last);
            }

            let mut next_layer = Vec::new();
            for pair in current_layer.chunks(2) {
                let combined = [pair[0].as_ref(), pair[1].as_ref()].concat();
                next_layer.push(tagged_hash(&combined, &branch_tag));
            }

            layers.push(next_layer.clone());
            current_layer = next_layer;
        }

        MerkleTree {
            layers,
            leaf_tag,
            branch_tag,
        }
    }

    pub fn root(&self) -> [u8; 32] {
        self.layers.last().unwrap()[0]
    }

    pub fn get_proof(&self, index: usize) -> Vec<(String, u8)> {
        let mut proof = Vec::new();
        let mut idx = index;

        for layer in &self.layers[..self.layers.len() - 1] {
            let pair_idx = if idx % 2 == 0 { idx + 1 } else { idx - 1 };

            if pair_idx < layer.len() {
                let sibling_hash = layer[pair_idx];
                let sibling_hash_hex = hex::encode(sibling_hash);

                let bit = if idx % 2 == 0 { 1 } else { 0 };

                proof.push((sibling_hash_hex, bit));
            }

            idx /= 2;
        }

        proof
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex::encode;

    // テスト用データ
    #[test]
    fn test_merkle_tree_with_odd_number_of_leaves() {
        let leaves = vec!["aaa", "bbb", "ccc", "ddd", "eee"];
        let tree = MerkleTree::new(
            &leaves,
            "ProofOfReserve_Leaf".to_string(),
            "ProofOfReserve_Branch".to_string(),
        );

        let root = tree.root();
        println!("Root (leaves): {}", encode(root));

        assert_eq!(root.len(), 32);
    }

    // 葉が1つの場合
    #[test]
    fn test_merkle_tree_with_single_leaf() {
        let leaves = vec!["a"];
        let tree = MerkleTree::new(
            &leaves,
            "ProofOfReserve_Leaf".to_string(),
            "ProofOfReserve_Branch".to_string(),
        );

        let expected_root = tree.root();
        println!("Root (single leaf): {}", encode(expected_root));

        assert_eq!(expected_root.len(), 32);
    }

    // 葉が偶数の場合
    #[test]
    fn test_merkle_tree_with_two_leaves() {
        let leaves = vec!["a", "b"];
        let tree = MerkleTree::new(
            &leaves,
            "ProofOfReserve_Leaf".to_string(),
            "ProofOfReserve_Branch".to_string(),
        );

        let root = tree.root();
        println!("Root (two leaves): {}", encode(root));

        assert_eq!(root.len(), 32);
    }

    // 葉が空の場合
    #[test]
    fn test_merkle_tree_with_empty_leaves() {
        let leaves: Vec<&str> = vec![];
        let tree = MerkleTree::new(
            &leaves,
            "ProofOfReserve_Leaf".to_string(),
            "ProofOfReserve_Branch".to_string(),
        );

        let root = tree.root();
        println!("Root (empty): {}", encode(root));

        assert_eq!(root.len(), 32);
    }

    #[test]
    fn test_get_proof_index5() {
        let tree = build_test_tree();

        let proof = tree.get_proof(5);

        assert_eq!(proof.len(), 3, "Unexpected proof length");

        let leaf_index4_str = "(5,5555)"; // index 4
        let expected_hash_index4 = {
            let raw = tagged_hash(leaf_index4_str.as_bytes(), "ProofOfReserve_Leaf");
            hex::encode(raw)
        };

        assert_eq!(
            proof[0].0, expected_hash_index4,
            "First sibling hash mismatch"
        );
        assert_eq!(proof[0].1, 0, "First sibling bit mismatch");

        let expected_hash = hex::encode(tree.layers[1][3]);
        assert_eq!(proof[1].0, expected_hash, "Second sibling hash mismatch");
        assert_eq!(proof[1].1, 1, "Second sibling bit mismatch");

        let expected_hash = hex::encode(tree.layers[2][0]);
        assert_eq!(proof[2].0, expected_hash, "Third sibling hash mismatch");
        assert_eq!(proof[2].1, 0, "Third sibling bit mismatch");
    }

    fn build_test_tree() -> MerkleTree {
        // 1) ユーザーデータ (1,1111)～(8,8888)
        let user_db = vec![
            (1, 1111),
            (2, 2222),
            (3, 3333),
            (4, 4444),
            (5, 5555),
            (6, 6666),
            (7, 7777),
            (8, 8888),
        ];

        let leaf_strings: Vec<String> = user_db
            .iter()
            .map(|(uid, bal)| format!("({},{})", uid, bal))
            .collect();

        let leaves: Vec<&str> = leaf_strings.iter().map(|s| s.as_str()).collect();

        // 3) MerkleTree を構築
        MerkleTree::new(
            &leaves,
            "ProofOfReserve_Leaf".to_string(),
            "ProofOfReserve_Branch".to_string(),
        )
    }
}
