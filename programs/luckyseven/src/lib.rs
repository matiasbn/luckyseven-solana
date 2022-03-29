use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("FcbmXvb6x3ahEktJMykvfnv2qKPowC1FcqhxD9aUac68");

#[program]
pub mod luckyseven {
    use super::*;
    use std::cmp::min;

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

    pub fn create_mint_authority(ctx: Context<CreateMintAuthority>) -> Result<()> {
        let (mint_address, mint_bump_seed) = Pubkey::find_program_address(&[br"TokenMint"], &id());
        let mint_signer_seeds: &[&[_]] = &[br"TokenMint", &[mint_bump_seed]];
        msg!("asdasdad {}", mint_address);
        msg!("{}", mint_bump_seed);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateMintAuthority<'info> {
    #[account(mut, seeds = [br"TokenMint"] , bump)]
    /// CHECK: this is not unsafe because we create the account into the function
    pub token_mint: AccountInfo<'info>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(address = system_program::ID)]
    /// CHECK: this is not unsafe because we check that the account is indeed system_program
    pub system_program: AccountInfo<'info>,
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
