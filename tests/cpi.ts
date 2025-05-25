import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Cpi } from "../target/types/cpi";
const { LAMPORTS_PER_SOL, SystemProgram } = anchor.web3;
import chai from "chai";
import chaiAsPromised from "chai-as-promised";

chai.should();
chai.use(chaiAsPromised);

const sol = (amount) => {
  return amount / LAMPORTS_PER_SOL
}


describe("hello", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();
  const wallet = provider.wallet;

  const program = anchor.workspace.cpi as Program<Cpi>;

  it("SOL Transfer Anchor", async () => {

    const recipient = new anchor.web3.Keypair();
    const fromBalanceBeforeTransfer = await provider.connection.getBalance(wallet.publicKey);

    const accounts = {
      sender: wallet.publicKey,
      recipient: recipient.publicKey,
      // systemProgram: SystemProgram.programId
    }

    const amount = new anchor.BN(1 * LAMPORTS_PER_SOL)
    const tx = await program.methods
      .solTransfer(amount)
      .accounts(accounts)
      .rpc();
    
    console.log("Your transaction signature", tx);

    const fromBalanceAfterTransfer = await provider.connection.getBalance(wallet.publicKey);
    const toBalance = await provider.connection.getBalance(recipient.publicKey);
    // console.log("Balance: ", sol(balance));

    sol(toBalance).toString().should.be.equal(sol(amount).toString(), "Recipient balance is not 1 SOL");

    console.log("Sender balance: ", fromBalanceAfterTransfer)
    console.log("Recipient balance: ", toBalance);
  });
});
