use crate::types::{DexStatus, SwapQuote};
use async_trait::async_trait;
use solana_sdk::pubkey::Pubkey;

/// Trait for DEX client implementations
#[async_trait]
pub trait DexClient: Send + Sync {
    /// Get a quote for a swap
    async fn get_quote(
        &self,
        input_mint: &Pubkey,
        output_mint: &Pubkey,
        amount: u64,
    ) -> anyhow::Result<SwapQuote>;
    
    /// Execute a swap
    async fn execute_swap(
        &self,
        input_mint: &Pubkey,
        output_mint: &Pubkey,
        amount: u64,
        keypair_path: &str,
        slippage_bps: u16,
    ) -> anyhow::Result<String>;
    
    /// Check DEX status
    async fn check_status(&self) -> anyhow::Result<DexStatus>;
}
