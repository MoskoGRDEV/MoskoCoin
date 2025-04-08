use thiserror::Error;
use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum MoskoCoinError {
    #[error("Invalid instruction")]
    InvalidInstruction,
    #[error("Insufficient funds")]
    InsufficientFunds,
}

impl From<MoskoCoinError> for ProgramError {
    fn from(e: MoskoCoinError) -> Self {
        ProgramError::Custom(e as u32)
    }
}