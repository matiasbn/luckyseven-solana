import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Luckyseven } from "../target/types/luckyseven";

describe("luckyseven", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Luckyseven as Program<Luckyseven>;
  const programStorage = anchor.web3.Keypair.generate();

  it("is initialized!", async () => {
    const tx = await program.rpc.initialize({
      accounts: {
        programStorage: programStorage.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        owner: program.provider.wallet.publicKey,
      },
      signers: [programStorage],
    });
    console.log("Your transaction signature", tx);
    await program.provider.connection.confirmTransaction(tx);
    const { initialized, lastDifference, gameOwner } =
      await program.account.programStorage.fetch(programStorage.publicKey);
    console.log(initialized);
    console.log(lastDifference.toString());
    console.log(gameOwner.toBase58());
  });

  it("stores a random number", async () => {
    const randomNumber = anchor.web3.Keypair.generate();
    // Add your test here.
    const tx = await program.rpc.storeNumber({
      accounts: {
        programStorage: programStorage.publicKey,
        owner: program.provider.wallet.publicKey,
        randomNumber: randomNumber.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [randomNumber],
    });
    console.log("Your transaction signature", tx);

    const { owner, number } = await program.account.randomNumber.fetch(
      randomNumber.publicKey
    );
    console.log(owner.toBase58());
    console.log(number.toString());
  });
});
