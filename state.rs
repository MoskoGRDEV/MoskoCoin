use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct TokenState {
    pub mint_authority: Pubkey,
    pub supply: u64,
    pub decimals: u8,
}