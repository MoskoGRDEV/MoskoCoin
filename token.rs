use solana_sdk::{pubkey::Pubkey, signature::{Keypair, read_keypair_file}, transaction::Transaction};
use solana_client::rpc_client::RpcClient;
use solana_program::system_instruction;
use solana_sdk::system_instruction::SystemInstruction;

pub fn create_mosko_coin() {
    // Δημιουργία ενός νέου wallet
    let payer = Keypair::new();
    let client = RpcClient::new("https://api.devnet.solana.com");

    // Δημιουργία του public key για το MoskoCoin
    let mosko_coin_address = Pubkey::new_unique();

    // Δημιουργία και αποστολή της συναλλαγής για το token
    let mut transaction = Transaction::new_with_payer(
        &[system_instruction::transfer(
            &payer.pubkey(),
            &mosko_coin_address,
            1_000_000_000,  // ποσό MoskoCoin για αποστολή
        )],
        Some(&payer.pubkey()),
    );

    // Υπογραφή της συναλλαγής και αποστολή
    let recent_blockhash = client.get_recent_blockhash().unwrap().0;
    transaction.sign(&[&payer], recent_blockhash);
    let result = client.send_and_confirm_transaction(&transaction);

    match result {
        Ok(signature) => println!("Το MoskoCoin δημιουργήθηκε με επιτυχία: {}", signature),
        Err(e) => println!("Σφάλμα: {:?}", e),
    }
}
