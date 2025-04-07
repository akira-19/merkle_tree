use crate::datastore::User;
use anyhow::Result;
use merkle_tree::MerkleTree;
use std::fs;
use std::path::Path;

pub fn create_merkle_tree(conn: &rusqlite::Connection) -> Result<MerkleTree, anyhow::Error> {
    let users = User::all(conn)?;

    let leaf_strings: Vec<String> = users
        .iter()
        .map(|u| format!("({},{})", u.id, u.balance))
        .collect();

    let leaves: Vec<&str> = leaf_strings.iter().map(|s| s.as_str()).collect();

    let tree = MerkleTree::new(
        &leaves,
        "ProofOfReserve_Leaf".to_string(),
        "ProofOfReserve_Branch".to_string(),
    );

    Ok(tree)
}

pub fn save_tree_to_file(tree: &MerkleTree, path: &str) -> Result<(), anyhow::Error> {
    let json_str = serde_json::to_string_pretty(&tree)?;

    fs::write(path, &json_str)?;
    Ok(())
}

pub fn load_tree_from_file(path: &str) -> Result<MerkleTree, anyhow::Error> {
    let loaded_str = fs::read_to_string(path)?;
    let tree: MerkleTree = serde_json::from_str(&loaded_str)?;

    Ok(tree)
}

pub fn get_merkle_root(conn: &rusqlite::Connection, path: &str) -> Result<[u8; 32], anyhow::Error> {
    if Path::new(path).exists() {
        if let Ok(json) = fs::read_to_string(path) {
            if let Ok(tree) = serde_json::from_str::<MerkleTree>(&json) {
                return Ok(tree.root());
            }
        }
    }

    let users = User::all(conn)?;

    let leaf_strings: Vec<String> = users
        .iter()
        .map(|u| format!("({},{})", u.id, u.balance))
        .collect();

    let leaves: Vec<&str> = leaf_strings.iter().map(|s| s.as_str()).collect();

    let tree = MerkleTree::new(
        &leaves,
        "ProofOfReserve_Leaf".to_string(),
        "ProofOfReserve_Branch".to_string(),
    );

    Ok(tree.root())
}

pub fn get_merkle_proof(
    conn: &rusqlite::Connection,
    id: u32,
    path: &str,
) -> Result<Vec<(String, u8)>, anyhow::Error> {
    let user = User::get_by_id(conn, id)?;

    if Path::new(path).exists() {
        if let Ok(json) = fs::read_to_string(path) {
            if let Ok(tree) = serde_json::from_str::<MerkleTree>(&json) {
                return Ok(tree.get_proof(user.idx as usize));
            }
        }
    }

    let users = User::all(conn)?;

    let leaf_strings: Vec<String> = users
        .iter()
        .map(|u| format!("({},{})", u.id, u.balance))
        .collect();

    let leaves: Vec<&str> = leaf_strings.iter().map(|s| s.as_str()).collect();

    let tree = MerkleTree::new(
        &leaves,
        "ProofOfReserve_Leaf".to_string(),
        "ProofOfReserve_Branch".to_string(),
    );

    Ok(tree.get_proof(user.idx as usize))
}
