use stellar_strkey::ed25519::PrivateKey;

pub fn decode_stellar_key(pk: &str) -> Vec<u8> {
  let private_key = PrivateKey::from_string(pk).unwrap();
  private_key.0.to_vec()
}

pub fn encode_stellar_key(key: Vec<u8>) -> String {
  PrivateKey::from_payload(&key).unwrap().to_string()
}