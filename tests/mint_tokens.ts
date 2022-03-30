import * as anchor from '@project-serum/anchor';
import { BN, Program } from '@project-serum/anchor';
import { Luckyseven } from '../target/types/luckyseven';
import {
  findAssociatedTokenAddress,
  getTokenMintPublicKey,
} from '../constants';
import { SYSVAR_RENT_PUBKEY } from '@solana/web3.js';
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
} from '@solana/spl-token';
import { expect } from 'chai';

describe('Mint tokens', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());
  const program = anchor.workspace.Luckyseven as Program<Luckyseven>;

  const {
    connection,
    wallet: { publicKey: signer },
  } = program.provider;

  it('should create a mint account and mint initial supply to signer', async () => {
    const tokenMintPublicKey = await getTokenMintPublicKey();

    const associatedTokenAccount = await findAssociatedTokenAddress(
      signer,
      tokenMintPublicKey,
    );
    const initialSupply = 5_000_000;
    await program.rpc.createMintAccount(new BN(initialSupply), {
      accounts: {
        signer,
        associatedTokenAccount,
        tokenMint: tokenMintPublicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      },
    });

    const { value } = await connection.getParsedTokenAccountsByOwner(signer, {
      mint: tokenMintPublicKey,
    });

    const {
      tokenAmount: { uiAmount },
    } = value[0].account.data.parsed.info;

    expect(uiAmount).to.be.eql(initialSupply);
    // console.log(JSON.parse(value[0].account.data.toString('base64')));
  });
});
