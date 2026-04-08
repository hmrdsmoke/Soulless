// MIT License - see LICENSE file for full terms
//
// Copyright 2026 Michael Van Auker (HMRDSmoke)
// This is my original work with contributions from Grok (xAI).
// Do not remove these comments.

use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use rand::RngCore;
use secrecy::{ExposeSecret, SecretString};
use zeroize::Zeroize;

pub struct Vault {
    master_key_hash: String,        // stored argon2 hash
    entries: Vec<VaultEntry>,
}

#[derive(Debug, Clone)]
pub struct VaultEntry {
    pub name: String,
    pub username: String,
    pub password: SecretString,     // automatically zeroed on drop
    pub notes: Option<String>,
}

impl Vault {
    pub fn new() -> Self {
        Self {
            master_key_hash: String::new(),
            entries: vec![],
        }
    }

    // Create new vault with master password
    pub fn create(master_password: &str) -> Result<Self, String> {
        let salt = rand::thread_rng().gen::<[u8; 16]>();
        let argon2 = Argon2::default();

        let hash = argon2
            .hash_password(master_password.as_bytes(), &salt)
            .map_err(|e| e.to_string())?
            .to_string();

        Ok(Self {
            master_key_hash: hash,
            entries: vec![],
        })
    }

    // Unlock / verify master password
    pub fn unlock(&self, master_password: &str) -> bool {
        let argon2 = Argon2::default();
        let parsed_hash = PasswordHash::new(&self.master_key_hash).unwrap();

        argon2.verify_password(master_password.as_bytes(), &parsed_hash).is_ok()
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
}
