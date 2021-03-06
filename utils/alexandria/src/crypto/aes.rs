//! Symmetric cipher utilities
//!
//! These functions are only used to bootstrap the unlocking process
//! for the database user table.  For all other cryptographic
//! operations, see the `asym` module instead.

use crate::{
    crypto::{CipherText, Encrypter},
    error::{Error, Result},
    wire::Encoder,
};
use keybob::{Key as KeyBuilder, KeyType};
use serde::{de::DeserializeOwned, Serialize};
use sodiumoxide::crypto::secretbox::{gen_nonce, open, seal, Nonce};

// Make it easier for alexandria internals to use this type
pub(crate) use sodiumoxide::crypto::secretbox::Key;

pub(crate) trait Constructor {
    /// Create an AES symmetric key from a user password and salt
    fn from_pw(pw: &str, salt: &str) -> Self;
}

impl Constructor for Key {
    fn from_pw(pw: &str, salt: &str) -> Self {
        let kb = KeyBuilder::from_pw(KeyType::Aes128, pw, salt);
        Self::from_slice(kb.as_slice()).unwrap()
    }
}

impl<T> Encrypter<T> for Key
where
    T: Encoder<T> + Serialize + DeserializeOwned,
{
    fn seal(&self, data: &T) -> Result<CipherText> {
        let nonce = gen_nonce();
        let encoded = data.encode()?;
        let data = seal(&encoded, &nonce, self);

        Ok(CipherText {
            nonce: nonce.0.iter().cloned().collect(),
            data,
        })
    }

    fn open(&self, data: &CipherText) -> Result<T> {
        let CipherText {
            ref nonce,
            ref data,
        } = data;
        let nonce = Nonce::from_slice(nonce.as_slice()).ok_or(Error::InternalError {
            msg: "Failed to read nonce!".into(),
        })?;
        let clear = open(data.as_slice(), &nonce, self).map_err(|_| Error::InternalError {
            msg: "Failed to decrypt data".into(),
        })?;
        Ok(T::decode(&clear)?)
    }
}

#[test]
fn key_is_key() {
    let k1 = KeyBuilder::from_pw(KeyType::Aes128, "password", "salt");
    let k2 = KeyBuilder::from_pw(KeyType::Aes128, "password", "salt");
    assert_eq!(k1, k2);
}
