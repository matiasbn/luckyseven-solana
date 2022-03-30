use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Token, TokenAccount};
use solana_program::native_token::LAMPORTS_PER_SOL;
use solana_program::program_pack::Pack;
use solana_program::system_instruction;
use spl_associated_token_account::create_associated_token_account;

declare_id!("FcbmXvb6x3ahEktJMykvfnv2qKPowC1FcqhxD9aUac68");

#[program]
pub mod luckyseven {
    use super::*;

    pub fn set_program_author(ctx: Context<SetAuthority>) -> Result<()> {
        let author_account: &mut Account<Author> = &mut ctx.accounts.author_account;
        author_account.author = ctx.accounts.owner.key();
        Ok(())
    }

    pub fn initialize(ctx: Context<Initialize>, max_number: i64, target_number: i64) -> Result<()> {
        let program_storage: &mut Account<ProgramStorage> = &mut ctx.accounts.program_storage;
        // let authority_account: &mut Account<Authority> = &mut ctx.accounts.authority_account;
        // let signer: &mut Signer = &mut ctx.accounts.owner;
        // require!(signer == ctx.accounts.authority_account)
        program_storage.initialized = true;
        program_storage.winner_difference = max_number;
        program_storage.max_number = max_number;
        program_storage.target_number = target_number;

        Ok(())
    }

    pub fn get_number(ctx: Context<GetNumber>) -> Result<()> {
        let program_storage: &mut Account<ProgramStorage> = &mut ctx.accounts.program_storage;
        require!(program_storage.initialized, ErrorCode::NotInitializedYet);
        let random_number: &mut Account<RandomNumber> = &mut ctx.accounts.random_number;
        let owner: &Signer = &ctx.accounts.owner;
        let clock: Clock = Clock::get().unwrap();
        random_number.number = clock.unix_timestamp % program_storage.max_number;
        random_number.owner = *owner.key;

        let difference: i64 = (random_number.number - program_storage.target_number).abs();
        if difference < program_storage.winner_difference {
            program_storage.winner_difference = difference;
            random_number.winner = true;
        }

        Ok(())
    }

    pub fn create_mint_account(ctx: Context<CreateMintAccount>) -> Result<()> {
        let signer = &ctx.accounts.signer;
        let token_mint = &ctx.accounts.token_mint;
        let rent = &ctx.accounts.rent;
        let system_program_info = &ctx.accounts.system_program;
        let token_program = &ctx.accounts.token_program;
        let associated_token_account = &ctx.accounts.associated_token_account;

        let (mint_address, mint_bump_seed) = Pubkey::find_program_address(&[br"TokenMint"], &id());
        let mint_signer_seeds: &[&[_]] = &[br"TokenMint", &[mint_bump_seed]];

        msg!("Create mint account");
        solana_program::program::invoke_signed(
            &system_instruction::create_account(
                signer.key,
                &token_mint.key(),
                1.max(rent.minimum_balance(spl_token::state::Mint::get_packed_len())),
                spl_token::state::Mint::get_packed_len() as u64,
                &spl_token::id(),
            ),
            &[
                signer.to_account_info().clone(),
                token_mint.to_account_info().clone(),
                system_program_info.to_account_info().clone(),
            ],
            &[mint_signer_seeds],
        )?;

        msg!("Initialize token mint account");
        solana_program::program::invoke(
            &spl_token::instruction::initialize_mint(
                &spl_token::id(),
                &token_mint.key(),
                &token_mint.key,
                Some(token_mint.key),
                spl_token::native_mint::DECIMALS,
            )?,
            &[
                token_mint.to_account_info().clone(),
                token_program.to_account_info().clone(),
                rent.to_account_info().clone(),
            ],
        )?;

        msg!("Create token account for signer");
        solana_program::program::invoke(
            &create_associated_token_account(signer.key, signer.key, token_mint.key),
            &[
                signer.to_account_info().clone(),
                associated_token_account.to_account_info().clone(),
                signer.to_account_info().clone(),
                token_mint.to_account_info().clone(),
                system_program_info.to_account_info().clone(),
                token_program.to_account_info().clone(),
                rent.to_account_info().clone(),
            ],
        )?;

        // msg!("Mint tokens to signer");
        // solana_program::program::invoke_signed(
        //     &spl_token::instruction::mint_to(
        //         &spl_token::id(),
        //         token_mint.key,
        //         associated_token_account.key, // Account
        //         token_mint.key,               // Owner
        //         &[],
        //         5000000 * LAMPORTS_PER_SOL,
        //     )?,
        //     &[
        //         token_mint.clone(),
        //         associated_token_account.to_account_info().clone(),
        //         token_program.to_account_info().clone(),
        //     ],
        //     &[mint_signer_seeds],
        // )?;
        Ok(())
    }

    // pub fn mint_tokens(ctx: Context<CreateMintAccount>) -> Result<()> {
    //     Ok(())
    // }
}

#[derive(Accounts)]
pub struct CreateMintAccount<'info> {
    #[account(mut, seeds = [br"TokenMint"] , bump)]
    /// CHECK: this is not unsafe because we create the account into the function
    pub token_mint: AccountInfo<'info>,
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(address = system_program::ID)]
    /// CHECK: this is not unsafe because we check that the account is indeed system_program
    pub system_program: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    #[account(mut)]
    /// CHECK: this is not unsafe because we create the account into the function
    pub associated_token_account: AccountInfo<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut, seeds = [br"TokenMint"] , bump)]
    /// CHECK: this is not unsafe because we create the account into the function
    pub token_mint: AccountInfo<'info>,
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(address = system_program::ID)]
    /// CHECK: this is not unsafe because we check that the account is indeed system_program
    pub system_program: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct SetAuthority<'info> {
    #[account(init, payer = owner , space = 200)]
    pub author_account: Account<'info, Author>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(address = system_program::ID)]
    /// CHECK: this is not unsafe because we check that the account is indeed system_program
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = owner, space = 200)]
    pub program_storage: Account<'info, ProgramStorage>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account()]
    pub authority_account: Account<'info, Author>,
    #[account(address = system_program::ID)]
    /// CHECK: this is not unsafe because we check that the account is indeed system_program
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct GetNumber<'info> {
    #[account(init, payer = owner, space = 200)]
    pub random_number: Account<'info, RandomNumber>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub program_storage: Account<'info, ProgramStorage>,
    #[account(address = system_program::ID)]
    /// CHECK: this is not unsafe because we check that the account is indeed system_program
    pub system_program: AccountInfo<'info>,
}

#[account]
#[derive(Default)]
pub struct Author {
    pub author: Pubkey,
}

#[account]
#[derive(Default)]
pub struct RandomNumber {
    pub owner: Pubkey,
    pub number: i64,
    pub winner: bool,
}

#[account]
#[derive(Default)]
pub struct ProgramStorage {
    pub winner_difference: i64,
    pub target_number: i64,
    pub max_number: i64,
    pub initialized: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Already initialized.")]
    AlreadyInitialized,
    #[msg("Not initialized yet.")]
    NotInitializedYet,
}
