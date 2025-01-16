pub fn decode_evm_key(pk: &str) -> Vec<u8> {
  let decoded = hex::decode(&pk)
    .expect("Failed to decode hex string");
  let key_bytes: [u8; 32] = decoded.try_into()
    .expect("Invalid key length - must be 32 bytes");

  key_bytes.to_vec()
}

pub fn encode_evm_key(key: Vec<u8>) -> String {
  hex::encode(key)
}