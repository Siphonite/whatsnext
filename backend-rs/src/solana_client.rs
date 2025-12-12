// backend-rs/src/solana_client.rs
use std::str::FromStr;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use anchor_client::{Client, Cluster, Program};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{read_keypair_file, Keypair, Signer},
    pubkey::Pubkey,
    system_instruction,
    transaction::Transaction,
    instruction::{AccountMeta, Instruction},
};

use crate::config::AppConfig;
use crate::constants::MARKET_ASSET;

pub struct SolanaClient {
    pub program_id: Pubkey,
    pub payer: Arc<Keypair>,
    pub cluster: Cluster,
}

impl SolanaClient {
    pub fn new(cfg: &AppConfig) -> Result<Self> {
        let payer = Arc::new(
            read_keypair_file(&cfg.admin_keypair)
                .map_err(|e| anyhow!("failed to read keypair {}: {}", cfg.admin_keypair, e))?,
        );

        let cluster = Cluster::Custom(cfg.rpc_url.clone(), cfg.rpc_url.clone());

        let program_id = Pubkey::from_str(&cfg.program_id)
            .map_err(|e| anyhow!("PROGRAM_ID in .env is invalid: {}", e))?;

        Ok(Self { program_id, payer, cluster })
    }

    pub fn program(&self) -> Program<Arc<Keypair>> {
        let client = Client::new_with_options(
            self.cluster.clone(),
            self.payer.clone(),
            CommitmentConfig::confirmed(),
        );
        client.program(self.program_id).unwrap()
    }

    fn system_program_id() -> Pubkey {
        Pubkey::from_str("11111111111111111111111111111111").unwrap()
    }

    pub fn derive_market_pda(&self, market_id: u64) -> (Pubkey, u8) {
        let seed = market_id.to_le_bytes();
        Pubkey::find_program_address(&[b"market", &seed], &self.program_id)
    }

