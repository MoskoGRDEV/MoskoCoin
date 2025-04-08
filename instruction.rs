use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum MoskoCoinInstruction {
    CreateToken {
        decimals: u8,
        initial_supply: u64,
    },
    Transfer {
        amount: u64,
    },
    Burn {
        amount: u64,
    },
}