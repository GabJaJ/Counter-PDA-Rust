import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CounterPda } from "../target/types/counter_pda";

describe("counter-pda", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.CounterPda as Program<CounterPda>;

  let counterPK: anchor.web3.PublicKey;
  let bump: number;


  before(async () => {
    [counterPK, bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("counter", "utf-8"), //seed "counter"
        provider.wallet.publicKey.toBuffer() // counter authority
      ]
      program.programId
    );

    console.log("Conter PDA Address: ", counterPK.toBase58());
    console.log("Counter canonical bump: ", bump);
  });

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.createCounter()
      .accounts({
        authority: provider.wallet.publicKey,
        counterPK: counterPK,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
