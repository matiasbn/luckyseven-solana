import { PublicKey } from '@solana/web3.js';

export const LUCKYSEVEN_PUBLIC_KEY =
  'FcbmXvb6x3ahEktJMykvfnv2qKPowC1FcqhxD9aUac68';
export const LUCKYSEVEN_PROGRAM_PUBLICKEY = new PublicKey(
  LUCKYSEVEN_PUBLIC_KEY,
);
export const TOKEN_MINT_SEED = 'TokenMint';

const enc = new TextEncoder();

export async function getTokenMintPublicKey(): Promise<PublicKey> {
  const [findMintPublicKey] = await PublicKey.findProgramAddress(
    [enc.encode(TOKEN_MINT_SEED)],
    LUCKYSEVEN_PROGRAM_PUBLICKEY,
  );
  return findMintPublicKey;
}
