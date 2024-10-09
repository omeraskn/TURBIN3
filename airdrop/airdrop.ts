import { Connection, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js"

//We're also going to import our wallet and recreate the Keypair object
//using its private key:


import wallet from "./dev-wallet.json"
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const connection = new Connection("https://api.devnet.solana.com");

// We're going to import our keypair from the wallet file
//Create a Solana devnet connection to devnet SOL tokens
//Now we're going to establish a connection to the Solana devnet:
