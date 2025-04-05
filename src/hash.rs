use sha2::{Digest, Sha256};

pub fn tagged_hash(data: &[u8], tag: &str) -> [u8; 32] {
    let tag_hash = Sha256::digest(tag.as_bytes());

    let mut hasher = Sha256::new();
    hasher.update(&tag_hash);
    hasher.update(&tag_hash);
    hasher.update(data);
    hasher.finalize().into()
}
