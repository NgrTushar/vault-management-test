use anchor_lang::prelude::*;

#[event]
pub struct VaultInitializedEvent {
    pub vault: Pubkey,
    pub owner: Pubkey,
    pub token_account: Pubkey,
    pub initial_deposit: u64,
    pub timestamp: i64,
}

#[event]
pub struct DepositEvent {
    pub vault: Pubkey,
    pub user: Pubkey,
    pub amount: u64,
    pub new_total_balance: u64,
    pub new_available_balance: u64,
    pub timestamp: i64,
}

#[event]
pub struct WithdrawEvent {
    pub vault: Pubkey,
    pub user: Pubkey,
    pub amount: u64,
    pub new_total_balance: u64,
    pub new_available_balance: u64,
    pub timestamp: i64,
}

#[event]
pub struct LockEvent {
    pub vault: Pubkey,
    pub amount: u64,
    pub new_locked_balance: u64,
    pub new_available_balance: u64,
    pub caller_program: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct UnlockEvent {
    pub vault: Pubkey,
    pub amount: u64,
    pub new_locked_balance: u64,
    pub new_available_balance: u64,
    pub caller_program: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct TransferEvent {
    pub from_vault: Pubkey,
    pub to_vault: Pubkey,
    pub amount: u64,
    pub caller_program: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct ProgramAuthorizedEvent {
    pub program_id: Pubkey,
    pub admin: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct ProgramDeauthorizedEvent {
    pub program_id: Pubkey,
    pub admin: Pubkey,
    pub timestamp: i64,
}