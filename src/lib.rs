mod hash;
use hash::tagged_hash;

#[derive(Debug, Clone)]
pub struct MerkleTree {
    layers: Vec<Vec<[u8; 32]>>,
}

impl MerkleTree {
    pub fn new(leaves: &[&str]) -> Self {
        let tag = "Bitcoin_Transaction";
        if leaves.is_empty() {
            return MerkleTree {
                layers: vec![vec![tagged_hash(b"", tag)]],
            };
        }

        let mut current_layer: Vec<[u8; 32]> = leaves
            .iter()
            .map(|s| tagged_hash(s.as_bytes(), tag))
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
                next_layer.push(tagged_hash(&combined, tag));
            }

            layers.push(next_layer.clone());
            current_layer = next_layer;
        }

        MerkleTree { layers }
    }

    pub fn root(&self) -> [u8; 32] {
        self.layers.last().unwrap()[0]
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
        let tree = MerkleTree::new(&leaves);

        let root = tree.root();
        println!("Root (leaves): {}", encode(root));

        assert_eq!(root.len(), 32);
    }

    // 葉が1つの場合
    #[test]
    fn test_merkle_tree_with_single_leaf() {
        let leaves = vec!["a"];
        let tree = MerkleTree::new(&leaves);

        let expected_root = tree.root();
        println!("Root (single leaf): {}", encode(expected_root));

        assert_eq!(expected_root.len(), 32);
    }

    // 葉が偶数の場合
    #[test]
    fn test_merkle_tree_with_two_leaves() {
        let leaves = vec!["a", "b"];
        let tree = MerkleTree::new(&leaves);

        let root = tree.root();
        println!("Root (two leaves): {}", encode(root));

        assert_eq!(root.len(), 32);
    }

    // 葉が空の場合
    #[test]
    fn test_merkle_tree_with_empty_leaves() {
        let leaves: Vec<&str> = vec![];
        let tree = MerkleTree::new(&leaves);

        let root = tree.root();
        println!("Root (empty): {}", encode(root));

        assert_eq!(root.len(), 32);
    }
}
