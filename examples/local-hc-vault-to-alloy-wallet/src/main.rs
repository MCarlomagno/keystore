use alloy_network::EthereumWallet;
use alloy_primitives::FixedBytes;
use alloy_signer_local::LocalSigner;
use oz_keystore::HashicorpLocalClient;
use reqwest::ClientBuilder;

#[tokio::main]
async fn main() {

  let client = ClientBuilder::new()
    .danger_accept_invalid_certs(true)  // Allow self-signed certificates
    .build()
    .unwrap();

  let client = HashicorpLocalClient::new_with_client("https://127.0.0.1:8200", "root", client);

  client.store_secret("my_secret", "0x1234").await.unwrap();

  let secrets = client.list_secrets().await.unwrap();
  println!("all secrets {:?}", secrets);

  let secret = client.get_secret("my_secret").await.unwrap();

  println!("secret: {:?}", secret);

  // // transforms the key into alloy wallet
  // let key_bytes = FixedBytes::from_slice(&key_raw);
  // let signer = LocalSigner::from_bytes(&key_bytes)
  //   .expect("failed to create signer");
  // let wallet = EthereumWallet::from(signer);
  // println!("wallet: {:?}", wallet);
}
