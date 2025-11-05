use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::{CollateralVault, VaultAuthority};
use crate::constants::{VAULT_SEED, AUTHORITY_SEED};
use crate::errors::ErrorCode;
use crate::events::TransferEvent;

#[derive(Accounts)]
pub struct TransferCollateral<'info> {
    #[account(
        seeds = [AUTHORITY_SEED],
        bump = authority.bump
    )]
    pub authority: Account<'info, VaultAuthority>,

    #[account(
        mut,
        seeds = [VAULT_SEED, from_vault.owner.as_ref()],
        bump = from_vault.bump
    )]
    pub from_vault: Account<'info, CollateralVault>,

    #[account(
        mut,
        seeds = [VAULT_SEED, to_vault.owner.as_ref()],
        bump = to_vault.bump
    )]
    pub to_vault: Account<'info, CollateralVault>,

    #[account(
        mut,
        constraint = from_vault_token_account.key() == from_vault.token_account
    )]
    pub from_vault_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = to_vault_token_account.key() == to_vault.token_account
    )]
    pub to_vault_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<TransferCollateral>, amount: u64) -> Result<()> {
    require!(amount > 0, ErrorCode::InvalidAmount);

    require!(
        ctx.accounts.from_vault.key() != ctx.accounts.to_vault.key(),
        ErrorCode::SameVaultTransfer
    );

    let from_vault = &mut ctx.accounts.from_vault;

    require!(
        from_vault.available_balance >= amount,
        ErrorCode::InsufficientAvailableBalance
    );

    let from_vault_owner = from_vault.owner;
    let seeds = &[
        VAULT_SEED,
        from_vault_owner.as_ref(),
        &[from_vault.bump],
    ];
    let signer_seeds = &[&seeds[..]];

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.from_vault_token_account.to_account_info(),
                to: ctx.accounts.to_vault_token_account.to_account_info(),
                authority: from_vault.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
    )?;

    from_vault.withdraw(amount)?;

    let to_vault = &mut ctx.accounts.to_vault;
    to_vault.deposit(amount)?;

    let clock = Clock::get()?;
    emit!(TransferEvent {
        from_vault: from_vault.key(),
        to_vault: to_vault.key(),
        amount,
        caller_program: *ctx.program_id,
        timestamp: clock.unix_timestamp,
    });

    msg!("✅ Transferred {} tokens", amount);
    msg!("From vault: {}", from_vault.key());
    msg!("To vault: {}", to_vault.key());

    Ok(())
}