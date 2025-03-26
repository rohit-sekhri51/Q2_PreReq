import { Keypair } from "@solana/web3.js";
import bs58 from 'bs58';

//Generate a new keypair
let kp = Keypair.generate();
console.log(`You've generated a new Solana wallet: ${kp.publicKey.toBase58()}`);

// Put the secret key in JSON file
console.log(`[${kp.secretKey}]`);

const privateKey = bs58.encode(kp.secretKey);
console.log(`Your privateKey is: ${privateKey}`);