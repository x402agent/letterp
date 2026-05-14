//! Thin wrappers around Solana syscalls.

use solana_program::{clock::Clock, msg, program_error::ProgramError, sysvar::Sysvar};

/// Log a message to the Solana program log.
pub fn log(message: &str) {
    msg!("{}", message);
}

/// Log current compute units (for profiling).
pub fn log_compute_units() {
    solana_program::log::sol_log_compute_units();
}

/// Get the current Solana cluster clock (slot, epoch, unix_timestamp).
pub fn get_clock() -> Result<Clock, ProgramError> {
    Clock::get()
}

/// Get the current Unix timestamp from the cluster clock.
pub fn unix_timestamp() -> Result<i64, ProgramError> {
    Ok(Clock::get()?.unix_timestamp)
}

/// Get the current slot.
pub fn current_slot() -> Result<u64, ProgramError> {
    Ok(Clock::get()?.slot)
}

/// Get the current epoch.
pub fn current_epoch() -> Result<u64, ProgramError> {
    Ok(Clock::get()?.epoch)
}
