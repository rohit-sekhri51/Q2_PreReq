import { Transaction, SystemProgram, Connection, Keypair,
    LAMPORTS_PER_SOL, sendAndConfirmTransaction, PublicKey } from "@solana/web3.js";

import wallet from "./dev-wallet.json";
import { Console } from "console";

// Import our dev wallet keypair from the wallet file
const from = Keypair.fromSecretKey(new Uint8Array(wallet));
// Define our Turbin3 public key
const to = new PublicKey("GLtaTaYiTQrgz411iPJD79rsoee59HhEy18rtRdrhEUJ");

// Create a Solana devnet connection
const connection = new Connection("https://api.devnet.solana.com");

// Transfer 0.1 SOL to Turbine
/* Commenting out this code block to avoid sending 0.1 SOL again
(async () => {
    try {
    const transaction = new Transaction().add(
        SystemProgram.transfer({
            fromPubkey: from.publicKey,
            toPubkey: to,
            lamports: LAMPORTS_PER_SOL * 0.1
        })
    );
    transaction.recentBlockhash = (await connection.getLatestBlockhash('confirmed')).blockhash;
    transaction.feePayer = from.publicKey;

    // Sign transaction, broadcast, and confirm
    const signature = await sendAndConfirmTransaction(
        connection,
        transaction,
        [from]
    );

    console.log(`Success! Check out your TX here: https://explorer.solana.com/tx/${signature}?cluster=devnet`);
    } catch(e) {
    console.error(`Oops, something went wrong: ${e}`)
    }
    })();
*/

(async () => {
    try {
    // Get balance of dev wallet
    const balance = await connection.getBalance(from.publicKey);
    console.log('Balance should equal to 1.899995 SOL. Actual balance is: ', balance / LAMPORTS_PER_SOL);
    
    // Create a test transaction to calculate fees
    const transaction = new Transaction().add(
        SystemProgram.transfer({        // instruction to be pop
            fromPubkey: from.publicKey,
            toPubkey: to,
            lamports: balance,
        })
    );
    transaction.recentBlockhash = (await connection.getLatestBlockhash('confirmed')).blockhash;
    transaction.feePayer = from.publicKey;

    // Calculate exact fee rate to transfer entire SOL amount out of account minus fees
    const fee = (await connection.getFeeForMessage(transaction.compileMessage(),'confirmed')).value || 0;
    console.log("Fee for this tx is: ", fee / LAMPORTS_PER_SOL);

    // Remove our transfer instruction to replace it
    transaction.instructions.pop();

    // Now add the instruction back with correct amount of lamports
    transaction.add(
        SystemProgram.transfer({        // instruction replaced
            fromPubkey: from.publicKey,
            toPubkey: to,
            lamports: balance - fee,
        })
    );
    console.log("Transfering back to Turbine the whole amount minus fee for this tx:", balance - fee / LAMPORTS_PER_SOL);
    // Sign transaction, broadcast, and confirm
    const signature = await sendAndConfirmTransaction(
            connection,
            transaction,
            [from]
        );
    console.log(`Success! Check out your TX here: https://explorer.solana.com/tx/${signature}?cluster=devnet`)
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
    })();