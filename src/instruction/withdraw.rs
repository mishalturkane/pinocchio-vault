use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::{self},
    ProgramResult,
};
use pinocchio_system::instructions::Transfer;

use crate::error::MyProgramError;

pub fn process_withdraw(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [withdraw_acc, vault_acc, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !withdraw_acc.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    //if not empty is not a system account and if no lamports is not initialized
    if !vault_acc.data_is_empty() && vault_acc.lamports() > 0 {
        return Err(MyProgramError::InvalidAccount.into());
    }

    let bump = data[0];

    let seeds = [
        "pinocchio_vault_pda".as_bytes(),
        withdraw_acc.key(),
        &[bump],
    ];
    let vault_pda = pubkey::create_program_address(&seeds, &crate::ID)?;

    //check if the vault belongs to the withdraw_acc
    if vault_pda != *vault_acc.key() {
        return Err(MyProgramError::IncorrectVaultAcc.into());
    }

    let binding_bump = [bump];
    let signer_seeds = [
        Seed::from("pinocchio_vault_pda".as_bytes()),
        Seed::from(withdraw_acc.key()),
        Seed::from(&binding_bump),
    ];

    let signer = [Signer::from(&signer_seeds)];

    Transfer {
        from: vault_acc,
        to: withdraw_acc,
        lamports: vault_acc.lamports(),
    }
    .invoke_signed(&signer)?;

    Ok(())
}
