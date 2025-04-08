use solana_sdk::{signature::{Keypair, read_keypair_file}, transaction::Transaction, pubkey::Pubkey, message::Message};
use solana_client::rpc_client::RpcClient;
use solana_program::{instruction::Instruction};
use solana_sdk::system_instruction;
use spl_token::{state::Mint, instruction::initialize_mint};

#[tokio::main]
async fn create_token() {
    // Δημιουργία ενός νέου wallet (πορτοφόλι για τον πληρωτή)
    let payer = Keypair::new();
    let client = RpcClient::new("https://api.devnet.solana.com");

    // Δημιουργία νέου mint για το MoskoCoin
    let mint_keypair = Keypair::new();
    let mint_pubkey = mint_keypair.pubkey();

    // Δημιουργία συναλλαγής για την δημιουργία του token (mint)
    let create_mint_instruction = initialize_mint(
        &spl_token::id(),
        &mint_pubkey,
        &payer.pubkey(),
        None,
        9, // Ο αριθμός δεκαδικών σημείων (π.χ. 9 για το MoskoCoin)
    ).unwrap();

    // Δημιουργία και αποστολή της συναλλαγής
    let mut transaction = Transaction::new_with_payer(
        &[create_mint_instruction],
        Some(&payer.pubkey()),
    );

    // Λήψη του recent blockhash
    let recent_blockhash = client.get_recent_blockhash().unwrap().0;
    transaction.sign(&[&payer, &mint_keypair], recent_blockhash);

    // Αποστολή της συναλλαγής
    let result = client.send_and_confirm_transaction(&transaction);
    match result {
        Ok(signature) => println!("Token MoskoCoin δημιουργήθηκε με επιτυχία: {}", signature),
        Err(e) => println!("Σφάλμα: {:?}", e),
    }
}

fn main() {
    create_token();
}
