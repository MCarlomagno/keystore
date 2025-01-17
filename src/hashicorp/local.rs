use base64::Engine;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use base64::engine::general_purpose::STANDARD as BASE64;
use alloy_primitives::{PrimitiveSignature as EthSignature, FixedBytes};
use ed25519_dalek::Signature as Ed25519Signature;

#[derive(Debug, Serialize)]
struct SignPayload {
    input: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_version: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prehashed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    signature_algorithm: Option<String>,
}

#[derive(Debug, Serialize)]
struct CreateKeyPayload {
    #[serde(rename = "type")]
    key_type: String,
    exportable: bool,
    allow_plaintext_backup: bool,
}

#[derive(Debug, Deserialize)]
struct SignResponse {
    data: SignData,
}

#[derive(Debug, Deserialize)]
struct SignData {
    signature: String,
}

pub enum KeyType {
  Ed25519,
  ECDSA,
}

impl KeyType {
  fn as_str(&self) -> &'static str {
      match self {
          KeyType::Ed25519 => "ed25519",
          KeyType::ECDSA => "ecdsa-p256",
      }
  }
}

#[derive(Debug)]
pub enum SignatureType {
    Ed25519(Ed25519Signature),  // Using dalek's Signature type
    Ecdsa(EthSignature),
}

pub struct HashicorpLocalClient {
    client: Client,
    token: String,
    address: String,
}

impl HashicorpLocalClient {
    pub fn new(token: String, address: String) -> Self {
        Self {
            client: Client::new(),
            token,
            address,
        }
    }

    pub async fn create_key_transit(
      &self,
      key_name: &str,
      key_type: KeyType,
      exportable: bool,
  ) -> Result<(), Box<dyn Error>> {
      let url = format!(
          "{}/v1/transit/keys/{}",
          self.address.trim_end_matches('/'),
          key_name
      );

      let payload = CreateKeyPayload {
          key_type: key_type.as_str().to_string(),
          exportable,
          allow_plaintext_backup: exportable,
      };

      let response = self
          .client
          .post(&url)
          .header("X-Vault-Token", &self.token)
          .json(&payload)
          .send()
          .await?;

      if !response.status().is_success() {
          return Err(format!(
              "Failed to create key: {} - {}",
              response.status(),
              response.text().await?
          )
          .into());
      }

      Ok(())
    }

    pub async fn sign_transit(
      &self,
      key_name: &str,
      payload: &[u8],
      key_type: KeyType,
  ) -> Result<SignatureType, Box<dyn Error>> {
      let hash_algorithm = match key_type {
          KeyType::ECDSA => Some("sha2-256"),
          KeyType::Ed25519 => None,
      };

      let vault_signature = self.sign_message(key_name, payload, hash_algorithm).await?;
      
      // Extract base64 signature
      let parts: Vec<&str> = vault_signature.split(':').collect();
      if parts.len() != 3 {
          return Err("Invalid vault signature format".into());
      }

      let sig_bytes = BASE64.decode(parts[2])?;
      
      match key_type {
        KeyType::Ed25519 => {
          todo!("parse signature bytes to stellar scheme");
          // Ok(SignatureType::Ed25519(sig_bytes))
        },
        KeyType::ECDSA => {
          todo!("parse signature bytes to EVM scheme");
          // Ok(SignatureType::Ecdsa(sig_bytes))
        }
    }
  }

    async fn sign_message(
      &self,
      key_name: &str,
      payload: &[u8],
      hash_algorithm: Option<&str>,
    ) -> Result<String, Box<dyn Error>> {
      let input = BASE64.encode(payload);

      let payload = SignPayload {
          input,
          key_version: None,
          prehashed: None,
          signature_algorithm: None,
      };

      let url = match hash_algorithm {
          Some(algo) => format!(
              "{}/v1/transit/sign/{}/{}",
              self.address.trim_end_matches('/'),
              key_name,
              algo
          ),
          None => format!(
              "{}/v1/transit/sign/{}",
              self.address.trim_end_matches('/'),
              key_name
          ),
      };

      let response = self
          .client
          .post(&url)
          .header("X-Vault-Token", &self.token)
          .json(&payload)
          .send()
          .await?;

      if !response.status().is_success() {
          return Err(format!(
              "Failed to sign message: {} - {}",
              response.status(),
              response.text().await?
          )
          .into());
      }

      let sign_response: SignResponse = response.json().await?;
      Ok(sign_response.data.signature)
    }
}
