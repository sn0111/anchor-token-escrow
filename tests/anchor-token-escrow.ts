import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorTokenEscrow } from "../target/types/anchor_token_escrow";
import { BN } from "bn.js";
import { readFileSync } from 'fs';

function createKeypairFromFile(path:string):anchor.web3.Keypair{
  return anchor.web3.Keypair.fromSecretKey(
      Buffer.from(JSON.parse(readFileSync(path,"utf-8")))
  )
}

describe("anchor-token-escrow", () => {
  
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.AnchorTokenEscrow as Program<AnchorTokenEscrow>;


  const alicePair = createKeypairFromFile("/home/satya0111/Documents/token-escrow-accounts/ALiMAttZLtGCLyEy8W4oQFNTxUZXtEnQTtF78ej7FAG.json")
  const aliceMint = new anchor.web3.PublicKey("ALMT6tg4zekQqupCsMmfzkAsrud72NHuVHkiTZJu4qXP")
  const aliceTokenAccount = new anchor.web3.PublicKey("ALabSCqev7rzfLRcea2R1QyYYJyQQczVA4WKGZL78ySU")

  const bobPair = createKeypairFromFile("/home/satya0111/Documents/token-escrow-accounts/BobF6tAzQpAwKBd3Xjt7QWKCYHekPByu5eXtMTjUumi.json")
  const bobMint = new anchor.web3.PublicKey("BoM7bjvFrGt8k9468M11QAHntfiqj8mQZtVG1qRkAit")
  const bobTokenAccount = new anchor.web3.PublicKey("BAT54B5FHPUaoFRyCi2HkWrr1Z61VHUX9jsBz92vxG3Q")

  const pda = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("escrow")],program.programId)

  const initilizerReceiveAccountPair = anchor.web3.Keypair.generate()
  const vaultAccountPair = anchor.web3.Keypair.generate()
  const escrowAccountPair = anchor.web3.Keypair.generate()

  const receiverReceiveAccountPair = anchor.web3.Keypair.generate()

  it("Is initialized escrow account!", async () => {
        
    try {
      const tx = await program.methods
      .initializeEscrow(new BN(10**9*10))
      .accounts({
        initilizer:alicePair.publicKey,
        initilizerMint:aliceMint,
        receiverMint:bobMint,
        initilizerDepositAccount:aliceTokenAccount,
        initilizerReceiveAccount:initilizerReceiveAccountPair.publicKey,
        escrowAccount:escrowAccountPair.publicKey,
        vaultAccount:vaultAccountPair.publicKey
      })
      .signers([alicePair,initilizerReceiveAccountPair,escrowAccountPair,vaultAccountPair])
      .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
     console.log(error) 
    }
  });

  it("Is exchange escrow tokens!", async () => {

    try {
      const tx = await program.methods
      .exchangeToken(new BN(10**9*11))
      .accounts({
        receiver:bobPair.publicKey,
        escrowAccount:escrowAccountPair.publicKey,
        receiverMint:bobMint,
        initilizerMint:aliceMint,
        receiverTokenAccount:bobTokenAccount,
        receiverReceiveAccount:receiverReceiveAccountPair.publicKey,
        initilizerReceiveAccount:initilizerReceiveAccountPair.publicKey,
        vaultAccount:vaultAccountPair.publicKey,
        vaultPda:pda[0]
      })
      .signers([bobPair,receiverReceiveAccountPair])
      .rpc();

      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error)
    }
    
  });

});
