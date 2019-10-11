//! Internal user authentication modules
//!
//! Hooks up to the secret data store and validates user passphrases,
//! tokens and secrets.

mod pwhash;
pub(crate) use pwhash::PwHash;

use crate::{DataStore, Identity, Persisted, QaulError, QaulResult, Token};
use base64::{encode_config, URL_SAFE};
use rand::prelude::*;
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

/// Internal storage component that tracks user auth state
///
/// Fundamentally it has two functions: hand out authentication
/// tokens, and compare password hashes with their recordings to make
/// sure that users are valid.
pub(crate) struct AuthStore {
    tokens: Arc<Mutex<BTreeMap<Identity, Token>>>,
    hashes: Arc<Mutex<BTreeMap<Identity, PwHash>>>,
}

impl AuthStore {
    pub(crate) fn new() -> Self {
        Self {
            tokens: Arc::new(Mutex::new(BTreeMap::new())),
            hashes: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }

    /// Generate a new login token, if password is valid
    ///
    /// If a token already exists, and the password is valid, it will
    /// be returned instead of generating a new one.
    pub(crate) fn new_login(&mut self, user: Identity, pw: &str) -> QaulResult<Token> {
        self.hashes
            .lock()
            .expect("Failed to unlock hash store")
            .get(&user)
            .filter(|hash| hash.matches_with(pw))
            .map_or(Err(QaulError::UnknownUser), |_| Ok(()))?;

        let mut tokens = self.tokens.lock().expect("Failed to lock token store!");
        let token = tokens
            .get(&user)
            .cloned()
            .map_or_else(|| Ok(Self::generate()), |t| Ok(t))?;

        tokens.insert(user, token.clone());
        Ok(token)
    }

    /// Verify that a user's token is valid
    pub(crate) fn verify_token(&self, user: &Identity, token: &Token) -> QaulResult<()> {
        self.tokens
            .lock()
            .expect("Failed to lock token store")
            .get(user)
            .map(|t| t == token)
            .map_or(Err(QaulError::NotAuthorised), |_| Ok(()))?;

        Ok(())
    }

    pub(crate) fn random(len: usize) -> Vec<u8> {
        (0..)
            .map(|_| rand::thread_rng().next_u64())
            .take(len)
            .map(|x| x.to_be_bytes())
            .fold(Vec::new(), |mut acc, arr| {
                acc.extend(arr.iter().cloned());
                acc
            })
    }

    /// Generate a new base64 encoded token
    fn generate() -> Token {
        let t = Self::random(32);
        encode_config(&t, URL_SAFE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rngs::OsRng, Rng};

    struct Context {
        id1: Identity,
        id1pw: String,

        id2: Identity,
        id2pw: String,

        auth: AuthStore,
    }

    /// A small function that will seed an AuthStore for test purposes
    fn setup() -> Context {
        let id1 = Identity::truncate(&AuthStore::random(12));
        let id1pw = "sunflowers".into();

        let id2 = Identity::truncate(&AuthStore::random(12));
        let id2pw = "mushrooms".into();

        let mut auth = AuthStore::new();
        {
            let mut hashes = auth.hashes.lock().unwrap();
            hashes.insert(id1.clone(), PwHash::new(&id1pw));
            hashes.insert(id2.clone(), PwHash::new(&id2pw));
        }

        Context {
            id1,
            id1pw,
            id2,
            id2pw,
            auth,
        }
    }

    #[test]
    fn collection() {
        let Context {
            id1,
            id1pw,
            id2,
            id2pw,
            mut auth,
        } = setup();

        // Test that correct user gets accepted, wrong gets rejected
        let t1 = auth.new_login(id1.clone(), &id1pw).unwrap();
        assert!(auth.new_login(id2.clone(), &id1pw).is_err());

        // Logging-in again results in the same token
        let t1_2 = auth.new_login(id1.clone(), &id1pw).unwrap();
        assert_eq!(t1, t1_2);

        // Verify "verify_token" endpoint
        assert!(auth.verify_token(&id1, &t1_2).is_ok())
    }
}