use anyhow::Result;
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    XChaCha20Poly1305, XNonce,
};
use hkdf::Hkdf;
use rand::rngs::OsRng;
use sha2::Sha256;
use x25519_dalek::{EphemeralSecret, PublicKey};


/// Ephemeral key pair for X25519 handshake
pub struct KeyPair {
    pub public: PublicKey,
    pub secret: EphemeralSecret,
}

impl KeyPair {
    pub fn generate() -> Self {
        let secret = EphemeralSecret::random_from_rng(OsRng);
        let public = PublicKey::from(&secret);
        Self { public, secret }
    }
}

/// Derives a 32-byte session key using HKDF-SHA256
pub fn derive_session_key(shared_secret: &[u8], salt: Option<&[u8]>) -> [u8; 32] {
    let hk = Hkdf::<Sha256>::new(salt, shared_secret);
    let mut okm = [0u8; 32];
    hk.expand(b"safesurf-v1-session-key", &mut okm)
        .expect("HKDF expansion failed");
    okm
}

/// Encrypts data using XChaCha20-Poly1305
pub fn encrypt(key: &[u8; 32], nonce: &[u8; 24], plaintext: &[u8]) -> Result<Vec<u8>> {
    let cipher = XChaCha20Poly1305::new(key.into());
    let ciphertext = cipher
        .encrypt(XNonce::from_slice(nonce), plaintext)
        .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;
    Ok(ciphertext)
}

/// Decrypts data using XChaCha20-Poly1305
pub fn decrypt(key: &[u8; 32], nonce: &[u8; 24], ciphertext: &[u8]) -> Result<Vec<u8>> {
    let cipher = XChaCha20Poly1305::new(key.into());
    let plaintext = cipher
        .decrypt(XNonce::from_slice(nonce), ciphertext)
        .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))?;
    Ok(plaintext)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handshake_and_aead() {
        // Alice generates keypair
        let alice_kp = KeyPair::generate();
        // Bob generates keypair
        let bob_kp = KeyPair::generate();

        // Perform Diffie-Hellman
        let alice_shared = alice_kp.secret.diffie_hellman(&bob_kp.public);
        let bob_shared = bob_kp.secret.diffie_hellman(&alice_kp.public);

        assert_eq!(alice_shared.as_bytes(), bob_shared.as_bytes());

        // Derive keys
        let salt = b"test-salt";
        let alice_key = derive_session_key(alice_shared.as_bytes(), Some(salt));
        let bob_key = derive_session_key(bob_shared.as_bytes(), Some(salt));

        assert_eq!(alice_key, bob_key);

        // Encrypt/Decrypt
        let plaintext = b"Top secret message";
        let nonce = [0u8; 24]; // In practice, use random nonces
        let ciphertext = encrypt(&alice_key, &nonce, plaintext).unwrap();
        let decrypted = decrypt(&bob_key, &nonce, &ciphertext).unwrap();

        assert_eq!(plaintext, &decrypted[..]);
    }
}
