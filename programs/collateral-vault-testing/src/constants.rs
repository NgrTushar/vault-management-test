// Constants for the Collateral Vault Program

/// Seed for vault PDA derivation
pub const VAULT_SEED: &[u8] = b"vault";

/// Seed for authority PDA derivation
pub const AUTHORITY_SEED: &[u8] = b"authority";

/// Maximum number of authorized programs
pub const MAX_AUTHORIZED_PROGRAMS: usize = 20;

/// Minimum deposit amount (1 token with 6 decimals)
pub const MIN_DEPOSIT_AMOUNT: u64 = 1_000_000;

/// Token decimals
pub const TOKEN_DECIMALS: u8 = 6;