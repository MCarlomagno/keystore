use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct SecretData {
    secret: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SecretRequest {
    data: SecretData,
}

#[derive(Deserialize, Debug)]
struct SecretResponseData {
  data: Option<SecretData>,
}

#[derive(Deserialize, Debug)]
struct SecretResponse {
  data: Option<SecretResponseData>,
}

#[derive(Deserialize, Debug)]
struct SecretListData {
    keys: Vec<String>,
}


#[derive(Deserialize, Debug)]
struct SecretListResponse {
    request_id: String,
    lease_id: String,
    renewable: bool,
    lease_duration: u64,
    data: SecretListData,
    #[serde(default)]
    wrap_info: Option<()>,
    #[serde(default)]
    warnings: Option<()>,
    #[serde(default)]
    auth: Option<()>,
    mount_type: String,
}

pub struct HashicorpLocalClient {
    client: Client,
    base_url: String,
    token: String,
}

impl HashicorpLocalClient {
    pub fn new(base_url: &str, token: &str) -> Self {
      Self {
        client: Client::new(),
        base_url: base_url.to_string(),
        token: token.to_string(),
      }
    }

    pub fn new_with_client(base_url: &str, token: &str, client: reqwest::Client) -> Self {
      Self {
          base_url: base_url.to_string(),
          token: token.to_string(),
          client,
      }
  }

    pub async fn store_secret(&self, id: &str, secret: &str) -> Result<(), Error> {
      let url = format!("{}/v1/secret/data/{}", self.base_url, id);
      let body = SecretRequest { data: SecretData { secret: secret.to_string() } };
      
      self.client.post(&url)
        .header("X-Vault-Token", &self.token)
        .json(&body)
        .send()
        .await?;
      
      Ok(())
    }

    pub async fn list_secrets(&self) -> Result<Vec<String>, Error> {
      let url = format!("{}/v1/secret/metadata?list=true", self.base_url);
      let response = self.client.get(&url) 
        .header("X-Vault-Token", &self.token)
        .send()
        .await?
        .json::<SecretListResponse>()
        .await?;

      println!("response {:?}", response);
      
      Ok(response.data.keys)
    }

    pub async fn get_secret(&self, id: &str) -> Result<Option<String>, Error> {
      let url = format!("{}/v1/secret/data/{}", self.base_url, id);
      let response: SecretResponse = self.client.get(&url)
        .header("X-Vault-Token", &self.token)
        .send()
        .await?
        .json()
        .await?;
      
        Ok(response.data
          .and_then(|d| d.data)
          .map(|d| d.secret))
    }

    pub async fn delete_secret(&self, id: &str) -> Result<(), Error> {
      let url = format!("{}/v1/secret/data/{}", self.base_url, id);
      self.client.delete(&url)
        .header("X-Vault-Token", &self.token)
        .send()
        .await?;
      
      Ok(())
    }
}
