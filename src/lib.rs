use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};

#[cfg(test)]
mod tests {
    use bs58;
    use solana_client::rpc_client::RpcClient;
    use solana_sdk::{
        signature::{Keypair, Signer, read_keypair_file},
        transaction::Transaction,
        message::Message
    };
    use solana_program::{
        pubkey::Pubkey,
        system_instruction::transfer,
        instruction::{AccountMeta, Instruction},
        system_program
    };
    use std::io::{self, BufRead};
    use std::str::FromStr;

    const RPC_URL: &str = "https://api.devnet.solana.com";

    // #[test]
    // fn keygen() {
    //     let kp = Keypair::new();

    //     println!("You've generated a new Solana wallet: {}", kp.pubkey());

    //     println!("\nTo save your wallet, copy and paste the following into a JSON file:");
    //     println!("{:?}", kp.to_bytes());
    // }

    // #[test]
    // fn base58_to_wallet() {
    //     println!("Paste your Phantom private key (Base58):");
    //     let stdin = io::stdin();
    //     let base58 = stdin.lock().lines().next().unwrap().unwrap();

    //     let wallet_bytes = bs58::decode(&base58).into_vec().unwrap();
    //     println!("Wallet JSON format:");
    //     println!("{:?}", wallet_bytes);
    // }

    // #[test]
    // fn wallet_to_base58() {
    //     println!("Paste wallet JSON array (e.g. [12,34,...]):");
    //     let stdin = io::stdin();
    //     let json_str = stdin.lock().lines().next().unwrap().unwrap();

    //     let wallet_bytes: Vec<u8> = json_str
    //         .trim_matches(|c| c == '[' || c == ']')
    //         .split(',')
    //         .map(|s| s.trim().parse::<u8>().unwrap())
    //         .collect();

    //     let base58 = bs58::encode(&wallet_bytes).into_string();
    //     println!("Phantom import format:");
    //     println!("{}", base58);
    // }

    // #[test]
    // fn airdrop() {
    //     let keypair = read_keypair_file("Turbin3-wallet.json").expect("Failed to read wallet file");

    //     let client = RpcClient::new(RPC_URL);

    //     let airdrop_amount = 2_000_000_000;
    //     match client.request_airdrop(&keypair.pubkey(), airdrop_amount) {
    //         Ok(sig) => {
    //             println!("Success! Airdropped 2 Devnet SOL. Check your TX here:");
    //             println!("https://explorer.solana.com/tx/{}?cluster=devnet", sig);
    //         }
    //         Err(err) => {
    //             println!("Airdrop failed: {}", err);
    //         }
    //     }
    // }

    // #[test]
    // fn transfer_sol() {
    //     let keypair = read_keypair_file("Turbin3-wallet.json")
    //         .expect("Couldn't find wallet file");
        
    //     println!("Verifying keypair...");
    //     let pubkey = keypair.pubkey();
    //     let message_bytes = b"I verify my Solana Keypair!";
    //     let sig = keypair.sign_message(message_bytes);
        
    //     // Using the direct verification method
    //     if sig.verify(pubkey.as_ref(), message_bytes) {
    //         println!("✅ Signature verified successfully");
    //     } else {
    //         panic!(" Keypair verification failed");
    //     }

    //     let to_pubkey = Pubkey::from_str("4NEhzQmwNKnvDtnZmHcykybBNmW1KL2xkD9PN9f8MPsc")
    //         .expect("Invalid public key");
    //     println!("Destination wallet: {}", to_pubkey);

    //     println!("Connecting to devnet...");
    //     let rpc_client = RpcClient::new(RPC_URL);

    //     println!("Fetching recent blockhash...");
    //     let recent_blockhash = rpc_client
    //         .get_latest_blockhash()
    //         .expect("Failed to get recent blockhash");

    //     let transfer_instruction = transfer(
    //         &keypair.pubkey(),
    //         &to_pubkey,
    //         100_000_000,  // 0.1 SOL
    //     );

    //     println!("Creating transaction...");
    //     let transaction = Transaction::new_signed_with_payer(
    //         &[transfer_instruction],
    //         Some(&keypair.pubkey()),
    //         &vec![&keypair],
    //         recent_blockhash,
    //     );

    //     println!("Sending transaction...");
    //     let signature = rpc_client
    //         .send_and_confirm_transaction(&transaction)
    //         .expect("Failed to send transaction");

    //     println!(
    //         "\n🚀 Transfer successful! Check your transaction:\n\
    //         https://explorer.solana.com/tx/{}?cluster=devnet",
    //         signature
    //     );
    // }

    // #[test]
    // fn empty_wallet() {

    //     let keypair = read_keypair_file("dev-wallet.json")
    //         .expect("Couldn't find wallet file");
    //     let pubkey = keypair.pubkey();
        
