# Migration Guide: SPL Token → Token-2022

Step-by-step guide for migrating existing SPL Token programs to Token-2022.

## Key Differences
1. **Program ID** — Update all hardcoded Token program ID references
2. **Account size** — Token-2022 accounts require extra space for extensions
3. **ATA derivation** — Include the token program ID in ATA seed derivation
4. **Multisig** — Token-2022 uses a different multisig account layout
5. **Extension init order** — Extensions must be initialized before the mint

## Migration Checklist
- [ ] Update TOKEN_PROGRAM_ID references
- [ ] Recalculate account space requirements
- [ ] Update ATA derivation logic
- [ ] Add extension initialization instructions
- [ ] Update client-side account deserialization
- [ ] Test with Token-2022 devnet program

> 🚧 Coming Soon
