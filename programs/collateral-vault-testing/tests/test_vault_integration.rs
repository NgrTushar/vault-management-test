// This is your new, working test file.
// Run this with `cargo test-sbf`

// Use the common helper module
mod common;
use common::CollateralVaultProgramTest;

use anchor_lang::prelude::{AccountDeserialize, ErrorCode};
use solana_program_test_2::BanksClientError;
use solana_sdk_2::transport::TransportError;
use collateral_vault_testing::errors;

// Use tokio::test for async tests
#[tokio::test]
async fn test_initialize_vault_success() {
    // 1. Setup
    let mut test = CollateralVaultProgramTest::new().await;
    let user_pubkey = test.user_pubkey();
    
    // Create ATAs
    let user_ata = test.create_and_fund_user_ata(&user_pubkey).await;
    let (vault_pda, _vault_bump) = test.find_vault_pda(&user_pubkey);
    let vault_ata = test.create_token_account(&vault_pda).await;
    
    let initial_deposit = 100_000_000; // 100 USDT

    // 2. Initialize Authority
    let init_auth_ix = test.initialize_authority_ix();
    test.process_transaction(&[init_auth_ix], &[])
        .await
        .unwrap();

    // 3. Initialize Vault
    let init_vault_ix = test.initialize_vault_ix(
        &user_pubkey,
        &vault_pda,
        &vault_ata,
        &user_ata,
        initial_deposit,
    );
    
    // The user must sign this transaction
    let result = test
        .process_transaction(&[init_vault_ix], &[&test.user_keypair])
        .await;
    
    assert!(result.is_ok(), "Transaction failed: {:?}", result.err());

    // 4. Verify
    let vault_state = test.get_vault_account(&vault_pda).await;
    let vault_token_balance = test.get_token_balance(&vault_ata).await;
    let user_token_balance = test.get_token_balance(&user_ata).await;
    let clock = test.get_clock().await;

    assert_eq!(vault_state.owner, user_pubkey);
    assert_eq!(vault_state.token_account, vault_ata);
    assert_eq!(vault_state.total_balance, initial_deposit);
    assert_eq!(vault_state.locked_balance, 0);
    assert_eq!(vault_state.available_balance, initial_deposit);
    assert_eq!(vault_state.total_deposited, initial_deposit);
    assert_eq!(vault_state.total_withdrawn, 0);
    assert_eq!(vault_state.created_at, clock.unix_timestamp);

    assert_eq!(vault_token_balance, initial_deposit);
    assert_eq!(user_token_balance, common::USER_STARTING_USDT - initial_deposit);
}


#[tokio::test]
async fn test_deposit_success() {
    // 1. Setup (Initialize Vault first)
    let mut test = CollateralVaultProgramTest::new().await;
    let user_pubkey = test.user_pubkey();
    let user_ata = test.create_and_fund_user_ata(&user_pubkey).await;
    let (vault_pda, _vault_bump) = test.find_vault_pda(&user_pubkey);
    let vault_ata = test.create_token_account(&vault_pda).await;
    let initial_deposit = 100_000_000; // 100 USDT

    // Init Authority
    let init_auth_ix = test.initialize_authority_ix();
    test.process_transaction(&[init_auth_ix], &[])
        .await
        .unwrap();

    // Init Vault
    let init_vault_ix = test.initialize_vault_ix(
        &user_pubkey,
        &vault_pda,
        &vault_ata,
        &user_ata,
        initial_deposit,
    );
    test.process_transaction(&[init_vault_ix], &[&test.user_keypair])
        .await
        .unwrap();

    // 2. Test Deposit
    let deposit_amount = 50_000_000; // 50 USDT
    let deposit_ix = test.deposit_ix(
        &user_pubkey,
        &user_ata,
        &vault_pda,
        &vault_ata,
        deposit_amount,
    );

    let result = test
        .process_transaction(&[deposit_ix], &[&test.user_keypair])
        .await;

    assert!(result.is_ok(), "Transaction failed: {:?}", result.err());

    // 4. Verify
    let vault_state = test.get_vault_account(&vault_pda).await;
    let vault_token_balance = test.get_token_balance(&vault_ata).await;
    let user_token_balance = test.get_token_balance(&user_ata).await;

    let expected_total = initial_deposit + deposit_amount;
    assert_eq!(vault_state.total_balance, expected_total);
    assert_eq!(vault_state.available_balance, expected_total);
    assert_eq!(vault_state.total_deposited, expected_total);
    assert_eq!(vault_state.locked_balance, 0);

    assert_eq!(vault_token_balance, expected_total);
    assert_eq!(user_token_balance, common::USER_STARTING_USDT - expected_total);
}

#[tokio::test]
async fn test_deposit_error_zero_amount() {
    // 1. Setup (Initialize Vault)
    let mut test = CollateralVaultProgramTest::new().await;
    let user_pubkey = test.user_pubkey();
    let user_ata = test.create_and_fund_user_ata(&user_pubkey).await;
    let (vault_pda, _vault_bump) = test.find_vault_pda(&user_pubkey);
    let vault_ata = test.create_token_account(&vault_pda).await;

    let init_auth_ix = test.initialize_authority_ix();
    let init_vault_ix = test.initialize_vault_ix(&user_pubkey, &vault_pda, &vault_ata, &user_ata, 100);
    test.process_transaction(&[init_auth_ix, init_vault_ix], &[&test.user_keypair])
        .await
        .unwrap();

    // 2. Test Deposit with 0
    let deposit_amount = 0;
    let deposit_ix = test.deposit_ix(
        &user_pubkey,
        &user_ata,
        &vault_pda,
        &vault_ata,
        deposit_amount,
    );

    let result = test
        .process_transaction(&[deposit_ix], &[&test.user_keypair])
        .await;

    // 3. Verify
    assert!(result.is_err());
    let err = result.unwrap_err();
    
    // Check for specific program error
    match err {
        BanksClientError::TransactionError(TransportError::TransactionError(tx_err)) => {
            let inner_err = tx_err.unwrap_instruction_error();
            match inner_err {
                solana_sdk_2::transaction::TransactionError::InstructionError(_, ix_err) => {
                     let program_error = u32::from(ix_err);
                     assert_eq!(program_error, errors::ErrorCode::InvalidAmount.into());
                }
                _ => panic!("Wrong instruction error type"),
            }
        }
        _ => panic!("Wrong error type: {:?}", err),
    }
}