use ed25519_dalek::{Signer, SigningKey};

pub struct Keystore {
  keys: Vec<SigningKey>
}

impl Keystore {
  pub fn new() -> Self {
    Keystore {
      keys: Vec::new()
    }
  }

  pub fn sign(&self, msg: &str) -> Option<ed25519_dalek::Signature> {
    // TODO: add parameter to indicate some specific key.
    return self.keys.first().map(|signing_key| {
      signing_key.sign(msg.as_bytes())
    })
  }

  pub fn import_key(&mut self, key: SigningKey) {
    self.keys.push(key);  
  }

  pub fn import_raw_key(&mut self, raw_key: String) {
    let decoded = hex::decode(&raw_key)
      .expect("Failed to decode hex string");
    let key_bytes: [u8; 32] = decoded.try_into()
      .expect("Invalid key length - must be 32 bytes");
  
    self.keys.push(SigningKey::from_bytes(&key_bytes));  
  }

  pub fn list_keys(&self) -> Vec<SigningKey> {
    self.keys.clone()
  }
}