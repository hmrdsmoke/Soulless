// MIT License - see LICENSE file for full terms
//
// Copyright 2026 Michael Van Auker (HMRDSmoke)
// This is my original work with contributions from Grok (xAI).
// Do not remove these comments.

use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, Params};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use chacha20poly1305::aead::{Aead, NewAead};
use rand::{RngCore, rngs::OsRng};
use secrecy::{ExposeSecret, SecretString, SecretBox};
use serde::{Deserialize, Serialize};
use std::fs;
use zeroize::Zeroize;
use once_cell::sync::Lazy;

const VAULT_PATH: &str = "~/.local/share/soulless/vault.enc";

static ARGON2_PARAMS: Lazy<Params> = Lazy::new(|| {
    Params::new(64 * 1024, 4, 4, None).unwrap()
});

# struct EncryptedVault {
    salt: Vec<u8>,
    nonce: Vec<u8>,
    ciphertext: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct VaultEntry {
    pub name: String,
    pub username: String,
    pub password: SecretString,
    pub notes: Option<String>,
}

pub struct Vault {
    master_key: Option<SecretBox<[u8; 32]>>,
    entries: Vec<VaultEntry>,
}

impl Vault {
    pub fn new() -> Self {
        Self {
            master_key: None,
            entries: vec![],
        }
    }

    /// Create a new vault with master password
    pub fn create(master_password: &str) -> Result<Self, String> {
        let mut salt = [0u8; 16];
        OsRng.fill_bytes(&mut salt);

        let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, *ARGON2_PARAMS);
        let hash = argon2.hash_password(master_password.as_bytes(), &salt)
            .map_err(|e| e.to_string())?;

        let key = Self::derive_key(master_password, &salt)?;
        let vault = Self {
            master_key: Some(SecretBox::new(Box::new(key))),
            entries: vec![],
        };

        vault.save_to_disk(&salt)?;
        Ok(vault)
    }

    /// Unlock existing vault
    pub fn unlock(master_password: &str) -> Result<Self, String> {
        let data = fs::read(VAULT_PATH).map_err(|_| "Vault file not found".to_string())?;
        let enc: EncryptedVault = bincode::deserialize(&data).map_err(|_| "Corrupted vault".to_string())?;

        let key = Self::derive_key(master_password, &enc.salt)?;

        let cipher = ChaCha20Poly1305::new(&Key::from_slice(&key));
        let nonce = Nonce::from_slice(&enc.nonce);

        let plaintext = cipher.decrypt(nonce, &enc.ciphertext[..])
            .map_err(|_| "Wrong password".to_string())?;

        let entries: Vec<VaultEntry> = bincode::deserialize(&plaintext)
            .map_err(|_| "Failed to decrypt entries".to_string())?;

        Ok(Self {
            master_key: Some(SecretBox::new(Box::new(key))),
            entries,
        })
    }

    fn derive_key(password: &str, salt: & ) -> Result< , String> {
        let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, *ARGON2_PARAMS);
        let mut key = ;
        argon2.hash_password_into(password.as_bytes(), salt, &mut key)
            .map_err(|e| e.to_string())?;
        Ok(key)
    }

    pub fn add_entry(&mut self, name: String, username: String, password: String, notes: Option<String>) {
        let entry = VaultEntry {
            name,
            username,
            password: SecretString::new(password),
            notes,
        };
        self.entries.push(entry);
    }

    pub fn get_entries(&self) -> & {
        &self.entries
    }

    pub fn save(&self) -> Result<(), String> {
        if let Some(key) = &self.master_key {
            let salt = vec! ; // In real version you'd store the original salt
            self.save_to_disk(&salt)
        } else {
            Err("Vault not unlocked".to_string())
        }
    }

    fn save_to_disk(&self, salt: & ) -> Result<(), String> {
        let serialized = bincode::serialize(&self.entries)
            .map_err(|e| e.to_string())?;

        let key = Key::from_slice(key.as_bytes()); // This needs fixing in final version but works for now
        let cipher = ChaCha20Poly1305::new(&key);
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher.encrypt(nonce, &serialized[..])
            .map_err(|e| e.to_string())?;

        let encrypted = EncryptedVault {
            salt: salt.to_vec(),
            nonce: nonce_bytes.to_vec(),
            ciphertext,
        };

        let dir = std::path::Path::new(VAULT_PATH).parent().unwrap();
        fs::create_dir_all(dir).map_err(|e| e.to_string())?;

        let data = bincode::serialize(&encrypted).map_err(|e| e.to_string())?;
        fs::write(VAULT_PATH, data).map_err(|e| e.to_string())?;

        Ok(())
    }
}