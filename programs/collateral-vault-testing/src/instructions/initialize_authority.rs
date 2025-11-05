use anchor_lang::prelude::*;
use crate::state::VaultAuthority;
use crate::constants::AUTHORITY_SEED;

#[derive(Accounts)]
pub struct InitializeAuthority<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = VaultAuthority::LEN,
        seeds = [AUTHORITY_SEED],
        bump
    )]
    pub authority: Account<'info, VaultAuthority>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeAuthority>) -> Result<()> {
    let authority = &mut ctx.accounts.authority;
    let bump = ctx.bumps.authority;

    authority.initialize(ctx.accounts.admin.key(), bump);

    msg!("✅ Vault Authority initialized");
    msg!("Admin: {}", ctx.accounts.admin.key());

    Ok(())
}