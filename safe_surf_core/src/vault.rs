use anyhow::{anyhow, Result};
use argon2::{password_hash::rand_core::OsRng, Argon2};
use crate::crypto::{encrypt, decrypt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use zeroize::Zeroize;

use crate::config::CryptoConfig;
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Zeroize)]
#[zeroize(drop)]
pub struct VaultSecret {
    pub label: String,
    pub username: String,
    pub secret: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Vault {
    pub secrets: HashMap<String, VaultSecret>,
    pub local_salt: [u8; 16], // Salt for blinded indicators
}

pub struct SecureVault {
    key: [u8; 32],
    config: CryptoConfig,
}

impl Drop for SecureVault {
    fn drop(&mut self) {
        self.key.zeroize();
    }
}

impl SecureVault {
    pub fn derive_from_passphrase(passphrase: &[u8], salt: &[u8], config: CryptoConfig) -> Result<Self> {
        let argon2 = Argon2::new(
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            argon2::Params::new(config.argon2_m_cost, config.argon2_t_cost, config.argon2_p_cost, None).unwrap(),
        );
        let mut key = [0u8; 32];
        let mut actual_salt = [0u8; 16];
        let len = salt.len().min(16);
        actual_salt[..len].copy_from_slice(&salt[..len]);

        argon2.hash_password_into(passphrase, &actual_salt, &mut key)
            .map_err(|e| anyhow!("KDF failed: {}", e))?;
        
        Ok(Self { key, config })
    }

    pub fn config(&self) -> &CryptoConfig {
        &self.config
    }

    /// Generates a blinded indicator for a credential (e.g., username + secret)
    /// This allows matching against known leaks without exposing the full credential.
    pub fn generate_indicator(&self, vault: &Vault, label: &str) -> Option<[u8; 32]> {
        let secret = vault.secrets.get(label)?;
        let mut hasher = Sha256::new();
        hasher.update(vault.local_salt);
        hasher.update(&secret.username);
        hasher.update(&secret.secret);
        Some(hasher.finalize().into())
    }

    pub fn seal(&self, vault: &Vault) -> Result<(Vec<u8>, [u8; 24])> {
        let plaintext = serde_json::to_vec(vault)?;
        let mut nonce = [0u8; 24];
        use rand::RngCore;
        OsRng.fill_bytes(&mut nonce);
        
        let ciphertext = encrypt(&self.key, &nonce, &plaintext)?;
        Ok((ciphertext, nonce))
    }

    pub fn unseal(&self, ciphertext: &[u8], nonce: &[u8; 24]) -> Result<Vault> {
        let plaintext = decrypt(&self.key, nonce, ciphertext)?;
        let vault = serde_json::from_slice(&plaintext)?;
        Ok(vault)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vault_seal_unseal() {
        let passphrase = b"correct horse battery staple";
        let salt = b"stable-salt-1234";
        let config = CryptoConfig::default();
        let sv = SecureVault::derive_from_passphrase(passphrase, salt, config).unwrap();

        let mut v = Vault::default();
        v.local_salt = [1u8; 16];
        v.secrets.insert("test".to_string(), VaultSecret {
            label: "Test".to_string(),
            username: "admin".to_string(),
            secret: "password123".to_string(),
        });

        let (ct, nonce) = sv.seal(&v).unwrap();
        let unsealed = sv.unseal(&ct, &nonce).unwrap();

        assert_eq!(unsealed.secrets["test"].secret, "password123");
        
        let indicator = sv.generate_indicator(&unsealed, "test").unwrap();
        assert!(indicator.len() == 32);
    }
}
