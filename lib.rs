mod error;
mod instruction;
mod processor;
mod state;

use solana_program::{
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    account_info::AccountInfo,
};
use processor::process_instruction;

entrypoint!(entry);

pub fn entry(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    process_instruction(program_id, accounts, instruction_data)
}