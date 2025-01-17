// pub mod hashicorp;
// pub mod keystore;
// pub mod stellar;
// pub mod evm;

// use stellar::StellarSigner;
// use std::env;
// use dotenv::dotenv;
// use hashicorp::HashicorpCloudClient;


fn main() {}
// #[tokio::main]
// async fn main() {
//     dotenv().ok();

//     let args: Vec<String> = env::args().collect();
//     if args.len() != 3 {
//         eprintln!("Usage: {} <key_name> <message>", args[0]);
//         std::process::exit(1);
//     }

//     let key_name = &args[1];
//     let message = &args[2];

//     let client_id = env::var("HASHICORP_CLIENT_ID").expect("HASHICORP_CLIENT_ID must be set");
//     let client_secret = env::var("HASHICORP_CLIENT_SECRET").expect("HASHICORP_CLIENT_SECRET must be set");
//     let org_id = env::var("HASHICORP_ORG_ID").expect("HASHICORP_ORG_ID must be set");
//     let project_id = env::var("HASHICORP_PROJECT_ID").expect("HASHICORP_PROJECT_ID must be set");
//     let app_name = env::var("HASHICORP_APP_NAME").expect("HASHICORP_APP_NAME must be set");

//     let hashicorp = HashicorpCloudClient::new(client_id, client_secret, org_id, project_id, app_name);

//     let response = match hashicorp.get_secret(key_name).await {
//         Ok(secret) => secret,
//         Err(e) => {
//             eprintln!("Failed to retrieve secret: {}", e);
//             std::process::exit(1);
//         }
//     };

//     let signer = StellarSigner::from_str(response.secret.static_version.value);

//     let signature = signer.sign(message);
//     println!("Signature: {:?}", signature)
// }