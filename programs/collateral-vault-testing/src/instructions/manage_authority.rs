use anchor_lang::prelude::*;
use crate::state::VaultAuthority;
use crate::constants::AUTHORITY_SEED;
use crate::errors::ErrorCode;
use crate::events::{ProgramAuthorizedEvent, ProgramDeauthorizedEvent};

#[derive(Accounts)]
pub struct AddAuthorizedProgram<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [AUTHORITY_SEED],
        bump = authority.bump,
        has_one = admin @ ErrorCode::UnauthorizedAdmin
    )]
    pub authority: Account<'info, VaultAuthority>,
}

#[derive(Accounts)]
pub struct RemoveAuthorizedProgram<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [AUTHORITY_SEED],
        bump = authority.bump,
        has_one = admin @ ErrorCode::UnauthorizedAdmin
    )]
    pub authority: Account<'info, VaultAuthority>,
}

pub fn add_authorized_program(
    ctx: Context<AddAuthorizedProgram>,
    program_id: Pubkey,
) -> Result<()> {
    let authority = &mut ctx.accounts.authority;
    authority.add_program(program_id)?;

    let clock = Clock::get()?;
    emit!(ProgramAuthorizedEvent {
        program_id,
        admin: ctx.accounts.admin.key(),
        timestamp: clock.unix_timestamp,
    });

    msg!("✅ Authorized program: {}", program_id);

    Ok(())
}

pub fn remove_authorized_program(
    ctx: Context<RemoveAuthorizedProgram>,
    program_id: Pubkey,
) -> Result<()> {
    let authority = &mut ctx.accounts.authority;
    authority.remove_program(program_id)?;

    let clock = Clock::get()?;
    emit!(ProgramDeauthorizedEvent {
        program_id,
        admin: ctx.accounts.admin.key(),
        timestamp: clock.unix_timestamp,
    });

    msg!("✅ Deauthorized program: {}", program_id);

    Ok(())
}