//! x402 Bonding-Curve Program
//!
//! A constant-product bonding-curve program written with Pinocchio
//! (no Anchor dependency). Supports:
//!
//!   - InitializeCurve  (discriminator 0)
//!   - Buy              (discriminator 1)
//!   - Sell             (discriminator 2)
//!   - Graduate         (discriminator 3)
//!   - ClaimCreatorFees (discriminator 4)
//!
//! Uses p-token (SIMD-0266) for the mint. After the feature gate activates,
//! every token instruction is ~95% cheaper CU-wise.

pub mod instruction;
pub mod processor;
pub mod state;
pub mod error;

// Re-export for the entrypoint macro.
pub use pinocchio::entrypoint;
