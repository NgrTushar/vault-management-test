use anchor_lang::{
    prelude::*,
    solana_program::{system_program, program_pack::Pack},
    InstructionData, ToAccountMetas,
};
use collateral_vault_testing::{
    self,
    constants::{AUTHORITY_SEED, VAULT_SEED},
    state::{CollateralVault, VaultAuthority},
};

// Use the Solana 2.0 library versions
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    clock::Clock,
    hash::Hash,
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use spl_token_2::{
    self,
    state::{Account as TokenAccount, Mint},
};

pub const USER_STARTING_USDT: u64 = 1_000_000_000; // 1000 USDT with 6 decimals

pub struct CollateralVaultProgramTest {
    pub context: ProgramTestContext,
    pub program_id: Pubkey,
    pub user_keypair: Keypair,
    pub usdt_mint: Pubkey,
    pub authority_pda: Pubkey,
    pub authority_bump: u8,
}

impl CollateralVaultProgramTest {
    pub async fn new() -> Self {
        let program_id = collateral_vault_testing::id();
        let mut pt = ProgramTest::new(
            "collateral_vault_testing",
            program_id,
            processor!(collateral_vault_testing::entry),
        );

        // Add SPL Token program
        pt.add_program(
            "spl_token_2",
            spl_token_2::id(),
            processor!(spl_token_2::processor::Processor::process),
        );

        // Add user account
        let user_keypair = Keypair::new();
        pt.add_account(
            user_keypair.pubkey(),
            Account {
                lamports: 100 * 1_000_000_000, // 100 SOL
                data: vec![],
                owner: system_program::id(),
                executable: false,
                rent_epoch: 0,
            },
        );

        // Add USDT Mint
        let usdt_mint = Keypair::new();
        pt.add_account(
            usdt_mint.pubkey(),
            create_mint_account(&user_keypair.pubkey(), 6),
        );

        // Start the test context
        let mut context = pt.start_with_context().await;

        let (authority_pda, authority_bump) =
            Pubkey::find_program_address(&[AUTHORITY_SEED], &program_id);

        Self {
            context,
            program_id,
            user_keypair,
            usdt_mint: usdt_mint.pubkey(),
            authority_pda,
            authority_bump,
        }
    }

    pub fn user_pubkey(&self) -> Pubkey {
        self.user_keypair.pubkey()
    }

