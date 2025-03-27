mod programs;
//mod turbine_prereq; // Add this line to declare the module

#[cfg(test)] 
mod tests {
    use solana_client::{ rpc_client::RpcClient};
    use solana_sdk::{message::Message, msg, signature::{read_keypair_file, Keypair, Signer}, system_program, transaction::{self, Transaction}};
    use solana_program::{hash::hash, pubkey::Pubkey, system_instruction::transfer};
    use std::str::FromStr;

    use crate::programs::Turbin3_prereq::{TurbinePrereqProgram, CompleteArgs, UpdateArgs};
    // turbine_prereq
    // TurbinePrereq

    const RPC_URL: &str = "https://api.devnet.solana.com";

    #[test]
    fn keygen() {
        // Create a new keypair
        let kp = Keypair::new();
        println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string());
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    } 

    #[test]
    fn base58_to_wallet() {

        use bs58;
        use std::io::{self, BufRead};

        println!("Input your private key as base58:");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap(); 
        println!("Your wallet file is: ");
        let wallet = bs58::decode(base58).into_vec().unwrap(); 
        println!("{:?}", wallet);

    }

    #[test]
    fn wallet_to_base58() {
        use bs58;
        use std::io::{self, BufRead};

        println!("Input your private key as a wallet file byte array:");
        let stdin = io::stdin();

        let wallet = stdin.lock().lines().next().unwrap().unwrap()
        .trim_start_matches('[').trim_end_matches(']')
        .split(',')
        .map(|x| x.trim().parse::<u8>().unwrap())
        .collect::<Vec<u8>>();

        println!("Your private key is: ");
        let base58 = bs58::encode(wallet).into_string(); 
        println!("{:?}", base58);

    }
    
    #[test] 
    fn airdop() {

        // Import our keypair
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file"); 
        // Connected to Solana Devnet RPC Client
        let client = RpcClient::new(RPC_URL);

        // We're going to claim 2 devnet SOL tokens (2 billion lamports)
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {

            Ok(tx) => {
                println!("Airdrop of 2 SOL successful! Check out your TX here: ");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", tx.to_string());
            },
            Err(e) => println!("Oops, something went wrong: {}", e.to_string())
        }
    } 
    
    #[test] 
    fn transfer_sol() {
        // Import our keypair
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file"); 

        // With the imported Keypair, we can sign a new message.
        let pubkey = keypair.pubkey();
        let message_bytes = b"Main verify karna chahta hun!";
        let sig = keypair.sign_message(message_bytes);
        let sig_hashed = hash(sig.as_ref());

        // After that we can verify the singature, using the default implementation
        match sig.verify(&pubkey.to_bytes(), &sig_hashed.to_bytes()) {
            true => println!("Signature verified"),
            false => println!("Verification FAILED"),
        }

        // Define our Turbin3 public key
        let to_pubkey = Pubkey::from_str("GLtaTaYiTQrgz411iPJD79rsoee59HhEy18rtRdrhEUJ").unwrap();

        // Connected to Solana Devnet RPC Client
        let rpc_client = RpcClient::new(RPC_URL);

        // Get recent blockhash
        let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");

        // transfer 0.1 SOL to Turbine pubkey
        let ix = transfer(&pubkey, &to_pubkey, 1_000_000_00u64);
        let transaction = Transaction::new_signed_with_payer(&[ix],
             Some(&keypair.pubkey()),
              &vec![&keypair],
               recent_blockhash);
        msg!(" after trans");
        // Send the transaction
        let signature = rpc_client.send_and_confirm_transaction(&transaction).expect("Failed to send transaction");
        msg!(" after signature");
        // Print our transaction out
        println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet", signature);
        }

        #[test]
        fn empty_wallet() {

            let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
            let to_pubkey = Pubkey::from_str("GLtaTaYiTQrgz411iPJD79rsoee59HhEy18rtRdrhEUJ").unwrap();
            
            let rpc_client = RpcClient::new(RPC_URL);
            let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");

            // Get balance of dev wallet
            let balance = rpc_client.get_balance(&keypair.pubkey()).expect("Failed to get balance");

            // Create a test transaction to calculate fees
            let ix = transfer(&keypair.pubkey(), &to_pubkey, balance);
            let message = Message::new_with_blockhash(&[ix], Some(&keypair.pubkey()), &recent_blockhash);

            // Calculate exact fee rate to transfer entire SOL amount out of account minus fees
            let fee = rpc_client.get_fee_for_message(&message).expect("Failed to get fee calculator");

            // Deduct fee from lamports amount and create a TX with correct balance
            let ix = transfer(&keypair.pubkey(), &to_pubkey, balance - fee);
            let transaction = Transaction::new_signed_with_payer(&[ix],
             Some(&keypair.pubkey()),
              &vec![&keypair],
               recent_blockhash);

            let signature = rpc_client.send_and_confirm_transaction(&transaction).expect("Failed to send transaction");  
                
            println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet", signature);

        }

        #[test]
        fn enroll() {
            let rpc_client = RpcClient::new(RPC_URL);
            let signer = read_keypair_file("Turbin3-wallet.json").expect("Couldn't find wallet file");
            //msg!("Phantom wallet: {}", signer.pubkey().to_string());

            let prereq = TurbinePrereqProgram::derive_program_address(&[
                b"prereq",
                signer.pubkey().to_bytes().as_ref(),
            ]);
            msg!("Prereq address: {}", prereq.to_string());

            // Make sure there's enough SOL in the wallet for account creation
            // let balance = rpc_client.get_balance(&signer.pubkey())
            //     .expect("Failed to get balance");
            // msg!("Phantom Balance: {}", balance);

            let args = CompleteArgs {
                github: b"rohit-sekhri51".to_vec()
            };

            let recent_blockhash = rpc_client.get_latest_blockhash()
                .expect("Failed to get recent blockhash");
            msg!("recent blockhash: {}", recent_blockhash);

            let transaction = TurbinePrereqProgram::complete(
                &[&signer.pubkey(), &prereq, &system_program::id()],
                &args,
                Some(&signer.pubkey()),
                &[&signer],
                recent_blockhash
            );
            msg!("BEFORE send_and_confirm_transaction");

            let signature = rpc_client.send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

            println!("Enrollment successful! Transaction signature: ");
            println!("https://explorer.solana.com/tx/{}/?cluster=devnet", signature);
           
            msg!("AFTER send_and_confirm_transaction");
        }
    }