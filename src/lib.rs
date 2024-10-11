// src/lib.rs
use solana_client::rpc_client::RpcClient;
use solana_program::{pubkey::Pubkey, system_program}; // Import system_program
use solana_sdk::{
    signature::{read_keypair_file, Keypair, Signer},
    transaction::Transaction,
};
use std::str::FromStr;

const RPC_URL: &str = "https://api.devnet.solana.com";

// Import the Turbin3 program structures
mod programs;
use crate::programs::Turbin3_prereq::{CompleteArgs, Turbin3PrereqProgram};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn submit_completion() {
        // Create a Solana devnet connection
        let rpc_client = RpcClient::new(RPC_URL);

        // Define our accounts
        let signer = read_keypair_file("turbin-wallet.json").expect("Couldn't find wallet file");

        // Create PDA for the prereq account
        let prereq = Turbin3PrereqProgram::derive_program_address(&[
            b"prereq",
            signer.pubkey().to_bytes().as_ref(),
        ]);

        // Define our instruction data
        let args = CompleteArgs {
            github: b"omeraskn".to_vec(), // Your GitHub username
        };

        // Get recent blockhash
        let blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        // Create the transaction to invoke the "complete" function
        let transaction = Turbin3PrereqProgram::complete(
            &[&signer.pubkey(), &prereq, &system_program::id()],
            &args,
            Some(&signer.pubkey()),
            &[&signer],
            blockhash,
        );

        // Send the transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        // Print the transaction output
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }

    #[test]
    fn wallet_converter() {
        // Load the keypair from the file (replace with the correct path)
        let signer = read_keypair_file("turbin-wallet.json").expect("Couldn't find wallet file");
    
        // Access the private key bytes directly
        let wallet = signer.to_bytes().to_vec();
    
        println!("Your private key as a byte array:");
        println!("{:?}", wallet);
    
        // Convert the byte array to Base58
        let base58 = bs58::encode(wallet).into_string();
        println!("Your private key in Base58 format:");
        println!("{:?}", base58);
    }

    #[test]
    fn keygen() {
        // Generate a new keypair
        let kp = Keypair::new();
        println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string()); println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());

    }
    // TURBIN3 Rust - 2JXVu4wcMTbxUqLhVDpH3f2bzJsg7WPeAsKhthAz4orj

    #[test]
    fn airdrop() {
        // Import the wallet keypair from JSON file
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file"); 

        // Create a connection to Solana devnet
        let rpc_url = "https://api.devnet.solana.com";
        let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

        // Request airdrop of 2 SOL
        match client.request_airdrop(&keypair.pubkey(),  2_000_000_000u64) {
            Ok(signature) => {
                println!("Success! Airdrop requested. Check out your TX here:");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", signature);
            },
            Err(e) => {
                eprintln!("Oops, something went wrong: {:?}", e);
            }
        }
    }

    /*
    #[test]
    fn transfer_sol() {
        // Import the wallet keypair from JSON file
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

        // Define the Turbin3 public key (replace with your actual public key)
        let to_pubkey = Pubkey::from_str("C4KXohCsHvLt1uc6YvhfuG8UaNkKRyBDM6BSqNrii5Sx").unwrap();

        // Create a Solana devnet connection
        let rpc_url = "https://api.devnet.solana.com";
        let rpc_client = RpcClient::new(rpc_url);

        // Get recent blockhash
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        // Get balance of the dev wallet
        let balance = rpc_client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get balance");

        println!("Current balance: {} lamports", balance);

        if balance == 0 {
            println!("The wallet has no SOL to transfer.");
            return;
        }

        // Create a mock transaction to calculate fees
        let message = Message::new_with_blockhash(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
            Some(&keypair.pubkey()),
            &recent_blockhash,
        );

        // Calculate the fee for the transaction
        let fee = rpc_client
            .get_fee_for_message(&message)
            .expect("Failed to get fee calculator");

        println!("Transaction fee: {} lamports", fee);

        if balance <= fee {
            println!("Not enough balance to cover the transaction fee.");
            return;
        }

        // Deduct fee from the balance and create the real transaction
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        // Send and confirm the transaction
        match rpc_client.send_and_confirm_transaction(&transaction) {
            Ok(signature) => {
                println!(
                    "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
                    signature
                );
            }
            Err(e) => {
                eprintln!("Failed to send transaction: {:?}", e);
            }
        }
    }
    */
/*
    // SEND 0.1 SOL
    #[test]
    fn transfer_sol() {
        // Import the wallet keypair from JSON file
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

        // Define the Turbin3 public key (replace with your actual public key)
        let to_pubkey = Pubkey::from_str("C4KXohCsHvLt1uc6YvhfuG8UaNkKRyBDM6BSqNrii5Sx").unwrap();

        // Create a Solana devnet connection
        let rpc_url = "https://api.devnet.solana.com";
        let rpc_client = RpcClient::new(rpc_url);

        // Get recent blockhash
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        // Create a transfer transaction to send 0.1 SOL (1_000_000 lamports)
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(
                &keypair.pubkey(), // From public key
                &to_pubkey,        // To public key (Turbin3 address)
                1_000_000          // Amount in lamports (0.1 SOL = 1_000_000 lamports)
            )],
            Some(&keypair.pubkey()),  // Payer of the transaction
            &vec![&keypair],          // Signers
            recent_blockhash          // Recent blockhash for transaction validity
        );

        // Send and confirm the transaction
        match rpc_client.send_and_confirm_transaction(&transaction) {
            Ok(signature) => {
                println!(
                    "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
                    signature
                );
            }
            Err(e) => {
                eprintln!("Failed to send transaction: {:?}", e);
            }
        }
    }
*/

}