

pub struct EvmSigner {
  // key: SigningKey
}

impl EvmSigner {
  pub fn from_bytes(_key: Vec<u8>) -> Self {
    todo!("EVM signer not implemented")
  }

  pub fn from_str(_key: String) -> Self {
    todo!("EVM signer not implemented")
  }

  pub fn sign(&self, _msg: &str) {
    todo!("EVM signer not implemented")
  }
}