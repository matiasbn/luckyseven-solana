import * as anchor from '@project-serum/anchor';
import { BN, Program } from '@project-serum/anchor';
import { Luckyseven } from '../target/types/luckyseven';
import { expect } from 'chai';
import consola from 'consola';
import { getTokenMintPublicKey } from '../constants';

describe('Mint tokens', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Luckyseven as Program<Luckyseven>;

  it('should create a mint authority', async () => {
    const tokenMintPublicKey = await getTokenMintPublicKey();
    await program.rpc.createMintAuthority({
      accounts: {
        tokenMint: tokenMintPublicKey,
        owner: program.provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
    });

    // const tx = await program.rpc.initialize(maxNumber, targetValue, {
    //   accounts: {
    //     programStorage: programStorage.publicKey,
    //     systemProgram: anchor.web3.SystemProgram.programId,
    //     owner: program.provider.wallet.publicKey,
    //     authorityAccount: authorityAccount.publicKey,
    //   },
    //   signers: [programStorage],
    // });
    // await program.provider.connection.confirmTransaction(tx);
    // const { initialized, winnerDifference, targetNumber } =
    //   await program.account.programStorage.fetch(programStorage.publicKey);
    // expect(initialized).to.be.true;
    // expect(winnerDifference).to.be.eql(maxNumber);
    // expect(targetNumber).to.be.eql(targetValue);
  });
});
