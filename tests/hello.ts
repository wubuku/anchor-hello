import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Hello } from "../target/types/hello";
import { Connection, Keypair, LAMPORTS_PER_SOL } from '@solana/web3.js';

describe("hello", () => {
  let provider = anchor.AnchorProvider.env();

  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  const authority = provider.wallet.publicKey;

  const program = anchor.workspace.Hello as Program<Hello>;


  it("Is initialized!", async () => {

    // ----------------------------------------------------------
    // Human's wallet
    const human = Keypair.generate();
    const connection = anchor.AnchorProvider.env().connection;
    // Request sol airdrop (for human to be able to do transactions)
    await requestAirdrop(connection, human.publicKey, LAMPORTS_PER_SOL)
    // ----------------------------------------------------------

    let [helloWorld] = anchor.web3.PublicKey.findProgramAddressSync(
        [
            Buffer.from("hello-world"),
            human.publicKey.toBuffer()
        ],
        program.programId
    );

    // Add your test here.
    const tx = await program.methods.initialize(
        new anchor.BN(1_234),
        ["foo", "bar"],
        ["hello", "world"],
        "hello"
    ).accounts(
        {
           helloWorld,
           //authority,
           authority: human.publicKey,
           systemProgram: anchor.web3.SystemProgram.programId,
        }
    )
    .signers(
        [human]
    )
    .rpc();
    console.log("Your transaction signature", tx);

    // Fetch the state struct from the network.
    const accountState = await program.account.helloWorld.fetch(helloWorld);
    console.log("account state: ", accountState);

  });
});

async function requestAirdrop(connection: Connection, address: anchor.web3.PublicKey, lamports: number) {
    const tx = await connection.requestAirdrop(address, lamports);
    await connection.confirmTransaction(tx);
}