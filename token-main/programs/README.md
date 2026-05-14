# LetterP Programs

These crates are minimal Solana entrypoint surfaces for the generated devnet program IDs:

- `p_agent`
- `p_token`
- `x402_gateway`
- `bonding_curve`
- `perpetuals`

They compile as `cdylib` crates and exercise the SDK policy/math primitives. They are intentionally small until the final audited instruction sets are defined.

Build all host crates:

```bash
cargo check --workspace
```

Build SBF artifacts before any deployment:

```bash
cargo build-sbf --manifest-path programs/p_agent/Cargo.toml
```
