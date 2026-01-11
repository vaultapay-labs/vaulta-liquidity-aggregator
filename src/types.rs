use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;

/// DEX identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Dex {
    Jupiter,
    Raydium,
    Orca,
}

impl Dex {
    pub fn all() -> Vec<Dex> {
        vec![Dex::Jupiter, Dex::Raydium, Dex::Orca]
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            Dex::Jupiter => "Jupiter",
            Dex::Raydium => "Raydium",
            Dex::Orca => "Orca",
        }
    }
}

/// Swap route through one or more DEXs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapRoute {
    pub hops: Vec<RouteHop>,
    pub total_input: u64,
    pub total_output: u64,
    pub price_impact_pct: f64,
    pub estimated_fee: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteHop {
    pub dex: Dex,
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
    pub input_amount: u64,
    pub output_amount: u64,
    pub pool_address: Option<Pubkey>,
}

/// Optimal path result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimalPath {
    pub route: SwapRoute,
    pub expected_output_amount: u64,
    pub price_impact_pct: f64,
    pub slippage_pct: f64,
    pub mev_protected: bool,
    pub execution_time_ms: Option<u64>,
}

/// Swap quote from a DEX
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapQuote {
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
    pub input_amount: u64,
    pub output_amount: u64,
    pub price: Decimal,
    pub price_impact_pct: f64,
    pub route: SwapRoute,
    pub dex: Dex,
    pub valid_for_seconds: u64,
}

/// Swap execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapResult {
    pub signature: String,
    pub input_amount: u64,
    pub output_amount: u64,
    pub price_impact_pct: f64,
    pub fee_paid: u64,
    pub route_used: SwapRoute,
}

/// DEX status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DexStatus {
    pub available: bool,
    pub latency_ms: Option<u64>,
    pub last_error: Option<String>,
    pub last_success: Option<chrono::DateTime<chrono::Utc>>,
}

/// MEV protection parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MevProtectionParams {
    pub max_price_impact_pct: f64,
    pub min_liquidity: u64,
    pub max_slippage_bps: u16,
    pub require_private_rpc: bool,
    pub front_run_protection: bool,
    pub sandwich_protection: bool,
}

impl Default for MevProtectionParams {
    fn default() -> Self {
        Self {
            max_price_impact_pct: 1.0,
            min_liquidity: 10_000_000, // $10k minimum liquidity
            max_slippage_bps: 50, // 0.5%
            require_private_rpc: false,
            front_run_protection: true,
            sandwich_protection: true,
        }
    }
}

/// Path finding parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathFindingParams {
    pub max_hops: usize,
    pub consider_price_impact: bool,
    pub consider_fees: bool,
    pub min_output_amount: Option<u64>,
    pub max_price_impact_pct: f64,
}

impl Default for PathFindingParams {
    fn default() -> Self {
        Self {
            max_hops: 3,
            consider_price_impact: true,
            consider_fees: true,
            min_output_amount: None,
            max_price_impact_pct: 5.0,
        }
    }
}

/// Aggregator configuration
#[derive(Debug, Clone)]
pub struct AggregatorConfig {
    pub rpc_url: String,
    pub jupiter_api_url: String,
    pub raydium_api_url: String,
    pub orca_api_url: String,
    pub mev_protection: MevProtectionParams,
    pub pathfinding: PathFindingParams,
    pub timeout_seconds: u64,
    pub max_retries: u32,
}

impl Default for AggregatorConfig {
    fn default() -> Self {
        Self {
            rpc_url: "https://api.mainnet-beta.solana.com".to_string(),
            jupiter_api_url: "https://quote-api.jup.ag/v6".to_string(),
            raydium_api_url: "https://api.raydium.io/v2".to_string(),
            orca_api_url: "https://api.orca.so/v1".to_string(),
            mev_protection: MevProtectionParams::default(),
            pathfinding: PathFindingParams::default(),
            timeout_seconds: 30,
            max_retries: 3,
        }
    }
}
