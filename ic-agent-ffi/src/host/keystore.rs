//! KeyStore Format Example
//!
//! ```json
//! {
//!     "encoded": ..,
//!     "principal": ..,
//!     "meta": {
//!                 "name": "my account",
//!                 "whenCreated": "2022-05-29 11:24:15.801607 UTC",
//!                 "sigScheme": "ed25519",
//!                 "standard": "pkcs8",
//!                 "encrypt": "scrypt"
//!             }
//! }
//! ```

use chacha20poly1305::{
    aead::{Aead, NewAead},
    ChaCha20Poly1305, Key, Nonce,
};
use chrono::{DateTime, Utc};
use ic_agent::identity::BasicIdentity;
use ic_agent::Identity;
use ic_types::Principal;
use lazy_static::lazy_static;
use ring::{rand::SystemRandom, signature::Ed25519KeyPair};
use serde_derive::{Deserialize, Serialize};

const ARGON2_SALT: &[u8] = b"IDENTITY HOST KEY STORE";

lazy_static! {
    static ref NONCE: Nonce = *Nonce::from_slice(b"unique nonce");
}

#[derive(Debug, Eq, PartialEq, Clone, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostKeyStore {
    encoded: String,
    principal: Principal,
    pub meta: HostKeyStoreMeta,
}

impl HostKeyStore {
    pub fn verify(self, pwd: &str) -> anyhow::Result<Self> {
        let pkcs8 = Self::decode_then_decrypt(self.encoded.clone(), pwd)?;

        let ed25519 = Ed25519KeyPair::from_pkcs8(pkcs8.as_slice())?;
        let identity = BasicIdentity::from_key_pair(ed25519);
        let principal = identity.sender().map_err(|e| anyhow::Error::msg(e))?;

        Ok(Self {
            encoded: self.encoded,
            principal,
            meta: self.meta,
        })
    }

    pub fn random(name: &str, pwd: &str) -> anyhow::Result<Self> {
        let rng = SystemRandom::new();
        let pkcs8 = Ed25519KeyPair::generate_pkcs8(&rng)?;

        Self::from_pkcs8(name, pwd, pkcs8.as_ref())
    }

    pub fn from_pkcs8(name: &str, pwd: &str, pkcs8: &[u8]) -> anyhow::Result<Self> {
        let ed25519 = Ed25519KeyPair::from_pkcs8(pkcs8)?;
        let identity = BasicIdentity::from_key_pair(ed25519);
        let principal = identity.sender().map_err(|e| anyhow::Error::msg(e))?;

        let encoded = Self::encrypt_then_encode(pkcs8, pwd)?;

        Ok(Self {
            encoded,
            principal,
            meta: HostKeyStoreMeta::new(name),
        })
    }

    pub fn to_identity(&self, pwd: &str) -> anyhow::Result<BasicIdentity> {
        let pkcs8 = Self::decode_then_decrypt(self.encoded.clone(), pwd)?;

        let ed25519 = Ed25519KeyPair::from_pkcs8(pkcs8.as_slice())?;
        let identity = BasicIdentity::from_key_pair(ed25519);

        Ok(identity)
    }

    pub fn change_password(&mut self, old_pwd: &str, new_pwd: &str) -> anyhow::Result<()> {
        let pkcs8 = Self::decode_then_decrypt(self.encoded.clone(), old_pwd)?;
        let encoded = Self::encrypt_then_encode(pkcs8.as_slice(), new_pwd)?;

        self.encoded = encoded;

        Ok(())
    }

    pub fn principal(&self) -> Principal {
        self.principal.clone()
    }

    fn hash_password(pwd: &str) -> anyhow::Result<[u8; 32]> {
        let config = argon2::Config::default();
        let pwd_hash = argon2::hash_raw(pwd.as_bytes(), ARGON2_SALT, &config)?;

        let pwd_hash: &[u8; 32] = pwd_hash.as_slice().try_into()?;

        Ok(pwd_hash.clone())
    }

    fn encrypt_then_encode(content: &[u8], pwd: &str) -> anyhow::Result<String> {
        let pwd_hash = Self::hash_password(pwd)?;

        let key = Key::from_slice(pwd_hash.as_slice());
        let cipher = ChaCha20Poly1305::new(key);
        let encrypted = cipher.encrypt(&NONCE, content)?;
        let encoded = base64::encode(encrypted.as_slice());

        Ok(encoded)
    }

