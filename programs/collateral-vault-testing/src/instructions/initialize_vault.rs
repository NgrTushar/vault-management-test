use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer},
};
use crate::state::CollateralVault;
use crate::constants::{VAULT_SEED, MIN_DEPOSIT_AMOUNT};
use crate::errors::ErrorCode;
use crate::events::VaultInitializedEvent;

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = CollateralVault::LEN,
        seeds = [VAULT_SEED, user.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, CollateralVault>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = vault
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeVault>, initial_deposit: u64) -> Result<()> {
    require!(
        initial_deposit >= MIN_DEPOSIT_AMOUNT,
        ErrorCode::DepositBelowMinimum
    );

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_token_account.to_account_info(),
                to: ctx.accounts.vault_token_account.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        initial_deposit,
    )?;

    let vault = &mut ctx.accounts.vault;
    let clock = Clock::get()?;
    let bump = ctx.bumps.vault;

    vault.initialize(
        ctx.accounts.user.key(),
        ctx.accounts.vault_token_account.key(),
        initial_deposit,
        clock.unix_timestamp,
        bump,
    );

    emit!(VaultInitializedEvent {
        vault: vault.key(),
        owner: ctx.accounts.user.key(),
        token_account: ctx.accounts.vault_token_account.key(),
        initial_deposit,
        timestamp: clock.unix_timestamp,
    });

    msg!("✅ Vault initialized for user: {}", ctx.accounts.user.key());
    msg!("Initial deposit: {} tokens", initial_deposit);

    Ok(())
}