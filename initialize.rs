use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
    system_instruction,
};
use moskocoin::instruction::MoskoCoinInstruction;
use borsh::BorshSerialize;

fn main() {
    let client = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
    let payer = Keypair::from_base58_string("YOUR_WALLET_PRIVATE_KEY_BASE58");
    let mint_keypair = Keypair::from_base58_string("MINT_KEYPAIR_FROM_JSON");
    let program_id = "YOUR_PROGRAM_ID".parse().unwrap();

    // Δημιουργία του mint account
    let rent = client.get_minimum_balance_for_rent_exemption(82).unwrap();
    let create_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &mint_keypair.pubkey(),
        rent,
        82,
        &spl_token::id(),
    );

    // Οδηγία δημιουργίας token
    let instruction_data = MoskoCoinInstruction::CreateToken {
        decimals: 6,
        initial_supply: 1_000_000_000, // 1 δις tokens
    };
    let mut data = Vec::new();
    instruction_data.serialize(&mut data).unwrap();

    let instruction = solana_sdk::instruction::Instruction {
        program_id,
        accounts: vec![
            solana_sdk::instruction::AccountMeta::new(mint_keypair.pubkey(), false),
            solana_sdk::instruction::AccountMeta::new_readonly(payer.pubkey(), true),
            solana_sdk::instruction::AccountMeta::new_readonly(spl_token::id(), false),
            solana_sdk::instruction::AccountMeta::new_readonly(solana_sdk::sysvar::rent::id(), false),
        ],
        data,
    };

    let recent_blockhash = client.get_latest_blockhash().unwrap();
    let tx = Transaction::new_signed_with_payer(
        &[create_account_ix, instruction],
        Some(&payer.pubkey()),
        &[&payer, &mint_keypair],
        recent_blockhash,
    );

    let signature = client.send_and_confirm_transaction(&tx).unwrap();
    println!("Mint Address: {}", mint_keypair.pubkey());
    println!("Transaction Signature: {}", signature);
}