    pub fn derive_bet_pda(&self, user: &Pubkey, market: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[b"bet", user.as_ref(), market.as_ref()],
            &self.program_id,
        )
    }

    pub fn derive_treasury_pda(&self) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[b"treasury"], &self.program_id)
    }

    // -----------------------------------------------------------
    // TREASURY INITIALIZATION - Manual instruction building
    // -----------------------------------------------------------
    pub fn initialize_treasury_and_send(&self) -> Result<String> {
        let (treasury_pda, _bump) = self.derive_treasury_pda();

        // Discriminator for initialize_treasury from IDL: [124, 186, 211, 195, 85, 165, 129, 166]
        let discriminator: [u8; 8] = [124, 186, 211, 195, 85, 165, 129, 166];
        
        let accounts = vec![
            AccountMeta::new(treasury_pda, false),
            AccountMeta::new(self.payer.pubkey(), true),
            AccountMeta::new_readonly(Self::system_program_id(), false),
        ];

        let instruction = Instruction {
            program_id: self.program_id,
            accounts,
            data: discriminator.to_vec(),
        };

        let blockhash = self
            .program()
            .rpc()
            .get_latest_blockhash()
            .map_err(|e| anyhow!("Blockhash error: {}", e))?;

        let mut tx = Transaction::new_unsigned(solana_sdk::message::Message::new(
            &[instruction],
            Some(&self.payer.pubkey()),
        ));

        tx.sign(&[&*self.payer], blockhash);

        let sig = self
            .program()
            .rpc()
            .send_and_confirm_transaction(&tx)
            .map_err(|e| anyhow!("Failed to initialize treasury: {}", e))?;

        Ok(sig.to_string())
    }

    // -----------------------------------------------------------
    // FUND TREASURY PDA WITH SOL
    // -----------------------------------------------------------
    pub fn fund_treasury_and_send(&self, lamports: u64) -> Result<String> {
        let (treasury_pda, _) = self.derive_treasury_pda();

        let ix = system_instruction::transfer(
            &self.payer.pubkey(),
            &treasury_pda,
            lamports,
        );

        let blockhash = self
            .program()
            .rpc()
            .get_latest_blockhash()
            .map_err(|e| anyhow!("Blockhash error: {}", e))?;

        let mut tx = Transaction::new_unsigned(solana_sdk::message::Message::new(
            &[ix],
            Some(&self.payer.pubkey()),
        ));

        tx.sign(&[&*self.payer], blockhash);

        let sig = self
            .program()
            .rpc()
            .send_and_confirm_transaction(&tx)
            .map_err(|e| anyhow!("Funding treasury failed: {}", e))?;

        Ok(sig.to_string())
    }

    // -----------------------------------------------------------
    // CREATE MARKET - Manual instruction building
    // -----------------------------------------------------------
    pub fn create_market_and_send(
        &self,
        open_price: u64,
        start_time: i64,
        end_time: i64,
        market_id: u64,
    ) -> Result<String> {
        let (market_pda, _) = self.derive_market_pda(market_id);

        // Discriminator for create_market from IDL: [103, 226, 97, 235, 200, 188, 251, 254]
        let mut data = vec![103, 226, 97, 235, 200, 188, 251, 254];
        
        // Serialize arguments: asset (String), open_price (u64), start_time (i64), end_time (i64), market_id (u64)
        let asset = MARKET_ASSET.to_string();
        let asset_bytes = asset.as_bytes();
        data.extend_from_slice(&(asset_bytes.len() as u32).to_le_bytes());
        data.extend_from_slice(asset_bytes);
        data.extend_from_slice(&open_price.to_le_bytes());
        data.extend_from_slice(&start_time.to_le_bytes());
        data.extend_from_slice(&end_time.to_le_bytes());
        data.extend_from_slice(&market_id.to_le_bytes());

        let accounts = vec![
            AccountMeta::new(market_pda, false),
            AccountMeta::new(self.payer.pubkey(), true),
            AccountMeta::new_readonly(Self::system_program_id(), false),
        ];

        let instruction = Instruction {
            program_id: self.program_id,
            accounts,
            data,
        };

        let blockhash = self
            .program()
            .rpc()
            .get_latest_blockhash()
            .map_err(|e| anyhow!("Blockhash error: {}", e))?;

        let mut tx = Transaction::new_unsigned(solana_sdk::message::Message::new(
            &[instruction],
            Some(&self.payer.pubkey()),
        ));

        tx.sign(&[&*self.payer], blockhash);

        let sig = self
            .program()
            .rpc()
            .send_and_confirm_transaction(&tx)
            .map_err(|e| anyhow!("Failed to send create_market tx: {}", e))?;

        Ok(sig.to_string())
    }

    // -----------------------------------------------------------
    // SETTLE MARKET - Manual instruction building
    // -----------------------------------------------------------
    pub fn settle_market_and_send(
        &self,
        market_id: u64,
        close_price: u64,
    ) -> Result<String> {
        let (market_pda, _) = self.derive_market_pda(market_id);

        // Discriminator for settle_market from IDL: [193, 153, 95, 216, 166, 6, 144, 217]
        let mut data = vec![193, 153, 95, 216, 166, 6, 144, 217];
        data.extend_from_slice(&close_price.to_le_bytes());

        let accounts = vec![
            AccountMeta::new(market_pda, false),
            AccountMeta::new_readonly(self.payer.pubkey(), true),
        ];

        let instruction = Instruction {
            program_id: self.program_id,
            accounts,
            data,
        };

        let blockhash = self
            .program()
            .rpc()
            .get_latest_blockhash()
            .map_err(|e| anyhow!("Blockhash error: {}", e))?;

        let mut tx = Transaction::new_unsigned(solana_sdk::message::Message::new(
            &[instruction],
            Some(&self.payer.pubkey()),
        ));

        tx.sign(&[&*self.payer], blockhash);

        let sig = self
            .program()
            .rpc()
            .send_and_confirm_transaction(&tx)
            .map_err(|e| anyhow!("Failed to send settle_market tx: {}", e))?;

        Ok(sig.to_string())
    }
}