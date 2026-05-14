# Confidential Transfer Extension

Hide transfer amounts using ElGamal encryption and zero-knowledge proofs.
Balances are stored as ciphertexts; amounts are never revealed on-chain.

## Key Concepts
- ElGamal keypair per token account
- ZK proof of valid transfer range (Sigma protocols)
- `ApplyPendingBalance` merges incoming amounts
- `Withdraw` decrypts and moves to visible balance

## Requirements
- Account must configure an ElGamal public key
- Proofs are submitted as account data (large accounts)

> 🚧 Coming Soon
