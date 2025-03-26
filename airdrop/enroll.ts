import { Connection, Keypair, PublicKey } from "@solana/web3.js"
import { Program, Wallet, AnchorProvider } from "@coral-xyz/anchor"
import { IDL, Turbin3Prereq } from "./programs/Turbin3_prereq";
import wallet from "./Turbin3-wallet.json";

// We're going to import our keypair from the wallet file
const kp = Keypair.fromSecretKey(new Uint8Array(wallet));
console.log(`You've generated a new Solana wallet: ${kp.publicKey.toBase58()}`);

// Create a devnet connection
const connection = new Connection("https://api.devnet.solana.com");

// Github account
const github = Buffer.from("rohit-sekhri51","utf8");
console.log("GitHub account:", github);

// Create our anchor provider
const provider = new AnchorProvider(connection, new Wallet(kp), {commitment: "confirmed"});

// Create our program
const program : Program<Turbin3Prereq> = new Program(IDL, provider);

// Create the PDA for our enrollment account
const enrollment_seeds = [Buffer.from("prereq"), kp.publicKey.toBuffer()];

const [enrollment_key, _bump] = PublicKey.findProgramAddressSync(enrollment_seeds, program.programId);
console.log("enrollment_key is: ", enrollment_key.toBase58());

// Execute our enrollment transaction
(async () => {
    try {
    const txhash = await program.methods
        .submit(github)
        .accounts({
            signer: kp.publicKey,
        })
        .signers([
            kp
        ]).rpc({ commitment: "confirmed" });

    console.log(`Success! Check out your TX here: https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
    } catch(e) {
    console.error(`Oops, something went wrong: ${e}`)
    }
}) 
();