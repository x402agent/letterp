//! Instruction discriminators and data layout for the bonding-curve program.

use pinocchio::pubkey::Pubkey;
use pinocchio::program_error::ProgramError;

/// Instruction discriminators.
pub enum BondingCurveInstruction {
    /// Initialize a new bonding curve.
    ///   Accounts (8):
    ///     [w] payer (signer)
    ///     [w] curve PDA
    ///     [w] vault PDA
    ///     [w] mint
    ///     [ ] creator_fee_wallet
    ///     [ ] token_program
    ///     [ ] system_program
    ///   Data: [u8 disc, u64 total_supply, u64 v_sol, u64 v_tok, u16 creator_bps, u16 proto_bps]
    InitializeCurve {
        total_supply: u64,
        virtual_sol_reserves: u64,
        virtual_token_reserves: u64,
        creator_fee_bps: u16,
        protocol_fee_bps: u16,
    },

    /// Buy tokens from the curve.
    ///   Accounts (8):
    ///     [w] buyer (signer)
    ///     [w] buyer ATA
    ///     [w] curve PDA
    ///     [w] vault PDA
    ///     [w] mint
    ///     [ ] token_program
    ///     [ ] ata_program
    ///     [ ] system_program
    ///   Data: [u8 disc, u64 sol_in, u64 min_tokens_out]
    Buy {
        sol_in: u64,
        min_tokens_out: u64,
    },

    /// Sell tokens back to the curve.
    ///   Accounts (7):
    ///     [w] seller (signer)
    ///     [w] seller ATA
    ///     [w] curve PDA
    ///     [w] vault PDA
    ///     [w] mint
    ///     [ ] token_program
    ///     [ ] system_program
    ///   Data: [u8 disc, u64 tokens_in, u64 min_sol_out]
    Sell {
        tokens_in: u64,
        min_sol_out: u64,
    },

    /// Graduate the curve (migrate liquidity to Raydium etc).
    ///   Accounts (7):
    ///     [w] cranker (signer)
    ///     [w] curve PDA
    ///     [w] vault PDA
    ///     [w] mint
    ///     [w] target_pool
    ///     [ ] token_program
    ///     [ ] system_program
    ///   Data: [u8 disc]
    Graduate,

    /// Claim accumulated creator fees.
    ///   Accounts (4):
    ///     [w] recipient (signer)
    ///     [w] curve PDA
    ///     [w] vault PDA
    ///     [ ] system_program
    ///   Data: [u8 disc]
    ClaimCreatorFees,
}

impl BondingCurveInstruction {
    /// Unpack a raw instruction buffer into the typed enum.
    pub fn unpack(data: &[u8]) -> Result<Self, ProgramError> {
        let (&disc, rest) = data
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match disc {
            0 => {
                let (total_supply, rest) = Self::unpack_u64(rest)?;
                let (virtual_sol_reserves, rest) = Self::unpack_u64(rest)?;
                let (virtual_token_reserves, rest) = Self::unpack_u64(rest)?;
                let (creator_fee_bps, rest) = Self::unpack_u16(rest)?;
                let (protocol_fee_bps, _) = Self::unpack_u16(rest)?;
                Self::InitializeCurve {
                    total_supply,
                    virtual_sol_reserves,
                    virtual_token_reserves,
                    creator_fee_bps,
                    protocol_fee_bps,
                }
            }
            1 => {
                let (sol_in, rest) = Self::unpack_u64(rest)?;
                let (min_tokens_out, _) = Self::unpack_u64(rest)?;
                Self::Buy {
                    sol_in,
                    min_tokens_out,
                }
            }
            2 => {
                let (tokens_in, rest) = Self::unpack_u64(rest)?;
                let (min_sol_out, _) = Self::unpack_u64(rest)?;
                Self::Sell {
                    tokens_in,
                    min_sol_out,
                }
            }
            3 => Self::Graduate,
            4 => Self::ClaimCreatorFees,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }

    fn unpack_u64(data: &[u8]) -> Result<(u64, &[u8]), ProgramError> {
        if data.len() < 8 {
            return Err(ProgramError::InvalidInstructionData);
        }
        let (bytes, rest) = data.split_at(8);
        Ok((u64::from_le_bytes(bytes.try_into().unwrap()), rest))
    }

    fn unpack_u16(data: &[u8]) -> Result<(u16, &[u8]), ProgramError> {
        if data.len() < 2 {
            return Err(ProgramError::InvalidInstructionData);
        }
        let (bytes, rest) = data.split_at(2);
        Ok((u16::from_le_bytes(bytes.try_into().unwrap()), rest))
    }
}
