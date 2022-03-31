import * as anchor from '@project-serum/anchor';
import { BN, Program } from '@project-serum/anchor';
import { Luckyseven } from '../target/types/luckyseven';
import {
  findAssociatedTokenAddress,
  getTokenMintPublicKey,
} from '../constants';
import { LAMPORTS_PER_SOL, SYSVAR_RENT_PUBKEY } from '@solana/web3.js';
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getMint,
  TOKEN_PROGRAM_ID,
} from '@solana/spl-token';
import { expect } from 'chai';

describe('Tokens', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());
  const program = anchor.workspace.Luckyseven as Program<Luckyseven>;

  const {
    connection,
    wallet: { publicKey: signer },
  } = program.provider;

  const initialSupply = 5_000_000 * LAMPORTS_PER_SOL;
  let tokenMint;
  let signerAssociatedTokenAccount;

  before(async () => {
    tokenMint = await getTokenMintPublicKey();
    signerAssociatedTokenAccount = await findAssociatedTokenAddress(
      signer,
      tokenMint,
    );
    await program.rpc.mintInitialSupply(new BN(initialSupply), {
      accounts: {
        signer,
        signerAssociatedTokenAccount,
        tokenMint,
        systemProgram: anchor.web3.SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      },
    });
  });

  it('should create a mint account and mint initial supply to signer', async () => {
    const { value } = await connection.getParsedTokenAccountsByOwner(signer, {
      mint: tokenMint,
    });

    const {
      tokenAmount: { amount },
    } = value[0].account.data.parsed.info;
    expect(amount).to.be.eql(initialSupply.toString());
    const mint = await getMint(connection, tokenMint);
    expect(mint.mintAuthority).to.be.null;
    expect(tokenMint.toBase58()).to.be.eql(mint.address.toBase58());
  });

  it('should transfer tokens properly', async () => {
    const { publicKey: destination } = anchor.web3.Keypair.generate();

    const destinationAssociatedTokenAccount = await findAssociatedTokenAddress(
      destination,
      tokenMint,
    );
    await program.rpc.transferTokens(new BN(initialSupply / 2), {
      accounts: {
        signer,
        destination,
        tokenMint,
        destinationAssociatedTokenAccount,
        signerAssociatedTokenAccount,
        systemProgram: anchor.web3.SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      },
    });

    const { value: signerBalance } =
      await connection.getParsedTokenAccountsByOwner(signer, {
        mint: tokenMint,
      });
    const {
      tokenAmount: { amount: signerUiAmount },
    } = signerBalance[0].account.data.parsed.info;
    expect(signerUiAmount).to.be.eql(new BN(initialSupply / 2).toString());

    const { value: destinationBalance } =
      await connection.getParsedTokenAccountsByOwner(destination, {
        mint: tokenMint,
      });
    const {
      tokenAmount: { amount: destinationUiAmount },
    } = destinationBalance[0].account.data.parsed.info;
    expect(destinationUiAmount).to.be.eql(new BN(initialSupply / 2).toString());
  });
});