    //     println!("Verifying keypair...");
    //     let message_bytes = b"I verify my Solana Keypair!";
    //     let sig = keypair.sign_message(message_bytes);
    //     if sig.verify(pubkey.as_ref(), message_bytes) {
    //         println!("✅ Signature verified successfully");
    //     } else {
    //         panic!(" Keypair verification failed");
    //     }

    //     let to_pubkey = Pubkey::from_str("4NEhzQmwNKnvDtnZmHcykybBNmW1KL2xkD9PN9f8MPsc")
    //         .expect("Invalid public key");
    //     println!("Destination wallet: {}", to_pubkey);


    //     println!("Connecting to devnet...");
    //     let rpc_client = RpcClient::new(RPC_URL);

        
    //     println!("Fetching current balance...");
    //     let balance = rpc_client
    //         .get_balance(&pubkey)
    //         .expect("Failed to get balance");
    //     println!("Current balance: {} SOL", balance as f64 / 1e9);

        
    //     println!("Fetching recent blockhash...");
    //     let recent_blockhash = rpc_client
    //         .get_latest_blockhash()
    //         .expect("Failed to get recent blockhash");

        
    //     println!("Calculating transaction fee...");
    //     let transfer_instruction = transfer(&pubkey, &to_pubkey, balance);
    //     let message = Message::new_with_blockhash(
    //         &[transfer_instruction],
    //         Some(&pubkey),
    //         &recent_blockhash,
    //     );
        
        
    //     let fee = rpc_client
    //         .get_fee_for_message(&message)
    //         .expect("Failed to get fee");
    //     println!("Estimated fee: {} lamports", fee);
        
        
    //     println!("Creating final transaction...");
    //     let final_transfer = transfer(&pubkey, &to_pubkey, balance - fee);
    //     let transaction = Transaction::new_signed_with_payer(
    //         &[final_transfer],
    //         Some(&pubkey),
    //         &[&keypair],
    //         recent_blockhash,
    //     );

        
    //     println!("Sending transaction...");
    //     let signature = rpc_client
    //         .send_and_confirm_transaction(&transaction)
    //         .expect("Failed to send transaction");

        
    //     println!(
    //         "\n💸 Entire balance transferred! Check your transaction:\n\
    //         https://explorer.solana.com/tx/{}?cluster=devnet",
    //         signature
    //     );
        
        
    //     let final_balance = rpc_client
    //         .get_balance(&pubkey)
    //         .expect("Failed to get final balance");
    //     println!("Wallet emptied! Final balance: {} lamports", final_balance);
    // }

    #[test]
    fn submit_rs() {
        let rpc_client = RpcClient::new(RPC_URL);
        
        
        let signer = read_keypair_file("Turbin3-wallet.json")
            .expect("Couldn't find wallet file");
        let signer_pubkey = signer.pubkey();
        println!("Signer: {}", signer_pubkey);

        
        let turbin3_program_id = Pubkey::from_str("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM")
            .expect("Invalid Turbin3 program ID");
        
        let collection = Pubkey::from_str("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2")
            .expect("Invalid collection address");
        
        let mpl_core_program = Pubkey::from_str("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d")
            .expect("Invalid MPL Core program ID");


        let (prereq_pda, _bump) = Pubkey::find_program_address(
            &[b"prereqs", signer_pubkey.as_ref()],
            &turbin3_program_id
        );
        println!("Prereq PDA: {}", prereq_pda);

        
        let (authority, _collection_bump) = Pubkey::find_program_address(
            &[b"collection", collection.as_ref()],
            &turbin3_program_id
        );
        println!("Collection Authority: {}", authority);

        
        let mint = Keypair::new();
        println!("New Mint: {}", mint.pubkey());

        
        let data = vec![77, 124, 82, 163, 21, 133, 181, 206];

        
        let accounts = vec![
            AccountMeta::new(signer_pubkey, true),
            AccountMeta::new(prereq_pda, false),
            AccountMeta::new(mint.pubkey(), true),
            AccountMeta::new(collection, false),
            AccountMeta::new_readonly(authority, false),
            AccountMeta::new_readonly(mpl_core_program, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ];


        let blockhash = rpc_client.get_latest_blockhash()
            .expect("Failed to get blockhash");

        
        let instruction = Instruction {
            program_id: turbin3_program_id,
            accounts,
            data,
        };

        
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&signer_pubkey),
            &[&signer, &mint],
            blockhash,
        );

        
        println!("⏳ Submitting transaction...");
        let signature = rpc_client.send_and_confirm_transaction(&transaction)
            .expect("Transaction failed");

        println!(
            "\n🎉 Turbin3 Rust submission complete! View transaction:\n\
            https://explorer.solana.com/tx/{}?cluster=devnet",
            signature
        );
    }
}