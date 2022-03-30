import * as anchor from '@project-serum/anchor';
import { BN, Program } from '@project-serum/anchor';
import { Luckyseven } from '../target/types/luckyseven';
import { expect } from 'chai';
import consola from 'consola';
import {
  findAssociatedTokenAddress,
  getTokenMintPublicKey,
} from '../constants';
import { SystemProgram, SYSVAR_RENT_PUBKEY } from '@solana/web3.js';
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddress,
  TOKEN_PROGRAM_ID,
} from '@solana/spl-token';

describe('Mint tokens', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Luckyseven as Program<Luckyseven>;

  it('should create a mint account', async () => {
    const tokenMintPublicKey = await getTokenMintPublicKey();
    const signer = program.provider.wallet.publicKey;

    const associatedTokenAccount = await findAssociatedTokenAddress(
      signer,
      tokenMintPublicKey,
    );
    await program.rpc.createMintAccount({
      accounts: {
        tokenMint: tokenMintPublicKey,
        signer: program.provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenAccount: associatedTokenAccount,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      },
    });
  });
});
