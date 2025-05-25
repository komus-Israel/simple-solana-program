import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Hello } from "../target/types/hello";

const { SystemProgram } = anchor.web3;

describe("Hello Program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const wallet = anchor.getProvider().wallet;

  const program = anchor.workspace.hello as Program<Hello>;

  it("Is initialized!", async () => {
    const data = new anchor.BN(75);
    const newAccount = new anchor.web3.Keypair();
    const accounts = {
      newAccount: newAccount.publicKey,
      signer: wallet.publicKey,
      systemProgram: SystemProgram.programId,
    }

    const tx = await program.methods
      .initialize(data)
      .accounts(accounts)
      .signers([newAccount])
      .rpc();
    // Add your test here.
  
    console.log("Your transaction signature", tx);
  });

  it("Fetches the new account", async ()=>{
    const accounts = await program.account.newAccount.all();
    // console.log(accounts[0].account.data.toString())
    console.log(accounts)
  })
});