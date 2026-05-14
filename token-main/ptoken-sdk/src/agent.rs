//! Policy primitives for LetterP autonomous agents.
//!
//! These types keep agent permissions explicit before a program or off-chain
//! runner signs token, curve, or perpetual instructions on behalf of a user.

use crate::{errors::PTokenError, PTokenResult};

/// Permission bits that can be granted to an agent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AgentCapabilityFlags(pub u32);

impl AgentCapabilityFlags {
    /// No agent permissions.
    pub const NONE: Self = Self(0);
    /// Agent can request x402 payment settlement.
    pub const X402_SETTLEMENT: Self = Self(1 << 0);
    /// Agent can quote or execute bonding-curve orders.
    pub const BONDING_CURVE_TRADING: Self = Self(1 << 1);
    /// Agent can quote or execute perpetual orders.
    pub const PERPETUAL_TRADING: Self = Self(1 << 2);
    /// Agent can manage Token-2022 extension workflows.
    pub const TOKEN_EXTENSION_ADMIN: Self = Self(1 << 3);

    /// Returns true when all bits in `required` are present.
    pub fn contains(self, required: Self) -> bool {
        self.0 & required.0 == required.0
    }

    /// Returns a flag set with both permissions.
    pub fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

/// Agent execution policy stored or derived by LetterP programs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AgentPolicy {
    /// Agent public identifier.
    pub agent_id: [u8; 32],
    /// Owner or controller public key bytes.
    pub owner: [u8; 32],
    /// Granted capability bitset.
    pub capabilities: AgentCapabilityFlags,
    /// Maximum lamports the agent can put at risk in one instruction.
    pub spending_limit_lamports: u64,
    /// Maximum slippage, leverage, or risk guard in basis points.
    pub risk_limit_bps: u16,
}

impl AgentPolicy {
    /// Validates the policy against non-vacuous execution limits.
    pub fn validate(&self) -> PTokenResult<()> {
        if self.risk_limit_bps > 10_000 {
            return Err(PTokenError::InvalidInstructionData);
        }

        let can_trade = self
            .capabilities
            .contains(AgentCapabilityFlags::BONDING_CURVE_TRADING)
            || self
                .capabilities
                .contains(AgentCapabilityFlags::PERPETUAL_TRADING);

        if can_trade && self.spending_limit_lamports == 0 {
            return Err(PTokenError::InvalidInstructionData);
        }

        Ok(())
    }

    /// Returns true when the policy grants `required` and passes validation.
    pub fn permits(&self, required: AgentCapabilityFlags) -> bool {
        self.validate().is_ok() && self.capabilities.contains(required)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn policy_requires_spend_limit_for_trading() {
        let policy = AgentPolicy {
            agent_id: [1; 32],
            owner: [2; 32],
            capabilities: AgentCapabilityFlags::BONDING_CURVE_TRADING,
            spending_limit_lamports: 0,
            risk_limit_bps: 100,
        };

        assert!(policy.validate().is_err());
    }
}
