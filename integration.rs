use solana_program_test::*;
use solana_sdk::{signature::Keypair, transaction::Transaction};
use moskocoin::{instruction::MoskoCoinInstruction, processor::process_instruction};

#[tokio::test]
async fn test_create_token() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::new(
        "moskocoin",
        program_id,
        processor!(process_instruction),
    );

    let mint_keypair = Keypair::new();
    let authority_keypair = Keypair::new();

    program_test.add_account(
        mint_keypair.pubkey(),
        Account {
            lamports: 1000000000,
            ..Account::default()
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let instruction_data = MoskoCoinInstruction::CreateToken {
        decimals: 6,
        initial_supply: 1000000,
    };
    let mut data = Vec::new();
    instruction_data.serialize(&mut data).unwrap();

    // Προσθήκη περισσότερης λογικής test εδώ
}