## Signer

A multi-chain keystore library that provides a unified interface for managing private keys.

> Warning: this is an experimental project under development.

## Features

- **Multiple Key Sources**
  - Local keystore management (JSON keystore files)
  - HashiCorp Vault integration (both local on-prem vaults and cloud)
  
- **Chain Support**
  - Ethereum (EVM) compatible chains
  - Stellar/Soroban
  
- **Key Operations**
  - Generate new keys
  - Load existing keys
  - Secure key storage and retrieval

## Examples

Check the `examples/` directory for complete usage examples:
- `keystore-to-alloy-wallet`: Convert keystore to EVM wallet
- `local-keystore-to-stellar-wallet`: Convert keystore to Stellar wallet

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