    fn decode_then_decrypt(encoded: String, pwd: &str) -> anyhow::Result<Vec<u8>> {
        let encrypted = base64::decode(encoded)?;

        let pwd_hash = Self::hash_password(pwd)?;
        let key = Key::from_slice(pwd_hash.as_slice());
        let cipher = ChaCha20Poly1305::new(key);

        let pkcs8 = cipher.decrypt(&NONCE, encrypted.as_slice())?;

        Ok(pkcs8)
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostKeyStoreMeta {
    pub name: String,
    when_created: DateTime<Utc>,
    store_syntax: String,
    sig_scheme: String,
    encrypt_scheme: (String, String),
}

impl HostKeyStoreMeta {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            when_created: Utc::now(),
            store_syntax: "pkcs8".into(),
            sig_scheme: "ed25519".into(),
            encrypt_scheme: ("argon2".into(), "chacha20-poly1305".into()),
        }
    }

    pub fn when_created(&self) -> DateTime<Utc> {
        self.when_created
    }

    pub fn store_syntax(&self) -> &str {
        &self.store_syntax
    }

    pub fn sig_scheme(&self) -> &str {
        &self.sig_scheme
    }

    pub fn encrypt_scheme(&self) -> (&str, &str) {
        (&self.encrypt_scheme.0, &self.encrypt_scheme.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NAME: &str = "test account";
    const CONTENT: &[u8] = b"content for tests";
    const PASSWORD: &str = "ABCDE123456abcde";
    const ENCODED: &str = "9SS0OTkmaBkLszBx3jqcB4Vyu1PPMCGini7/v3i4MDf4";

    #[test]
    fn encrypt_then_encode_should_work() -> anyhow::Result<()> {
        let encoded = HostKeyStore::encrypt_then_encode(CONTENT, PASSWORD)?;

        assert_eq!(&encoded, ENCODED);

        Ok(())
    }

    #[test]
    fn decode_then_decrypt_should_work() -> anyhow::Result<()> {
        let content = HostKeyStore::decode_then_decrypt(ENCODED.into(), PASSWORD)?;

        assert_eq!(content.as_slice(), CONTENT);

        Ok(())
    }

    #[test]
    fn from_pkcs8_should_work() -> anyhow::Result<()> {
        let rng = SystemRandom::new();
        let pkcs8 = Ed25519KeyPair::generate_pkcs8(&rng)?;

        let ed25519 = Ed25519KeyPair::from_pkcs8(pkcs8.as_ref())?;
        let identity = BasicIdentity::from_key_pair(ed25519);
        let principal = identity.sender().map_err(|e| anyhow::Error::msg(e))?;

        let key_store = HostKeyStore::from_pkcs8(NAME, PASSWORD, pkcs8.as_ref())?;

        let pkcs8_infer = HostKeyStore::decode_then_decrypt(key_store.encoded, PASSWORD)?;

        assert_eq!(pkcs8_infer.as_slice(), pkcs8.as_ref());
        assert_eq!(key_store.principal, principal);
        assert_eq!(key_store.meta.name, NAME);

        Ok(())
    }

    #[test]
    fn random_should_work() -> anyhow::Result<()> {
        let key_store = HostKeyStore::random(NAME, PASSWORD)?;

        let pkcs8_infer = HostKeyStore::decode_then_decrypt(key_store.encoded, PASSWORD)?;

        let ed25519 = Ed25519KeyPair::from_pkcs8(pkcs8_infer.as_slice())?;
        let identity = BasicIdentity::from_key_pair(ed25519);
        let principal = identity.sender().map_err(|e| anyhow::Error::msg(e))?;

        assert_eq!(key_store.principal, principal);
        assert_eq!(key_store.meta.name, NAME);

        Ok(())
    }

    #[test]
    fn to_identity_should_work() -> anyhow::Result<()> {
        let key_store = HostKeyStore::random(NAME, PASSWORD)?;

        let identity = key_store.to_identity(PASSWORD)?;
        let principal = identity.sender().unwrap_or(Principal::anonymous());

        assert_eq!(key_store.principal, principal);

        Ok(())
    }

    #[test]
    fn update_pwd_should_work() -> anyhow::Result<()> {
        const NEW_PASSWORD: &str = "123456ABCDEabcde";

        let mut key_store = HostKeyStore::random(NAME, PASSWORD)?;

        let pkcs8_infer_old =
            HostKeyStore::decode_then_decrypt(key_store.encoded.clone(), PASSWORD)?;

        let _ = key_store.change_password(PASSWORD, NEW_PASSWORD)?;

        let pkcs8_infer_new = HostKeyStore::decode_then_decrypt(key_store.encoded, NEW_PASSWORD)?;

        assert_eq!(pkcs8_infer_old, pkcs8_infer_new);

        Ok(())
    }
}
