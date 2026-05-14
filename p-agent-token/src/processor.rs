//! Instruction processor for the P Agent Token program.

use pinocchio::{
    cpi::Signer,
    error::ProgramError,
    sysvars::{rent::Rent, Sysvar},
    AccountView, Address, ProgramResult,
};
use pinocchio::cpi::Seed;
use pinocchio_system::instructions::{CreateAccount, Transfer as SolTransfer};
use pinocchio_token::instructions::{Burn, InitializeMint2, MintTo};

use crate::{
    error::AgentError,
    instruction::PAgentInstruction,
    pdas::*,
    state::{AgentState, CurveState},
};

/// SOL required in vault before curve is eligible for graduation.
pub const GRADUATION_THRESHOLD_LAMPORTS: u64 = 85_000_000_000;

// ---------------------------------------------------------------------------
// Constant-product math helpers
// ---------------------------------------------------------------------------

/// Quote a buy: `sol_in_gross` SOL enters the curve.
///
/// Returns `(tokens_out, creator_fee_lamports, protocol_fee_lamports)`.
///
/// Formula:
///   net_sol   = sol_in_gross - creator_fee - protocol_fee
///   new_v_sol = v_sol + net_sol
///   tokens_out = v_tok - (v_sol * v_tok) / new_v_sol
fn quote_buy(
    v_sol: u64,
    v_tok: u64,
    sol_in_gross: u64,
    creator_bps: u16,
    protocol_bps: u16,
) -> Result<(u64, u64, u64), ProgramError> {
    let creator_fee = fee_amount(sol_in_gross, creator_bps)?;
    let protocol_fee = fee_amount(sol_in_gross, protocol_bps)?;
    let total_fee = creator_fee
        .checked_add(protocol_fee)
        .ok_or(AgentError::ArithmeticError)?;
    let net_sol = sol_in_gross
        .checked_sub(total_fee)
        .ok_or(AgentError::ArithmeticError)?;

    let new_v_sol = v_sol
        .checked_add(net_sol)
        .ok_or(AgentError::ArithmeticError)?;

    // k = v_sol * v_tok (use u128 to avoid overflow)
    let k = (v_sol as u128)
        .checked_mul(v_tok as u128)
        .ok_or(AgentError::ArithmeticError)?;

    // new_v_tok = k / new_v_sol
    let new_v_tok = k
        .checked_div(new_v_sol as u128)
        .ok_or(AgentError::ArithmeticError)? as u64;

    let tokens_out = v_tok
        .checked_sub(new_v_tok)
        .ok_or(AgentError::ArithmeticError)?;

    Ok((tokens_out, creator_fee, protocol_fee))
}

/// Quote a sell: `tokens_in` tokens enter the curve.
///
/// Returns `(sol_out_net, creator_fee_lamports, protocol_fee_lamports)`.
///
/// Formula:
///   new_v_tok     = v_tok + tokens_in
///   sol_out_gross = v_sol - (v_sol * v_tok) / new_v_tok
///   sol_out_net   = sol_out_gross - creator_fee - protocol_fee
fn quote_sell(
    v_sol: u64,
    v_tok: u64,
    tokens_in: u64,
    creator_bps: u16,
    protocol_bps: u16,
) -> Result<(u64, u64, u64), ProgramError> {
    let new_v_tok = v_tok
        .checked_add(tokens_in)
        .ok_or(AgentError::ArithmeticError)?;

    let k = (v_sol as u128)
        .checked_mul(v_tok as u128)
        .ok_or(AgentError::ArithmeticError)?;

    let new_v_sol = k
        .checked_div(new_v_tok as u128)
        .ok_or(AgentError::ArithmeticError)? as u64;

    let sol_out_gross = v_sol
        .checked_sub(new_v_sol)
        .ok_or(AgentError::ArithmeticError)?;

    let creator_fee = fee_amount(sol_out_gross, creator_bps)?;
    let protocol_fee = fee_amount(sol_out_gross, protocol_bps)?;
    let total_fee = creator_fee
        .checked_add(protocol_fee)
        .ok_or(AgentError::ArithmeticError)?;
    let sol_out_net = sol_out_gross
        .checked_sub(total_fee)
        .ok_or(AgentError::ArithmeticError)?;

    Ok((sol_out_net, creator_fee, protocol_fee))
}

