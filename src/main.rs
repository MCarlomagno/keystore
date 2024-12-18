pub mod hashicorp;
pub mod keystore;
pub mod utils;

use hashicorp::HashicorpClient;
use keystore::Keystore;
use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <key_name> <message>", args[0]);
        std::process::exit(1);
    }

    let key_name = &args[1];
    let message = &args[2];

    let mut keystore = Keystore::new();

    let client_id = env::var("HASHICORP_CLIENT_ID").expect("HASHICORP_CLIENT_ID must be set");
    let client_secret = env::var("HASHICORP_CLIENT_SECRET").expect("HASHICORP_CLIENT_SECRET must be set");
    let org_id = env::var("HASHICORP_ORG_ID").expect("HASHICORP_ORG_ID must be set");
    let project_id = env::var("HASHICORP_PROJECT_ID").expect("HASHICORP_PROJECT_ID must be set");
    let app_name = env::var("HASHICORP_APP_NAME").expect("HASHICORP_APP_NAME must be set");

    let hashicorp = HashicorpClient::new(client_id, client_secret, org_id, project_id, app_name);

    let response = match hashicorp.get_secret(key_name).await {
        Ok(secret) => secret,
        Err(e) => {
            eprintln!("Failed to retrieve secret: {}", e);
            std::process::exit(1);
        }
    };

    keystore.import_raw_key(response.secret.static_version.value);

    let result = keystore.sign(message);
    match result {
        Some(signature) => println!("Signature: {:?}", signature),
        None => {
            eprintln!("Failed to sign message");
            std::process::exit(1);
        }
    };
}
