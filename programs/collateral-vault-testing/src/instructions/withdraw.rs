use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::CollateralVault;
use crate::constants::VAULT_SEED;
use crate::errors::ErrorCode;
use crate::events::WithdrawEvent;

#[derive(Accounts)]
pub struct Withdraw<'info> {
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

pub fn handler(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    require!(amount > 0, ErrorCode::InvalidAmount);

    let vault = &mut ctx.accounts.vault;

    require!(
        vault.available_balance >= amount,
        ErrorCode::InsufficientAvailableBalance
    );
    let user_key=ctx.accounts.user.key();
    let seeds = &[
        VAULT_SEED,
        user_key.as_ref(),
        &[vault.bump],
    ];
    let signer_seeds = &[&seeds[..]];

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault_token_account.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: vault.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
    )?;

    vault.withdraw(amount)?;

    let clock = Clock::get()?;
    emit!(WithdrawEvent {
        vault: vault.key(),
        user: ctx.accounts.user.key(),
        amount,
        new_total_balance: vault.total_balance,
        new_available_balance: vault.available_balance,
        timestamp: clock.unix_timestamp,
    });

    msg!("✅ Withdrew {} tokens", amount);
    msg!("New available balance: {}", vault.available_balance);

    Ok(())
}