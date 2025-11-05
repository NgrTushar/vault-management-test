use anchor_lang::prelude::*;

#[account]
pub struct CollateralVault {
    pub owner: Pubkey,
    pub token_account: Pubkey,
    pub total_balance: u64,
    pub locked_balance: u64,
    pub available_balance: u64,
    pub total_deposited: u64,
    pub total_withdrawn: u64,
    pub created_at: i64,
    pub bump: u8,
}

impl CollateralVault {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 8 + 8 + 8 + 8 + 8 + 1;

    pub fn initialize(
        &mut self,
        owner: Pubkey,
        token_account: Pubkey,
        initial_deposit: u64,
        created_at: i64,
        bump: u8,
    ) {
        self.owner = owner;
        self.token_account = token_account;
        self.total_balance = initial_deposit;
        self.locked_balance = 0;
        self.available_balance = initial_deposit;
        self.total_deposited = initial_deposit;
        self.total_withdrawn = 0;
        self.created_at = created_at;
        self.bump = bump;
    }

    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        self.total_balance = self.total_balance
            .checked_add(amount)
            .ok_or(error!(crate::errors::ErrorCode::ArithmeticOverflow))?;
        self.available_balance = self.available_balance
            .checked_add(amount)
            .ok_or(error!(crate::errors::ErrorCode::ArithmeticOverflow))?;
        self.total_deposited = self.total_deposited
            .checked_add(amount)
            .ok_or(error!(crate::errors::ErrorCode::ArithmeticOverflow))?;
        Ok(())
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        self.total_balance = self.total_balance
            .checked_sub(amount)
            .ok_or(error!(crate::errors::ErrorCode::ArithmeticUnderflow))?;
        self.available_balance = self.available_balance
            .checked_sub(amount)
            .ok_or(error!(crate::errors::ErrorCode::ArithmeticUnderflow))?;
        self.total_withdrawn = self.total_withdrawn
            .checked_add(amount)
            .ok_or(error!(crate::errors::ErrorCode::ArithmeticOverflow))?;
        Ok(())
    }

    pub fn lock(&mut self, amount: u64) -> Result<()> {
        self.locked_balance = self.locked_balance
            .checked_add(amount)
            .ok_or(error!(crate::errors::ErrorCode::ArithmeticOverflow))?;
        self.available_balance = self.available_balance
            .checked_sub(amount)
            .ok_or(error!(crate::errors::ErrorCode::ArithmeticUnderflow))?;
        Ok(())
    }

    pub fn unlock(&mut self, amount: u64) -> Result<()> {
        self.locked_balance = self.locked_balance
            .checked_sub(amount)
            .ok_or(error!(crate::errors::ErrorCode::ArithmeticUnderflow))?;
        self.available_balance = self.available_balance
            .checked_add(amount)
            .ok_or(error!(crate::errors::ErrorCode::ArithmeticOverflow))?;
        Ok(())
    }
}