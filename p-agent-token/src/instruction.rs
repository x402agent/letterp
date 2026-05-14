//! Instruction definitions and deserialization for P Agent Token.

extern crate alloc;
use alloc::vec::Vec;

use pinocchio::error::ProgramError;

use crate::error::AgentError;

/// All instructions understood by the P Agent Token program.
#[allow(dead_code)]
pub enum PAgentInstruction {
    /// 0 — Initialize agent state PDA
    ///
    /// Accounts:
    ///   0. [w,s] owner
    ///   1. [w]   agent_pda
    ///   2. []    system_program
    ///
    /// Data layout: [u8 disc=0, u8 bump, [u8;32] core_asset, u8 uri_len, ...uri]
    InitializeAgent {
        bump:       u8,
        core_asset: [u8; 32],
        uri:        Vec<u8>,
    },

    /// 1 — Initialize agent mint and bonding curve accounts
    ///
    /// Accounts:
    ///   0. [w,s] owner
    ///   1. [w]   agent_pda
    ///   2. [w,s] mint
    ///   3. [w]   curve_pda
    ///   4. [w]   vault_pda
    ///   5. [w]   creator_vault_pda
    ///   6. []    token_program
    ///   7. []    system_program
    ///
    /// Data layout: [u8 disc=1, u8 decimals, u64 total_supply, u64 v_sol,
    ///               u64 v_tok, u16 creator_bps, u16 proto_bps,
    ///               u8 curve_bump, u8 vault_bump]
    InitializeAgentMint {
        decimals:        u8,
        total_supply:    u64,
        virtual_sol:     u64,
        virtual_token:   u64,
        creator_fee_bps: u16,
        protocol_fee_bps: u16,
        curve_bump:      u8,
        vault_bump:      u8,
    },

    /// 2 — Permanently bind token to agent (irreversible)
    ///
    /// Accounts:
    ///   0. [w,s] owner
    ///   1. [w]   agent_pda
    ///   2. []    mint
    ///
    /// Data layout: [u8 disc=2]
    BindAgentToken,

    /// 3 — Set executive delegate
    ///
    /// Accounts:
    ///   0. [w,s] owner
    ///   1. [w]   agent_pda
    ///
    /// Data layout: [u8 disc=3, [u8;32] delegate]
    DelegateExecutor { delegate: [u8; 32] },

    /// 4 — Buy tokens from bonding curve
    ///
    /// Accounts:
    ///   0. [w,s] buyer
    ///   1. [w]   buyer_ata
    ///   2. [w]   curve_pda
    ///   3. [w]   vault_pda
    ///   4. [w]   mint
    ///   5. [w]   creator_vault_pda
    ///   6. []    token_program
    ///   7. []    system_program
    ///
    /// Data layout: [u8 disc=4, u64 sol_in, u64 min_tokens_out]
    Buy {
        sol_in:         u64,
        min_tokens_out: u64,
    },

    /// 5 — Sell tokens back to curve
    ///
    /// Accounts:
    ///   0. [w,s] seller
    ///   1. [w]   seller_ata
    ///   2. [w]   curve_pda
    ///   3. [w]   vault_pda
    ///   4. [w]   mint
    ///   5. [w]   creator_vault_pda
    ///   6. []    token_program
    ///   7. []    system_program
    ///
    /// Data layout: [u8 disc=5, u64 tokens_in, u64 min_sol_out]
    Sell {
        tokens_in:   u64,
        min_sol_out: u64,
    },

    /// 6 — Graduate bonding curve to DEX
    ///
    /// Accounts:
    ///   0. [w,s] cranker
    ///   1. [w]   curve_pda
    ///   2. [w]   vault_pda
    ///   3. [w]   mint
    ///   4. [w]   dex_pool
    ///   5. []    token_program
    ///   6. []    system_program
    ///
    /// Data layout: [u8 disc=6]
    Graduate,
}

impl PAgentInstruction {
    /// Deserialize an instruction from raw bytes.
    pub fn unpack(data: &[u8]) -> Result<Self, ProgramError> {
        let (&disc, rest) = data.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        match disc {
            0 => {
                // bump (1) + core_asset (32) + uri_len (1) + uri
                if rest.len() < 34 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let bump = rest[0];
                let mut core_asset = [0u8; 32];
                core_asset.copy_from_slice(&rest[1..33]);
                let uri_len = rest[33] as usize;
                if rest.len() < 34 + uri_len {
                    return Err(ProgramError::InvalidInstructionData);
                }
                if uri_len > 200 {
                    return Err(AgentError::UriTooLong.into());
                }
                let uri = rest[34..34 + uri_len].to_vec();
                Ok(PAgentInstruction::InitializeAgent { bump, core_asset, uri })
            }

            1 => {
                // decimals(1) + total_supply(8) + v_sol(8) + v_tok(8)
                // + creator_bps(2) + proto_bps(2) + curve_bump(1) + vault_bump(1) = 31
                if rest.len() < 31 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let decimals = rest[0];
                let total_supply    = u64::from_le_bytes(rest[1..9].try_into().unwrap());
                let virtual_sol     = u64::from_le_bytes(rest[9..17].try_into().unwrap());
                let virtual_token   = u64::from_le_bytes(rest[17..25].try_into().unwrap());
                let creator_fee_bps = u16::from_le_bytes(rest[25..27].try_into().unwrap());
                let protocol_fee_bps= u16::from_le_bytes(rest[27..29].try_into().unwrap());
                let curve_bump      = rest[29];
                let vault_bump      = rest[30];
                Ok(PAgentInstruction::InitializeAgentMint {
                    decimals,
                    total_supply,
                    virtual_sol,
                    virtual_token,
                    creator_fee_bps,
                    protocol_fee_bps,
                    curve_bump,
                    vault_bump,
                })
            }

            2 => Ok(PAgentInstruction::BindAgentToken),

            3 => {
                if rest.len() < 32 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let mut delegate = [0u8; 32];
                delegate.copy_from_slice(&rest[..32]);
                Ok(PAgentInstruction::DelegateExecutor { delegate })
            }

            4 => {
                if rest.len() < 16 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let sol_in         = u64::from_le_bytes(rest[0..8].try_into().unwrap());
                let min_tokens_out = u64::from_le_bytes(rest[8..16].try_into().unwrap());
                Ok(PAgentInstruction::Buy { sol_in, min_tokens_out })
            }

            5 => {
                if rest.len() < 16 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let tokens_in   = u64::from_le_bytes(rest[0..8].try_into().unwrap());
                let min_sol_out = u64::from_le_bytes(rest[8..16].try_into().unwrap());
                Ok(PAgentInstruction::Sell { tokens_in, min_sol_out })
            }

            6 => Ok(PAgentInstruction::Graduate),

            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
