use anchor_lang::prelude::*;
use crate::state::{CollateralVault, VaultAuthority};
use crate::constants::{VAULT_SEED, AUTHORITY_SEED};
use crate::errors::ErrorCode;
use crate::events::UnlockEvent;

#[derive(Accounts)]
pub struct UnlockCollateral<'info> {
    #[account(
        seeds = [AUTHORITY_SEED],
        bump = authority.bump
    )]
    pub authority: Account<'info, VaultAuthority>,

    #[account(
        mut,
        seeds = [VAULT_SEED, vault.owner.as_ref()],
        bump = vault.bump
    )]
    pub vault: Account<'info, CollateralVault>,

    /// CHECK: Vault owner for validation
    pub vault_owner: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<UnlockCollateral>, amount: u64) -> Result<()> {
    require!(amount > 0, ErrorCode::InvalidAmount);

    let vault = &mut ctx.accounts.vault;

    require!(
        vault.owner == ctx.accounts.vault_owner.key(),
        ErrorCode::UnauthorizedOwner
    );

    require!(
        vault.locked_balance >= amount,
        ErrorCode::InsufficientLockedBalance
    );

    vault.unlock(amount)?;

    let clock = Clock::get()?;
    emit!(UnlockEvent {
        vault: vault.key(),
        amount,
        new_locked_balance: vault.locked_balance,
        new_available_balance: vault.available_balance,
        caller_program: *ctx.program_id,
        timestamp: clock.unix_timestamp,
    });

    msg!("✅ Unlocked {} tokens", amount);
    msg!("New available balance: {}", vault.available_balance);

    Ok(())
}