## Signet

A tool for creating secure keys and signing raw transactions using ECDSA (ethereum) or ED25519 (stellar, solana) schemas. This enables flexible key management and signing operations through local keystores or HashiCorp's secret management service.

> Warning: this is an experimental project under development.

## Running the project

```bash
cargo run <key_name> <message_to_sign>
```

Where `key_name` is the name of the secret key in the vault, and `message_to_sign` is any arbitrary string.


## Getting started with Hashicorp

1. Create a [Hashicorp account](https://portal.cloud.hashicorp.com/sign-in)
2. Create new organization
3. Go to [secrets app](portal.cloud.hashicorp.com/services/secrets/apps) and create a new app.
4. Create new static secret, the value must be a valid ed25519 secret key, you can generate a random key using https://cyphr.me/ed25519_tool/ed.html

### Setup

Create .env file with following entries

```bash
HASHICORP_CLIENT_ID=L5...Xa
HASHICORP_CLIENT_SECRET=Q9...2P
HASHICORP_ORG_ID=1b345678-b123-a123-c123-1b345678 # in org settings
HASHICORP_PROJECT_ID=1b345678-b123-a123-c123-1b345678 # in project settings
HASHICORP_APP_NAME=your_app_name
```