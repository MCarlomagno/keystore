use ed25519_dalek::{Signer, SigningKey, Signature};
use crate::stellar::decode_stellar_key;

pub struct StellarSigner {
  key: SigningKey
}

impl StellarSigner {
  pub fn from_bytes(key: Vec<u8>) -> Self {
    let key_bytes: [u8; 32] = key.try_into()
      .expect("Invalid key length - must be 32 bytes");

    StellarSigner {
      key: SigningKey::from_bytes(&key_bytes)
    }
  }

  pub fn from_str(key: String) -> Self {
    let decoded = decode_stellar_key(&key);
    let key_bytes: [u8; 32] = decoded.try_into()
      .expect("Invalid key length - must be 32 bytes");
  
    StellarSigner {
      key: SigningKey::from_bytes(&key_bytes)
    }
  }

  pub fn sign(&self, msg: &str) -> Signature {
    self.key.sign(msg.as_bytes())
  }
}