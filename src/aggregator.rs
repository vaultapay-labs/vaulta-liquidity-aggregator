use crate::config::Config;
use crate::dex::DexClient;
use crate::jupiter::JupiterClient;
use crate::mev::MevProtector;
use crate::orca::OrcaClient;
use crate::pathfinder::PathFinder;
use crate::raydium::RaydiumClient;
use crate::types::*;
use anyhow::Result;
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;
use std::str::FromStr;
use tracing::{info, warn};

/// Main liquidity aggregator
pub struct LiquidityAggregator {
    config: AggregatorConfig,
    jupiter: JupiterClient,
    raydium: RaydiumClient,
    orca: OrcaClient,
    pathfinder: PathFinder,
    mev_protector: MevProtector,
}

impl LiquidityAggregator {
    /// Create a new liquidity aggregator
    pub async fn new(config: Config) -> Result<Self> {
        let config_inner = config.inner().clone();
        
        info!("Initializing liquidity aggregator...");
        info!("RPC URL: {}", config_inner.rpc_url);
        
        let jupiter = JupiterClient::new(&config_inner.jupiter_api_url).await?;
        let raydium = RaydiumClient::new(&config_inner.raydium_api_url).await?;
        let orca = OrcaClient::new(&config_inner.orca_api_url).await?;
        let pathfinder = PathFinder::new(config_inner.pathfinding.clone());
        let mev_protector = MevProtector::new(config_inner.mev_protection.clone());
        
        Ok(Self {
            config: config_inner,
            jupiter,
            raydium,
            orca,
            pathfinder,
            mev_protector,
        })
    }
    
    /// Find optimal swap path across all DEXs
    pub async fn find_optimal_path(
        &self,
        input_mint: &str,
        output_mint: &str,
        amount: u64,
        slippage_bps: u16,
    ) -> Result<OptimalPath> {
        let input_pubkey = Pubkey::from_str(input_mint)?;
        let output_pubkey = Pubkey::from_str(output_mint)?;
        
        info!("Finding optimal path: {} -> {} (amount: {})", 
              input_mint, output_mint, amount);
        
        // Get quotes from all DEXs
        let mut quotes = Vec::new();
        
        // Jupiter quote
        if let Ok(quote) = self.jupiter.get_quote(&input_pubkey, &output_pubkey, amount).await {
            quotes.push(quote);
        }
        
        // Raydium quote
        if let Ok(quote) = self.raydium.get_quote(&input_pubkey, &output_pubkey, amount).await {
            quotes.push(quote);
        }
        
        // Orca quote
        if let Ok(quote) = self.orca.get_quote(&input_pubkey, &output_pubkey, amount).await {
            quotes.push(quote);
        }
        
        if quotes.is_empty() {
            return Err(anyhow::anyhow!("No quotes available from any DEX"));
        }
        
        // Find optimal path using pathfinder
        let optimal_quote = self.pathfinder.find_best_quote(&quotes)?;
        
        // Apply MEV protection
        let mev_protected = self.mev_protector.check_protection(
            &optimal_quote.route,
            optimal_quote.price_impact_pct,
        )?;
        
        let optimal_path = OptimalPath {
            route: optimal_quote.route.clone(),
            expected_output_amount: optimal_quote.output_amount,
            price_impact_pct: optimal_quote.price_impact_pct,
            slippage_pct: (slippage_bps as f64) / 10000.0,
            mev_protected,
            execution_time_ms: None,
        };
        
        info!("Optimal path found: output={}, price_impact={:.4}%, mev_protected={}",
              optimal_path.expected_output_amount,
              optimal_path.price_impact_pct,
              optimal_path.mev_protected);
        
        Ok(optimal_path)
    }
    
    /// Get quote for a swap
    pub async fn get_quote(
        &self,
        input_mint: &str,
        output_mint: &str,
        amount: u64,
    ) -> Result<SwapQuote> {
        let input_pubkey = Pubkey::from_str(input_mint)?;
        let output_pubkey = Pubkey::from_str(output_mint)?;
        
        // Get quotes from all DEXs and return the best one
        let path = self.find_optimal_path(input_mint, output_mint, amount, 50).await?;
        
        // Convert to quote
        Ok(SwapQuote {
            input_mint: input_pubkey,
            output_mint: output_pubkey,
            input_amount: amount,
            output_amount: path.expected_output_amount,
            price: rust_decimal::Decimal::from(path.expected_output_amount) 
                / rust_decimal::Decimal::from(amount),
            price_impact_pct: path.price_impact_pct,
            route: path.route,
            dex: Dex::Jupiter, // Best quote's DEX
            valid_for_seconds: 30,
        })
    }
    
    /// Execute a swap
    pub async fn execute_swap(
        &self,
        input_mint: &str,
        output_mint: &str,
        amount: u64,
        keypair_path: &str,
        slippage_bps: u16,
    ) -> Result<SwapResult> {
        info!("Executing swap: {} -> {} (amount: {})", 
              input_mint, output_mint, amount);
        
        // Find optimal path
        let path = self.find_optimal_path(input_mint, output_mint, amount, slippage_bps).await?;
        
        // Verify MEV protection
        if !path.mev_protected {
            warn!("MEV protection check failed, but proceeding with swap");
        }
        
        // Execute swap on the best DEX
        // In a real implementation, we'd use the appropriate DEX client
        let signature = self.jupiter
            .execute_swap(
                &Pubkey::from_str(input_mint)?,
                &Pubkey::from_str(output_mint)?,
                amount,
                keypair_path,
                slippage_bps,
            )
            .await?;
        
        Ok(SwapResult {
            signature,
            input_amount: amount,
            output_amount: path.expected_output_amount,
            price_impact_pct: path.price_impact_pct,
            fee_paid: 0, // Would be calculated from actual transaction
            route_used: path.route,
        })
    }
    
    /// Get status of all DEXs
    pub async fn get_status(&self) -> Result<HashMap<String, DexStatus>> {
        let mut status = HashMap::new();
        
        // Check Jupiter
        let jupiter_status = self.jupiter.check_status().await.unwrap_or(DexStatus {
            available: false,
            latency_ms: None,
            last_error: Some("Status check failed".to_string()),
            last_success: None,
        });
        status.insert("Jupiter".to_string(), jupiter_status);
        
        // Check Raydium
        let raydium_status = self.raydium.check_status().await.unwrap_or(DexStatus {
            available: false,
            latency_ms: None,
            last_error: Some("Status check failed".to_string()),
            last_success: None,
        });
        status.insert("Raydium".to_string(), raydium_status);
        
        // Check Orca
        let orca_status = self.orca.check_status().await.unwrap_or(DexStatus {
            available: false,
            latency_ms: None,
            last_error: Some("Status check failed".to_string()),
            last_success: None,
        });
        status.insert("Orca".to_string(), orca_status);
        
        Ok(status)
    }
}
