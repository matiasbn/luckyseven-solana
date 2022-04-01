import * as anchor from '@project-serum/anchor';
import { BN, Program } from '@project-serum/anchor';
import { Luckyseven } from '../target/types/luckyseven';
import { expect } from 'chai';
import consola from 'consola';

describe('luckyseven', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Luckyseven as Program<Luckyseven>;
  const programStorage = anchor.web3.Keypair.generate();
  const authorityAccount = anchor.web3.Keypair.generate();

  const randomValue1 = Number((Math.random() * 10000).toFixed(0));
  const randomValue2 = Number((Math.random() * 10000).toFixed(0));
  const maxNumber = new BN(Math.max(randomValue1, randomValue2));
  const targetValue = new BN(Math.min(randomValue1, randomValue2));

  it('is initialized!', async () => {
    await program.rpc.setProgramAuthor({
      accounts: {
        authorAccount: authorityAccount.publicKey,
        owner: program.provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [authorityAccount],
    });

    const tx = await program.rpc.initialize(maxNumber, targetValue, {
      accounts: {
        programStorage: programStorage.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        owner: program.provider.wallet.publicKey,
        authorityAccount: authorityAccount.publicKey,
      },
      signers: [programStorage],
    });
    await program.provider.connection.confirmTransaction(tx);
    const { initialized, winnerDifference, targetNumber } =
      await program.account.programStorage.fetch(programStorage.publicKey);
    expect(initialized).to.be.true;
    expect(winnerDifference).to.be.eql(maxNumber);
    expect(targetNumber).to.be.eql(targetValue);
  });

  it('gets a random number', async () => {
    const randomNumber = anchor.web3.Keypair.generate();
    // Add your test here.
    await program.rpc.getNumber({
      accounts: {
        programStorage: programStorage.publicKey,
        owner: program.provider.wallet.publicKey,
        randomNumber: randomNumber.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [randomNumber],
    });

    const { number, winner } = await program.account.randomNumber.fetch(
      randomNumber.publicKey,
    );
    const { targetNumber, winnerDifference } =
      await program.account.programStorage.fetch(programStorage.publicKey);
    const difference = number.sub(targetNumber).abs();
    expect(difference.toString()).to.be.eq(winnerDifference.toString());
    consola.info('Random number: ', number.toString());
    consola.info('Target number:', targetNumber.toString());
    consola.info('Difference:', difference.toString());
    consola.info('Is winner?: ', winner);
  });
});
