use sha2::{Sha256, Digest};

/// Computes the SHA-256 hash of the given data.
/// # Arguments
/// * `data` - A byte slice containing the data to be hashed.
/// # Returns
/// * `String` - The hexadecimal representation of the SHA-256 hash.
pub(crate) fn sha256_hash(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    format!("{:x}", result)
}