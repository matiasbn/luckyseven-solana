use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

const MAX_VALUE: i64 = 1234;

#[program]
pub mod luckyseven {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let program_storage: &mut Account<ProgramStorage> = &mut ctx.accounts.program_storage;
        let deployer: &mut Signer = &mut ctx.accounts.owner;
        program_storage.initialized = true;
        program_storage.last_difference = MAX_VALUE;
        program_storage.game_owner = *deployer.key;

        Ok(())
    }

    pub fn store_number(ctx: Context<StoreNumber>) -> Result<()> {
        let program_storage: &mut Account<ProgramStorage> = &mut ctx.accounts.program_storage;
        require!(program_storage.initialized, ErrorCode::NotInitializedYet);
        let random_number: &mut Account<RandomNumber> = &mut ctx.accounts.random_number;
        let owner: &Signer = &ctx.accounts.owner;
        let clock: Clock = Clock::get().unwrap();
        random_number.number = clock.unix_timestamp % MAX_VALUE;
        random_number.owner = *owner.key;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = owner)]
    pub program_storage: Account<'info, ProgramStorage>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(address = system_program::ID)]
    /// CHECK: this is not unsafe because we check that the account is indeed system_program
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct StoreNumber<'info> {
    #[account(init, payer = owner, space = RandomNumber::LEN)]
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
pub struct RandomNumber {
    pub owner: Pubkey,
    pub number: i64,
    pub winner: bool,
}

#[account]
#[derive(Default)]
pub struct ProgramStorage {
    pub last_difference: i64,
    pub initialized: bool,
    pub game_owner: Pubkey,
}

const DISCRIMINATOR_LENGTH: usize = 8;
const OWNER_LENGTH: usize = 32;
const NUMBER_LENGTH: usize = 32;

impl RandomNumber {
    const LEN: usize = DISCRIMINATOR_LENGTH + OWNER_LENGTH + NUMBER_LENGTH;
}

#[error_code]
pub enum ErrorCode {
    #[msg("Already initialized.")]
    AlreadyInitialized,
    #[msg("Not initialized yet.")]
    NotInitializedYet,
}
