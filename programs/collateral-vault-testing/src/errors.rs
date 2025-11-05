use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid amount: must be greater than 0")]
    InvalidAmount,

    #[msg("Deposit amount below minimum required (1 token)")]
    DepositBelowMinimum,

    #[msg("Insufficient available balance")]
    InsufficientAvailableBalance,

    #[msg("Insufficient locked balance")]
    InsufficientLockedBalance,

    #[msg("Unauthorized: only vault owner can perform this action")]
    UnauthorizedOwner,

    #[msg("Unauthorized: only admin can perform this action")]
    UnauthorizedAdmin,

    #[msg("Unauthorized program: not in authorized list")]
    UnauthorizedProgram,

    #[msg("Program already authorized")]
    ProgramAlreadyAuthorized,

    #[msg("Program not found in authorized list")]
    ProgramNotAuthorized,

    #[msg("Maximum authorized programs reached")]
    MaxAuthorizedProgramsReached,

    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,

    #[msg("Arithmetic underflow")]
    ArithmeticUnderflow,

    #[msg("Cannot transfer to same vault")]
    SameVaultTransfer,
}