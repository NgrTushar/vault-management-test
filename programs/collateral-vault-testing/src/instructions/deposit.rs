use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::CollateralVault;
use crate::constants::VAULT_SEED;
use crate::errors::ErrorCode;
use crate::events::DepositEvent;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [VAULT_SEED, user.key().as_ref()],
        bump = vault.bump,
        constraint= vault.owner == user.key() @ ErrorCode::UnauthorizedOwner
    )]
    pub vault: Account<'info, CollateralVault>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = vault_token_account.key() == vault.token_account @ ErrorCode::UnauthorizedOwner
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    require!(amount > 0, ErrorCode::InvalidAmount);

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_token_account.to_account_info(),
                to: ctx.accounts.vault_token_account.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        amount,
    )?;

    let vault = &mut ctx.accounts.vault;
    vault.deposit(amount)?;

    let clock = Clock::get()?;
    emit!(DepositEvent {
        vault: vault.key(),
        user: ctx.accounts.user.key(),
        amount,
        new_total_balance: vault.total_balance,
        new_available_balance: vault.available_balance,
        timestamp: clock.unix_timestamp,
    });

    msg!("✅ Deposited {} tokens", amount);
    msg!("New total balance: {}", vault.total_balance);

    Ok(())
}