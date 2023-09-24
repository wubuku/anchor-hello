import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Hello } from "../target/types/hello";

describe("hello", () => {
  let provider = anchor.AnchorProvider.env();

  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  const program = anchor.workspace.Hello as Program<Hello>;

  const authority = provider.wallet.publicKey;

  let [helloWorld] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("hello-world")],
    program.programId
  );

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize(
        new anchor.BN(1_234),
        "hello",
        ["foo", "bar"]
    ).accounts(
        {
           helloWorld,
           authority,
           systemProgram: anchor.web3.SystemProgram.programId,
        }
    ).rpc();
    console.log("Your transaction signature", tx);

    // Fetch the state struct from the network.
    const accountState = await program.account.helloWorld.fetch(helloWorld);
    console.log("account state: ", accountState);

  });
});
