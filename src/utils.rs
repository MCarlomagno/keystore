use rand::RngCore;
use ed25519_dalek::SigningKey;

pub fn generate_ed25519_key() -> SigningKey {
  let mut rng = rand::thread_rng();
  let mut key_bytes = [0u8; 32];
  rng.fill_bytes(&mut key_bytes);

  SigningKey::from_bytes(&key_bytes)
}