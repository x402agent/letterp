# P Agent Token — Mainnet Deploy Guide

> First Pinocchio agent program on Solana.

## Prerequisites

```bash
# Solana CLI 1.18+
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Rust with SBF target
rustup target add sbf-solana-solana

# Solana toolchain matching Pinocchio workspace
rustup toolchain install 1.89.0
rustup override set 1.89.0   # inside p-agent-token/
```

## Program ID

Before deploying, generate a vanity keypair and update `src/lib.rs`:

```bash
# Find a keypair starting with "pAGNT"
solana-keygen grind --starts-with pAGNT:1 --ignore-case

# Save to keys/program-keypair.json (do NOT commit)
mkdir -p keys
mv <generated>.json keys/program-keypair.json

# Update the declare_id! in src/lib.rs with the generated address
solana-keygen pubkey keys/program-keypair.json
```

Add `keys/` to `.gitignore`.

## Build

```bash
cd p-agent-token

# Verify it compiles
cargo check

# Build for SBF
cargo build-sbf

# The .so lands at:
ls target/deploy/p_agent_token.so
```

## Devnet Test Deploy

```bash
# Fund your deploy wallet
solana airdrop 5 --url devnet

# Deploy
solana program deploy \
  --program-id keys/program-keypair.json \
  --keypair ~/.config/solana/id.json \
  --url devnet \
  target/deploy/p_agent_token.so

# Note the program ID printed — update env:
export P_TOKEN_LAUNCHPAD_PROGRAM_ID=<program-id>
```

Run the TypeScript integration tests against devnet:

```bash
cd examples
SOLANA_NETWORK=solana-devnet npx ts-node src/p-agent-full.ts
```

## Mainnet Deploy

```bash
# Fund deploy wallet (need ~10-15 SOL for program rent)
# Current cost: ~0.003 SOL/KB of binary

solana program deploy \
  --program-id keys/program-keypair.json \
  --keypair ~/.config/solana/id.json \
  --url mainnet-beta \
  --with-compute-unit-price 1000 \
  target/deploy/p_agent_token.so

# Verify deployment
solana program show <program-id> --url mainnet-beta
```

## Post-Deploy: Set Upgrade Authority

After confirming the program works correctly, transfer upgrade authority to a multisig or burn it for immutability:

```bash
# Transfer to a multisig (recommended)
solana program set-upgrade-authority <program-id> \
  --new-upgrade-authority <multisig-address> \
  --keypair ~/.config/solana/id.json

# OR: make immutable (irreversible — program can never be upgraded)
solana program set-upgrade-authority <program-id> \
  --final \
  --keypair ~/.config/solana/id.json
```

## Launch the First P Agent

Once deployed, update your environment and run the full flow:

```bash
export P_TOKEN_LAUNCHPAD_PROGRAM_ID=<your-deployed-program-id>
export P_TOKEN_PROGRAM_ID=ptok6rngomXrDbWf5v5Mkmu5CEbB51hzSCPDoj9DrvF
export HELIUS_API_KEY=<your-key>
export SOLANA_NETWORK=solana-mainnet
export USE_P_TOKEN=1

# Start the launchpad API
cd launchpad && npm start &

# Launch the first P Agent
cd examples && npx ts-node src/p-agent-full.ts
```

## Security Checklist Before Mainnet

- [ ] `cargo check` passes with no warnings
- [ ] All PDA bumps are stored in state and re-verified on every instruction
- [ ] All signer bits checked before any write (`account.is_signer()`)
- [ ] All writable bits checked before any mutation (`account.is_writable()`)
- [ ] `bind_agent_token` is provably one-way (discriminant == 2 blocks reinit)
- [ ] Curve math uses only `checked_*` arithmetic — no panics possible
- [ ] Graduation threshold: 85 SOL real reserves (`85_000_000_000` lamports)
- [ ] Creator fee route: accumulates in `creator_vault` PDA, claimed separately
- [ ] No private keys stored on-chain
- [ ] Token mint authority is the vault PDA — no human can mint outside the curve
- [ ] Devnet integration test passes end-to-end
- [ ] Binary size verified: `ls -lh target/deploy/p_agent_token.so`
- [ ] Program ID updated in `src/lib.rs` to match deploy keypair
- [ ] Upgrade authority transferred or burned

## Program Addresses

| Item | Value |
|------|-------|
| Program ID | `pAGNT...` (set before deploy) |
| P-Token Program | `ptok6rngomXrDbWf5v5Mkmu5CEbB51hzSCPDoj9DrvF` |
| MPL Core | `CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d` |
| SPL Token | `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA` |
| Raydium CPMM | `CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C` |

## PDA Seeds (on-chain, must match TypeScript)

| Account | Seeds |
|---------|-------|
| Agent | `["agent", owner_pubkey]` |
| AgentToken | `["agent-token", mint_pubkey]` |
| BondingCurve | `["bonding-curve", mint_pubkey]` |
| Vault | `["bonding-curve", mint_pubkey, "vault"]` |
| CreatorVault | `["creator-vault", creator_pubkey]` |
| ExecDelegation | `["exec-delegation", agent_pda, delegate_pubkey]` |
