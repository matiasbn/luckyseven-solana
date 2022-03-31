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
  getMint,
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
    const tokenMint = await getTokenMintPublicKey();

    const associatedTokenAccount = await findAssociatedTokenAddress(
      signer,
      tokenMint,
    );
    const initialSupply = 5_000_000;
    await program.rpc.mintInitialSupply(new BN(initialSupply), {
      accounts: {
        signer,
        associatedTokenAccount,
        tokenMint,
        systemProgram: anchor.web3.SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      },
    });

    const { value } = await connection.getParsedTokenAccountsByOwner(signer, {
      mint: tokenMint,
    });

    const {
      tokenAmount: { uiAmount },
    } = value[0].account.data.parsed.info;

    expect(uiAmount).to.be.eql(initialSupply);
    const mint = await getMint(connection, tokenMint);
    expect(mint.mintAuthority).to.be.null;
    expect(tokenMint.toBase58()).to.be.eql(mint.address);
  });
});
