use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod luckyseven {
    use super::*;

    pub fn set_authority(ctx: Context<SetAuthority>) -> Result<()> {
        let authority_account: &mut Account<Authority> = &mut ctx.accounts.authority_account;
        authority_account.authority = ctx.accounts.owner.key();
        Ok(())
    }

    pub fn initialize(ctx: Context<Initialize>, max_number: i64, target_number: i64) -> Result<()> {
        let program_storage: &mut Account<ProgramStorage> = &mut ctx.accounts.program_storage;
        // let authority_account: Account<Authority> = *ctx.accounts.authority_account;
        // let signer: &mut Signer = &mut ctx.accounts.owner;
        // require!(signer == authority_account.)
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
}

#[derive(Accounts)]
pub struct SetAuthority<'info> {
    #[account(init, payer = owner)]
    pub authority_account: Account<'info, Authority>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(address = system_program::ID)]
    /// CHECK: this is not unsafe because we check that the account is indeed system_program
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = owner)]
    pub program_storage: Account<'info, ProgramStorage>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account()]
    pub authority_account: Account<'info, Authority>,
    #[account(address = system_program::ID)]
    /// CHECK: this is not unsafe because we check that the account is indeed system_program
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct GetNumber<'info> {
    #[account(init, payer = owner)]
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
pub struct Authority {
    pub authority: Pubkey,
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