    pub fn find_vault_pda(&self, user: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[VAULT_SEED, user.as_ref()], &self.program_id)
    }

    pub async fn create_token_account(&mut self, owner: &Pubkey) -> Pubkey {
        let token_account = Keypair::new();
        let rent = self.context.banks_client.get_rent().await.unwrap();
        let rent = rent.minimum_balance(spl_token_2::state::Account::LEN);

        let tx = Transaction::new_signed_with_payer(
            &[
                system_program::create_account(
                    &self.context.payer.pubkey(),
                    &token_account.pubkey(),
                    rent,
                    spl_token_2::state::Account::LEN as u64,
                    &spl_token_2::id(),
                ),
                spl_token_2::instruction::initialize_account(
                    &spl_token_2::id(),
                    &token_account.pubkey(),
                    &self.usdt_mint,
                    owner,
                )
                .unwrap(),
            ],
            Some(&self.context.payer.pubkey()),
            &[&self.context.payer, &token_account],
            self.context.last_blockhash,
        );
        self.context
            .banks_client
            .process_transaction(tx)
            .await
            .unwrap();
        token_account.pubkey()
    }

    pub async fn mint_tokens(&mut self, token_account: &Pubkey, amount: u64) {
        let tx = Transaction::new_signed_with_payer(
            &[spl_token_2::instruction::mint_to(
                &spl_token_2::id(),
                &self.usdt_mint,
                token_account,
                &self.user_keypair.pubkey(), // Mint authority
                &[],
                amount,
            )
            .unwrap()],
            Some(&self.context.payer.pubkey()),
            &[&self.context.payer, &self.user_keypair], // Mint authority must sign
            self.context.last_blockhash,
        );
        self.context
            .banks_client
            .process_transaction(tx)
            .await
            .unwrap();
    }

    pub async fn create_and_fund_user_ata(&mut self, user: &Pubkey) -> Pubkey {
        let user_ata = self.create_token_account(user).await;
        self.mint_tokens(&user_ata, USER_STARTING_USDT).await;
        user_ata
    }

    pub async fn get_account_data(&mut self, pubkey: &Pubkey) -> Option<Vec<u8>> {
        self.context
            .banks_client
            .get_account(*pubkey)
            .await
            .expect("Failed to get account")
            .map(|a| a.data)
    }

    pub async fn get_vault_account(&mut self, vault_pda: &Pubkey) -> CollateralVault {
        let data = self.get_account_data(vault_pda).await.unwrap();
        CollateralVault::try_from_slice(&data[8..]).unwrap()
    }

    pub async fn get_authority_account(&mut self) -> VaultAuthority {
        let data = self.get_account_data(&self.authority_pda).await.unwrap();
        VaultAuthority::try_from_slice(&data[8..]).unwrap()
    }

    pub async fn get_token_balance(&mut self, token_account_pubkey: &Pubkey) -> u64 {
        let data = self.get_account_data(token_account_pubkey).await.unwrap();
        let token_account = spl_token_2::state::Account::unpack(&data).unwrap();
        token_account.amount
    }

    pub async fn warp_to_slot(&mut self, slot: u64) {
        self.context.warp_to_slot(slot).await.unwrap();
    }
    
    pub async fn get_clock(&mut self) -> Clock {
        self.context.banks_client.get_clock().await.unwrap()
    }

    pub async fn process_transaction(
        &mut self,
        instructions: &[Instruction],
        signers: &[&Keypair],
    ) -> Result<(), BanksClientError> {
        let mut all_signers = vec![&self.context.payer];
        all_signers.extend(signers);

        let tx = Transaction::new_signed_with_payer(
            instructions,
            Some(&self.context.payer.pubkey()),
            &all_signers,
            self.context.last_blockhash,
        );
        self.context.banks_client.process_transaction(tx).await
    }

    // --- Instruction Helper ---

    pub fn initialize_authority_ix(&self) -> Instruction {
        collateral_vault_testing::instruction::InitializeAuthority {}
            .to_instruction(
                collateral_vault_testing::accounts::InitializeAuthority {
                    admin: self.context.payer.pubkey(), // Use test payer as admin
                    authority: self.authority_pda,
                    system_program: system_program::id(),
                },
            )
            .unwrap()
    }
    
    pub fn initialize_vault_ix(
        &self,
        user: &Pubkey,
        vault_pda: &Pubkey,
        vault_token_account: &Pubkey,
        user_token_account: &Pubkey,
        initial_deposit: u64,
    ) -> Instruction {
        collateral_vault_testing::instruction::InitializeVault { initial_deposit }
            .to_instruction(
                collateral_vault_testing::accounts::InitializeVault {
                    user: *user,
                    vault: *vault_pda,
                    authority: self.authority_pda,
                    vault_token_account: *vault_token_account,
                    user_token_account: *user_token_account,
                    mint: self.usdt_mint,
                    token_program: spl_token_2::id(),
                    system_program: system_program::id(),
                },
            )
            .unwrap()
    }

    pub fn deposit_ix(
        &self,
        user: &Pubkey,
        user_token_account: &Pubkey,
        vault_pda: &Pubkey,
        vault_token_account: &Pubkey,
        amount: u64,
    ) -> Instruction {
        collateral_vault_testing::instruction::Deposit { amount }
            .to_instruction(
                collateral_vault_testing::accounts::Deposit {
                    user: *user,
                    user_token_account: *user_token_account,
                    vault: *vault_pda,
                    vault_token_account: *vault_token_account,
                    token_program: spl_token_2::id(),
                },
            )
            .unwrap()
    }
}

// --- Private Helpers ---

fn create_mint_account(mint_authority: &Pubkey, decimals: u8) -> Account {
    let mut mint_data = vec![0; Mint::LEN];
    let mint = Mint {
        mint_authority: Some(*mint_authority).into(),
        supply: 0,
        decimals,
        is_initialized: true,
        freeze_authority: None.into(),
    };
    mint.pack_into_slice(&mut mint_data);

    Account {
        lamports: 1_000_000_000, // Rent
        data: mint_data,
        owner: spl_token_2::id(),
        executable: false,
        rent_epoch: 0,
    }
}