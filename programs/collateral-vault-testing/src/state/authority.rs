use anchor_lang::prelude::*;
use crate::constants::MAX_AUTHORIZED_PROGRAMS;

#[account]
pub struct VaultAuthority {
    pub admin: Pubkey,
    pub authorized_programs: Vec<Pubkey>,
    pub bump: u8,
}

impl VaultAuthority {
    pub const LEN: usize = 8 + 32 + 4 + (32 * MAX_AUTHORIZED_PROGRAMS) + 1;

    pub fn initialize(&mut self, admin: Pubkey, bump: u8) {
        self.admin = admin;
        self.authorized_programs = Vec::new();
        self.bump = bump;
    }

    pub fn is_authorized(&self, program_id: &Pubkey) -> bool {
        self.authorized_programs.contains(program_id)
    }

    pub fn add_program(&mut self, program_id: Pubkey) -> Result<()> {
        require!(
            !self.is_authorized(&program_id),
            crate::errors::ErrorCode::ProgramAlreadyAuthorized
        );
        require!(
            self.authorized_programs.len() < MAX_AUTHORIZED_PROGRAMS,
            crate::errors::ErrorCode::MaxAuthorizedProgramsReached
        );
        self.authorized_programs.push(program_id);
        Ok(())
    }

    pub fn remove_program(&mut self, program_id: Pubkey) -> Result<()> {
        let position = self.authorized_programs
            .iter()
            .position(|&id| id == program_id)
            .ok_or(crate::errors::ErrorCode::ProgramNotAuthorized)?;
        self.authorized_programs.remove(position);
        Ok(())
    }
}