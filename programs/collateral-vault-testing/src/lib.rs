use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

pub use state::{CollateralVault, VaultAuthority};


use instructions::*;

declare_id!("3H9kFFeZZZpaqaTv1qdZfz9odsguQjysYUQ8ELLJ8Pqp");

#[program]
pub mod collateral_vault {
    use super::*;

    pub fn initialize_authority(ctx: Context<InitializeAuthority>) -> Result<()> {
        instructions::initialize_authority::handler(ctx)
    }

    pub fn initialize_vault(
        ctx: Context<InitializeVault>,
        initial_deposit: u64,
    ) -> Result<()> {
        instructions::initialize_vault::handler(ctx, initial_deposit)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::deposit::handler(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        instructions::withdraw::handler(ctx, amount)
    }

    pub fn lock_collateral(ctx: Context<LockCollateral>, amount: u64) -> Result<()> {
        instructions::lock_collateral::handler(ctx, amount)
    }

    pub fn unlock_collateral(ctx: Context<UnlockCollateral>, amount: u64) -> Result<()> {
        instructions::unlock_collateral::handler(ctx, amount)
    }

    pub fn transfer_collateral(
        ctx: Context<TransferCollateral>,
        amount: u64,
    ) -> Result<()> {
        instructions::transfer_collateral::handler(ctx, amount)
    }

    pub fn add_authorized_program(
        ctx: Context<AddAuthorizedProgram>,
        program_id: Pubkey,
    ) -> Result<()> {
        instructions::manage_authority::add_authorized_program(ctx, program_id)
    }

    pub fn remove_authorized_program(
        ctx: Context<RemoveAuthorizedProgram>,
        program_id: Pubkey,
    ) -> Result<()> {
        instructions::manage_authority::remove_authorized_program(ctx, program_id)
    }
}