use {
    core::str::from_utf8,
    pinocchio::{
        entrypoint::{InstructionContext, MaybeAccount},
        program_error::ProgramError,
        ProgramResult,
    },
    pinocchio_log::log,
};

const BASE58_PUBKEY_BUFFER_LEN: usize = 44;

pub fn process_instruction(mut context: InstructionContext) -> ProgramResult {
    log_signers(&mut context)?;

    // SAFETY: all account metas have been consumed before reading instruction data.
    let memo = unsafe { context.instruction_data_unchecked() };
    let memo = from_utf8(memo).map_err(|error| {
        log!(1300, "Invalid UTF-8, from byte {}", error.valid_up_to());
        ProgramError::InvalidInstructionData
    })?;

    log!(1300, "Memo (len {}): \"{}\"", memo.len(), memo);
    Ok(())
}

fn log_signers(context: &mut InstructionContext) -> ProgramResult {
    let mut encoded_key = [0u8; BASE58_PUBKEY_BUFFER_LEN];
    let mut missing_required_signature = false;

    while context.remaining() > 0 {
        if let MaybeAccount::Account(account) = context.next_account()? {
            if account.is_signer() {
                let encoded_len = five8::encode_32(account.key(), &mut encoded_key) as usize;
                let signer = from_utf8(&encoded_key[..encoded_len])
                    .map_err(|_| ProgramError::InvalidAccountData)?;
                log!("Signed by {}", signer);
            } else {
                missing_required_signature = true;
            }
        }
    }

    if missing_required_signature {
        Err(ProgramError::MissingRequiredSignature)
    } else {
        Ok(())
    }
}
