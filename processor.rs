use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};
use spl_token::{
    instruction::{initialize_mint, mint_to, transfer, burn},
};
use crate::instruction::MoskoCoinInstruction;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = MoskoCoinInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;
    let accounts_iter = &mut accounts.iter();

    match instruction {
        MoskoCoinInstruction::CreateToken { decimals, initial_supply } => {
            msg!("Creating MoskoCoin");
            let mint_account = next_account_info(accounts_iter)?;
            let mint_authority = next_account_info(accounts_iter)?;
            let token_program = next_account_info(accounts_iter)?;
            let rent = next_account_info(accounts_iter)?;

            let rent_sysvar = Rent::from_account_info(rent)?;
            let required_lamports = rent_sysvar.minimum_balance(spl_token::state::Mint::LEN);

            if **mint_account.lamports.borrow() < required_lamports {
                return Err(ProgramError::InsufficientFunds);
            }

            let init_mint_ix = initialize_mint(
                token_program.key,
                mint_account.key,
                mint_authority.key,
                None,
                decimals,
            )?;

            solana_program::program::invoke(
                &init_mint_ix,
                &[token_program.clone(), mint_account.clone(), mint_authority.clone(), rent.clone()],
            )?;

            let mint_to_ix = mint_to(
                token_program.key,
                mint_account.key,
                mint_authority.key,
                mint_authority.key,
                &[],
                initial_supply,
            )?;

            solana_program::program::invoke(
                &mint_to_ix,
                &[token_program.clone(), mint_account.clone(), mint_authority.clone()],
            )?;

            msg!("MoskoCoin created successfully");
            Ok(())
        }

        MoskoCoinInstruction::Transfer { amount } => {
            msg!("Transferring MoskoCoin");
            let source_account = next_account_info(accounts_iter)?;
            let dest_account = next_account_info(accounts_iter)?;
            let authority = next_account_info(accounts_iter)?;
            let token_program = next_account_info(accounts_iter)?;

            let transfer_ix = transfer(
                token_program.key,
                source_account.key,
                dest_account.key,
                authority.key,
                &[],
                amount,
            )?;

            solana_program::program::invoke(
                &transfer_ix,
                &[token_program.clone(), source_account.clone(), dest_account.clone(), authority.clone()],
            )?;

            msg!("Transfer completed");
            Ok(())
        }

        MoskoCoinInstruction::Burn { amount } => {
            msg!("Burning MoskoCoin");
            let account_to_burn = next_account_info(accounts_iter)?;
            let authority = next_account_info(accounts_iter)?;
            let token_program = next_account_info(accounts_iter)?;
            let mint_account = next_account_info(accounts_iter)?;

            let burn_ix = burn(
                token_program.key,
                account_to_burn.key,
                mint_account.key,
                authority.key,
                &[],
                amount,
            )?;

            solana_program::program::invoke(
                &burn_ix,
                &[token_program.clone(), account_to_burn.clone(), mint_account.clone(), authority.clone()],
            )?;

            msg!("Burn completed");
            Ok(())
        }
    }
}