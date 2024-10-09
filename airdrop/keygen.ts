import { Keypair } from "@solana/web3.js";

//Generate a new keypair
let kp = Keypair.generate();
console.log(`You've generated a new Solana wallet: ${kp.publicKey.toBase58()}`);
console.log(`You can fill it at https://faucet.solana.com`);
console.log(`[${kp.secretKey}]`);

// TURBIN3 -> A1CShprYJmKFFpofciX5juVJbeyCPCYEsDJJCk33fsb4
// TURBIN3 -> [86,5,80,130,238,193,62,221,29,26,52,152,128,90,134,74,123,194,51,89,195,190,133,45,61,206,139,106,136,54,191,43,133,198,31,5,18,101,136,211,244,164,25,11,43,68,203,112,154,4,48,50,11,111,165,53,224,62,74,203,37,150,123,79]

// Bridge to TURBIN3 -> C4KXohCsHvLt1uc6YvhfuG8UaNkKRyBDM6BSqNrii5Sx CH6ZGcHAmovtnb11dm88nw25Ed2uDNsxTu8krr2abUdG
// Bridge to TURBIN3 -> [101,205,95,180,95,189,163,214,42,81,208,234,37,192,42,5,242,118,135,90,208,157,215,199,88,146,210,3,191,87,236,0,164,74,64,119,123,32,62,9,38,165,243,240,63,201,35,148,204,150,160,249,85,146,215,50,74,184,75,28,96,15,50,73]