use pbkdf2::pbkdf2_hmac_array;
use sha2::Sha256;
/// Generates a random salt for key derivation.
/// # Arguments
/// * `password` - The password to derive the key from.
/// * `iterations` - The number of iterations for the key derivation function.
/// # Returns
/// * `Vec<u8>` - A vector containing the generated salt.
fn derive_aes_key(password: &str, salt: &[u8], iterations: u32) -> [u8; 32] {
    pbkdf2_hmac_array::<Sha256, 32>(password.as_bytes(), salt, iterations)
}