/// Compute `amount * bps / 10_000` using u128 to avoid overflow.
#[inline(always)]
fn fee_amount(amount: u64, bps: u16) -> Result<u64, ProgramError> {
    if bps == 0 {
        return Ok(0);
    }
    let fee = (amount as u128)
        .checked_mul(bps as u128)
        .ok_or(AgentError::ArithmeticError)?
        .checked_div(10_000)
        .ok_or(AgentError::ArithmeticError)? as u64;
    Ok(fee)
}

// ---------------------------------------------------------------------------
// Entrypoint dispatcher
// ---------------------------------------------------------------------------

pub fn process_instruction(
    program_id: &Address,
    accounts: &mut [AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = PAgentInstruction::unpack(instruction_data)?;

    match instruction {
        PAgentInstruction::InitializeAgent { bump, core_asset, uri } => {
            process_initialize_agent(program_id, accounts, bump, core_asset, &uri)
        }
        PAgentInstruction::InitializeAgentMint {
            decimals,
            total_supply,
            virtual_sol,
            virtual_token,
            creator_fee_bps,
            protocol_fee_bps,
            curve_bump,
            vault_bump,
        } => process_initialize_agent_mint(
            program_id,
            accounts,
            decimals,
            total_supply,
            virtual_sol,
            virtual_token,
            creator_fee_bps,
            protocol_fee_bps,
            curve_bump,
            vault_bump,
        ),
        PAgentInstruction::BindAgentToken => {
            process_bind_agent_token(program_id, accounts)
        }
        PAgentInstruction::DelegateExecutor { delegate } => {
            process_delegate_executor(accounts, &delegate)
        }
        PAgentInstruction::Buy { sol_in, min_tokens_out } => {
            process_buy(program_id, accounts, sol_in, min_tokens_out)
        }
        PAgentInstruction::Sell { tokens_in, min_sol_out } => {
            process_sell(program_id, accounts, tokens_in, min_sol_out)
        }
        PAgentInstruction::Graduate => {
            process_graduate(program_id, accounts)
        }
    }
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// Instruction 0 — InitializeAgent
///
/// Accounts: [w,s] owner | [w] agent_pda | [] system_program
fn process_initialize_agent(
    program_id: &Address,
    accounts: &mut [AccountView],
    bump: u8,
    core_asset: [u8; 32],
    _uri: &[u8],
) -> ProgramResult {
    let [owner, agent_pda, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // --- Security checks ---
    if !owner.is_signer() {
        return Err(AgentError::NotSigner.into());
    }
    if !agent_pda.is_writable() {
        return Err(AgentError::NotWritable.into());
    }

    let owner_key = *owner.address();

    // Verify agent PDA address.
    verify_pda(
        agent_pda.address().as_ref(),
        &agent_seeds(owner_key.as_ref()),
        program_id,
        bump,
    )?;

    // Allocate the agent PDA via system program.
    let rent = Rent::get()?;
    let lamports = rent.try_minimum_balance(AgentState::LEN)?;

    let bump_arr = [bump];
    let agent_signer_seeds = [
        Seed::from(b"agent" as &[u8]),
        Seed::from(owner_key.as_ref()),
        Seed::from(bump_arr.as_ref()),
    ];
    let signers: &[Signer] = &[Signer::from(&agent_signer_seeds)];

    CreateAccount {
        from: owner,
        to: agent_pda,
        lamports,
        space: AgentState::LEN as u64,
        owner: program_id,
    }
    .invoke_signed(signers)?;

    // Write initial state.
    let mut data = agent_pda.try_borrow_mut()?;
    let state = AgentState::from_bytes_mut(&mut *data)
        .ok_or(ProgramError::AccountDataTooSmall)?;

    // Guard: must be uninitialized.
    if state.discriminant != 0 {
        return Err(AgentError::AlreadyInitialized.into());
    }

    state.discriminant = 1;
    state.owner.copy_from_slice(owner_key.as_ref());
    state.core_asset = core_asset;
    state.bump = bump;
    // mint, executive, creator_vault, created_at stay zero — filled in later.

    Ok(())
}

/// Instruction 1 — InitializeAgentMint
///
/// Accounts: [w,s] owner | [w] agent_pda | [w,s] mint | [w] curve_pda |
///           [w] vault_pda | [w] creator_vault_pda | [] token_program |
///           [] system_program
#[allow(clippy::too_many_arguments)]
fn process_initialize_agent_mint(
    program_id: &Address,
    accounts: &mut [AccountView],
    decimals: u8,
    _total_supply: u64,
    virtual_sol: u64,
    virtual_token: u64,
    creator_fee_bps: u16,
    protocol_fee_bps: u16,
    curve_bump: u8,
    vault_bump: u8,
) -> ProgramResult {
    let [owner, agent_pda, mint, curve_pda, vault_pda, creator_vault_pda, _token_program, _system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !owner.is_signer() {
        return Err(AgentError::NotSigner.into());
    }
    if !agent_pda.is_writable()
        || !mint.is_writable()
        || !curve_pda.is_writable()
        || !vault_pda.is_writable()
    {
        return Err(AgentError::NotWritable.into());
    }

    let owner_key = *owner.address();
    let mint_key = *mint.address();
    let creator_vault_key = *creator_vault_pda.address();

    // Verify agent PDA is initialized and owner matches.
    let agent_bump = {
        let data = agent_pda.try_borrow()?;
        let agent = AgentState::from_bytes(&*data)
            .ok_or(ProgramError::AccountDataTooSmall)?;
        if agent.discriminant == 0 {
            return Err(AgentError::NotInitialized.into());
        }
        if agent.owner.as_ref() != owner_key.as_ref() {
            return Err(AgentError::InvalidAuthority.into());
        }
        agent.bump
    };

    verify_pda(
        agent_pda.address().as_ref(),
        &agent_seeds(owner_key.as_ref()),
        program_id,
        agent_bump,
    )?;

    // Verify curve PDA.
    verify_pda(
        curve_pda.address().as_ref(),
        &curve_seeds(mint_key.as_ref()),
        program_id,
        curve_bump,
    )?;

    // Verify vault PDA.
    verify_pda(
        vault_pda.address().as_ref(),
        &vault_seeds(mint_key.as_ref()),
        program_id,
        vault_bump,
    )?;

    let rent = Rent::get()?;

    // --- Allocate curve PDA ---
    let curve_lamports = rent.try_minimum_balance(CurveState::LEN)?;
    {
        let bump_arr = [curve_bump];
        let curve_signer_seeds = [
            Seed::from(b"bonding-curve" as &[u8]),
            Seed::from(mint_key.as_ref()),
            Seed::from(bump_arr.as_ref()),
        ];
        let curve_signers: &[Signer] = &[Signer::from(&curve_signer_seeds)];
        CreateAccount {
            from: owner,
            to: curve_pda,
            lamports: curve_lamports,
            space: CurveState::LEN as u64,
            owner: program_id,
        }
        .invoke_signed(curve_signers)?;
    }

    // --- Allocate vault PDA (zero data, owned by system program initially) ---
    let vault_lamports = rent.try_minimum_balance(0)?;
    {
        let bump_arr = [vault_bump];
        let vault_signer_seeds = [
            Seed::from(b"bonding-curve" as &[u8]),
            Seed::from(mint_key.as_ref()),
            Seed::from(b"vault" as &[u8]),
            Seed::from(bump_arr.as_ref()),
        ];
        let vault_signers: &[Signer] = &[Signer::from(&vault_signer_seeds)];
        CreateAccount {
            from: owner,
            to: vault_pda,
            lamports: vault_lamports,
            space: 0,
            owner: &pinocchio_system::ID,
        }
        .invoke_signed(vault_signers)?;
    }

    // --- Initialize the SPL mint with curve_pda as mint authority ---
    {
        let curve_addr = *curve_pda.address();
        InitializeMint2 {
            mint,
            decimals,
            mint_authority: &curve_addr,
            freeze_authority: None,
        }
        .invoke()?;
    }

    // --- Write curve state ---
    {
        let mut data = curve_pda.try_borrow_mut()?;
        let curve = CurveState::from_bytes_mut(&mut *data)
            .ok_or(ProgramError::AccountDataTooSmall)?;

        curve.discriminant = 1;
        curve.mint.copy_from_slice(mint_key.as_ref());
        curve.creator_fee_wallet.copy_from_slice(creator_vault_key.as_ref());
        curve.virtual_sol_reserves = virtual_sol;
        curve.virtual_token_reserves = virtual_token;
        curve.real_sol_reserves = 0;
        curve.real_token_reserves = 0;
        curve.total_supply = _total_supply;
        curve.tokens_sold = 0;
        curve.creator_fee_bps = creator_fee_bps;
        curve.protocol_fee_bps = protocol_fee_bps;
        curve.graduated = 0;
        curve.curve_bump = curve_bump;
        curve.vault_bump = vault_bump;
    }

    // --- Update agent_pda with mint address ---
    {
        let mut data = agent_pda.try_borrow_mut()?;
        let agent = AgentState::from_bytes_mut(&mut *data)
            .ok_or(ProgramError::AccountDataTooSmall)?;
        agent.mint.copy_from_slice(mint_key.as_ref());
        agent.creator_vault.copy_from_slice(creator_vault_key.as_ref());
    }

    Ok(())
}

/// Instruction 2 — BindAgentToken (irreversible)
///
/// Accounts: [w,s] owner | [w] agent_pda | [] mint
fn process_bind_agent_token(
    program_id: &Address,
    accounts: &mut [AccountView],
) -> ProgramResult {
    let [owner, agent_pda, mint] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !owner.is_signer() {
        return Err(AgentError::NotSigner.into());
    }
    if !agent_pda.is_writable() {
        return Err(AgentError::NotWritable.into());
    }

    let owner_key = *owner.address();
    let mint_key = *mint.address();
    // Capture agent_pda address before mutably borrowing it.
    let agent_pda_key = *agent_pda.address();

    let mut data = agent_pda.try_borrow_mut()?;
    let agent = AgentState::from_bytes_mut(&mut *data)
        .ok_or(ProgramError::AccountDataTooSmall)?;

    // Must be in initialized (not yet bound) state.
    match agent.discriminant {
        0 => return Err(AgentError::NotInitialized.into()),
        2 => return Err(AgentError::AlreadyBound.into()),
        1 => {}
        _ => return Err(ProgramError::InvalidAccountData),
    }

    // Verify owner.
    if agent.owner.as_ref() != owner_key.as_ref() {
        return Err(AgentError::InvalidAuthority.into());
    }

    let agent_bump = agent.bump;
    let agent_mint = agent.mint;

    // Verify the PDA using the stored bump.
    verify_pda(
        agent_pda_key.as_ref(),
        &agent_seeds(owner_key.as_ref()),
        program_id,
        agent_bump,
    )?;

    // Verify the mint matches what was stored during InitializeAgentMint.
    if agent_mint.as_ref() != mint_key.as_ref() {
        return Err(AgentError::InvalidMint.into());
    }

    agent.discriminant = 2; // BOUND — irreversible

    Ok(())
}

/// Instruction 3 — DelegateExecutor
///
/// Accounts: [w,s] owner | [w] agent_pda
fn process_delegate_executor(
    accounts: &mut [AccountView],
    delegate: &[u8; 32],
) -> ProgramResult {
    let [owner, agent_pda] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !owner.is_signer() {
        return Err(AgentError::NotSigner.into());
    }
    if !agent_pda.is_writable() {
        return Err(AgentError::NotWritable.into());
    }

    let owner_key = *owner.address();

    let mut data = agent_pda.try_borrow_mut()?;
    let agent = AgentState::from_bytes_mut(&mut *data)
        .ok_or(ProgramError::AccountDataTooSmall)?;

    if agent.discriminant == 0 {
        return Err(AgentError::NotInitialized.into());
    }
    if agent.owner.as_ref() != owner_key.as_ref() {
        return Err(AgentError::InvalidAuthority.into());
    }

    agent.executive.copy_from_slice(delegate);

    Ok(())
}

/// Instruction 4 — Buy
///
/// Accounts: [w,s] buyer | [w] buyer_ata | [w] curve_pda | [w] vault_pda |
///           [w] mint | [w] creator_vault_pda | [] token_program |
///           [] system_program
fn process_buy(
    program_id: &Address,
    accounts: &mut [AccountView],
    sol_in: u64,
    min_tokens_out: u64,
) -> ProgramResult {
    let [buyer, buyer_ata, curve_pda, vault_pda, mint, creator_vault_pda, _token_program, _system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !buyer.is_signer() {
        return Err(AgentError::NotSigner.into());
    }

    let mint_key = *mint.address();

    // Read and validate curve state.
    let (v_sol, v_tok, creator_bps, protocol_bps, curve_bump, vault_bump) = {
        let data = curve_pda.try_borrow()?;
        let curve = CurveState::from_bytes(&*data)
            .ok_or(ProgramError::AccountDataTooSmall)?;
        if curve.discriminant != 1 {
            return Err(AgentError::NotInitialized.into());
        }
        if curve.graduated != 0 {
            return Err(AgentError::AlreadyGraduated.into());
        }
        // Verify the mint stored in the curve matches the account passed in.
        if curve.mint.as_ref() != mint_key.as_ref() {
            return Err(AgentError::InvalidMint.into());
        }
        (
            curve.virtual_sol_reserves,
            curve.virtual_token_reserves,
            curve.creator_fee_bps,
            curve.protocol_fee_bps,
            curve.curve_bump,
            curve.vault_bump,
        )
    };

    // Verify PDAs.
    verify_pda(curve_pda.address().as_ref(), &curve_seeds(mint_key.as_ref()), program_id, curve_bump)?;
    verify_pda(vault_pda.address().as_ref(), &vault_seeds(mint_key.as_ref()), program_id, vault_bump)?;

    // Compute tokens out and fees.
    let (tokens_out, creator_fee, protocol_fee) =
        quote_buy(v_sol, v_tok, sol_in, creator_bps, protocol_bps)?;

    if tokens_out < min_tokens_out {
        return Err(AgentError::SlippageExceeded.into());
    }

    // Transfer sol_in from buyer to vault (buyer is signer).
    SolTransfer {
        from: buyer,
        to: vault_pda,
        lamports: sol_in,
    }
    .invoke()?;

    // Transfer creator_fee from vault to creator_vault (vault PDA signer).
    let bump_arr = [vault_bump];
    let vault_signer_seeds = [
        Seed::from(b"bonding-curve" as &[u8]),
        Seed::from(mint_key.as_ref()),
        Seed::from(b"vault" as &[u8]),
        Seed::from(bump_arr.as_ref()),
    ];
    let vault_signers: &[Signer] = &[Signer::from(&vault_signer_seeds)];

    if creator_fee > 0 {
        SolTransfer {
            from: vault_pda,
            to: creator_vault_pda,
            lamports: creator_fee,
        }
        .invoke_signed(vault_signers)?;
    }

    // Mint tokens_out to buyer_ata (curve_pda is mint authority, PDA signer).
    {
        let bump_arr2 = [curve_bump];
        let curve_signer_seeds = [
            Seed::from(b"bonding-curve" as &[u8]),
            Seed::from(mint_key.as_ref()),
            Seed::from(bump_arr2.as_ref()),
        ];
        let curve_signers: &[Signer] = &[Signer::from(&curve_signer_seeds)];
        MintTo::new(mint, buyer_ata, curve_pda, tokens_out)
            .invoke_signed(curve_signers)?;
    }

    // Update curve state.
    {
        let net_sol = sol_in
            .checked_sub(creator_fee)
            .and_then(|x| x.checked_sub(protocol_fee))
            .ok_or(AgentError::ArithmeticError)?;

        let mut data = curve_pda.try_borrow_mut()?;
        let curve = CurveState::from_bytes_mut(&mut *data)
            .ok_or(ProgramError::AccountDataTooSmall)?;

        curve.virtual_sol_reserves = curve
            .virtual_sol_reserves
            .checked_add(net_sol)
            .ok_or(AgentError::ArithmeticError)?;
        curve.virtual_token_reserves = curve
            .virtual_token_reserves
            .checked_sub(tokens_out)
            .ok_or(AgentError::ArithmeticError)?;
        curve.real_sol_reserves = curve
            .real_sol_reserves
            .checked_add(net_sol)
            .ok_or(AgentError::ArithmeticError)?;
        curve.real_token_reserves = curve
            .real_token_reserves
            .checked_sub(tokens_out)
            .ok_or(AgentError::ArithmeticError)?;
        curve.tokens_sold = curve
            .tokens_sold
            .checked_add(tokens_out)
            .ok_or(AgentError::ArithmeticError)?;
    }

    Ok(())
}

/// Instruction 5 — Sell
///
/// Accounts: [w,s] seller | [w] seller_ata | [w] curve_pda | [w] vault_pda |
///           [w] mint | [w] creator_vault_pda | [] token_program |
///           [] system_program
fn process_sell(
    program_id: &Address,
    accounts: &mut [AccountView],
    tokens_in: u64,
    min_sol_out: u64,
) -> ProgramResult {
    let [seller, seller_ata, curve_pda, vault_pda, mint, creator_vault_pda, _token_program, _system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !seller.is_signer() {
        return Err(AgentError::NotSigner.into());
    }

    let mint_key = *mint.address();

    // Read and validate curve state.
    let (v_sol, v_tok, creator_bps, protocol_bps, curve_bump, vault_bump) = {
        let data = curve_pda.try_borrow()?;
        let curve = CurveState::from_bytes(&*data)
            .ok_or(ProgramError::AccountDataTooSmall)?;
        if curve.discriminant != 1 {
            return Err(AgentError::NotInitialized.into());
        }
        if curve.graduated != 0 {
            return Err(AgentError::AlreadyGraduated.into());
        }
        if curve.mint.as_ref() != mint_key.as_ref() {
            return Err(AgentError::InvalidMint.into());
        }
        (
            curve.virtual_sol_reserves,
            curve.virtual_token_reserves,
            curve.creator_fee_bps,
            curve.protocol_fee_bps,
            curve.curve_bump,
            curve.vault_bump,
        )
    };

    verify_pda(curve_pda.address().as_ref(), &curve_seeds(mint_key.as_ref()), program_id, curve_bump)?;
    verify_pda(vault_pda.address().as_ref(), &vault_seeds(mint_key.as_ref()), program_id, vault_bump)?;

    // Quote the sell.
    let (sol_out_net, creator_fee, protocol_fee) =
        quote_sell(v_sol, v_tok, tokens_in, creator_bps, protocol_bps)?;

    if sol_out_net < min_sol_out {
        return Err(AgentError::SlippageExceeded.into());
    }

    // Burn tokens from seller_ata (seller is the authority/signer).
    Burn::new(seller_ata, mint, seller, tokens_in).invoke()?;

    // Build vault signer seeds.
    let vault_bump_arr = [vault_bump];
    let vault_signer_seeds = [
        Seed::from(b"bonding-curve" as &[u8]),
        Seed::from(mint_key.as_ref()),
        Seed::from(b"vault" as &[u8]),
        Seed::from(vault_bump_arr.as_ref()),
    ];
    let vault_signers: &[Signer] = &[Signer::from(&vault_signer_seeds)];

    // Transfer sol_out_net from vault to seller (vault PDA is signer).
    SolTransfer {
        from: vault_pda,
        to: seller,
        lamports: sol_out_net,
    }
    .invoke_signed(vault_signers)?;

    // Transfer creator_fee from vault to creator_vault.
    if creator_fee > 0 {
        SolTransfer {
            from: vault_pda,
            to: creator_vault_pda,
            lamports: creator_fee,
        }
        .invoke_signed(vault_signers)?;
    }

    // Update curve state.
    {
        let total_out = sol_out_net
            .checked_add(creator_fee)
            .and_then(|x| x.checked_add(protocol_fee))
            .ok_or(AgentError::ArithmeticError)?;

        let mut data = curve_pda.try_borrow_mut()?;
        let curve = CurveState::from_bytes_mut(&mut *data)
            .ok_or(ProgramError::AccountDataTooSmall)?;

        curve.virtual_sol_reserves = curve
            .virtual_sol_reserves
            .checked_sub(total_out)
            .ok_or(AgentError::ArithmeticError)?;
        curve.virtual_token_reserves = curve
            .virtual_token_reserves
            .checked_add(tokens_in)
            .ok_or(AgentError::ArithmeticError)?;
        curve.real_sol_reserves = curve
            .real_sol_reserves
            .checked_sub(total_out)
            .ok_or(AgentError::ArithmeticError)?;
        curve.real_token_reserves = curve
            .real_token_reserves
            .checked_add(tokens_in)
            .ok_or(AgentError::ArithmeticError)?;
        curve.tokens_sold = curve
            .tokens_sold
            .checked_sub(tokens_in)
            .ok_or(AgentError::ArithmeticError)?;
    }

    Ok(())
}

/// Instruction 6 — Graduate
///
/// Accounts: [w,s] cranker | [w] curve_pda | [w] vault_pda |
///           [w] mint | [w] dex_pool | [] token_program | [] system_program
fn process_graduate(
    program_id: &Address,
    accounts: &mut [AccountView],
) -> ProgramResult {
    let [cranker, curve_pda, vault_pda, mint, dex_pool, _token_program, _system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !cranker.is_signer() {
        return Err(AgentError::NotSigner.into());
    }

    let mint_key = *mint.address();

    // Read curve state.
    let (curve_bump, vault_bump, real_sol, total_supply, tokens_sold) = {
        let data = curve_pda.try_borrow()?;
        let curve = CurveState::from_bytes(&*data)
            .ok_or(ProgramError::AccountDataTooSmall)?;
        if curve.discriminant != 1 {
            return Err(AgentError::NotInitialized.into());
        }
        if curve.graduated != 0 {
            return Err(AgentError::AlreadyGraduated.into());
        }
        if curve.mint.as_ref() != mint_key.as_ref() {
            return Err(AgentError::InvalidMint.into());
        }
        (
            curve.curve_bump,
            curve.vault_bump,
            curve.real_sol_reserves,
            curve.total_supply,
            curve.tokens_sold,
        )
    };

    // Graduation threshold check.
    if real_sol < GRADUATION_THRESHOLD_LAMPORTS {
        return Err(AgentError::NotGraduatable.into());
    }

    verify_pda(curve_pda.address().as_ref(), &curve_seeds(mint_key.as_ref()), program_id, curve_bump)?;
    verify_pda(vault_pda.address().as_ref(), &vault_seeds(mint_key.as_ref()), program_id, vault_bump)?;

    // Mark graduated.
    {
        let mut data = curve_pda.try_borrow_mut()?;
        let curve = CurveState::from_bytes_mut(&mut *data)
            .ok_or(ProgramError::AccountDataTooSmall)?;
        curve.graduated = 1;
    }

    // Transfer all SOL from vault to dex_pool (vault PDA signer).
    let vault_balance = vault_pda.lamports();
    if vault_balance > 0 {
        let vault_bump_arr = [vault_bump];
        let vault_signer_seeds = [
            Seed::from(b"bonding-curve" as &[u8]),
            Seed::from(mint_key.as_ref()),
            Seed::from(b"vault" as &[u8]),
            Seed::from(vault_bump_arr.as_ref()),
        ];
        let vault_signers: &[Signer] = &[Signer::from(&vault_signer_seeds)];
        SolTransfer {
            from: vault_pda,
            to: dex_pool,
            lamports: vault_balance,
        }
        .invoke_signed(vault_signers)?;
    }

    // Mint remaining unsold tokens to dex_pool (curve_pda is mint authority).
    let remaining_tokens = total_supply
        .checked_sub(tokens_sold)
        .ok_or(AgentError::ArithmeticError)?;

    if remaining_tokens > 0 {
        let curve_bump_arr = [curve_bump];
        let curve_signer_seeds = [
            Seed::from(b"bonding-curve" as &[u8]),
            Seed::from(mint_key.as_ref()),
            Seed::from(curve_bump_arr.as_ref()),
        ];
        let curve_signers: &[Signer] = &[Signer::from(&curve_signer_seeds)];
        MintTo::new(mint, dex_pool, curve_pda, remaining_tokens)
            .invoke_signed(curve_signers)?;
    }

    Ok(())
}
