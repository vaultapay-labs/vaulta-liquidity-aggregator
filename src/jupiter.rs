use crate::dex::DexClient;
use crate::types::{Dex, DexStatus, RouteHop, SwapQuote, SwapRoute};
use anyhow::Result;
use chrono::Utc;
use rust_decimal::Decimal;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use std::time::Instant;
use tracing::{debug, info, warn};

/// Jupiter DEX client
pub struct JupiterClient {
    api_url: String,
    client: reqwest::Client,
}

impl JupiterClient {
    pub async fn new(api_url: &str) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;
        
        Ok(Self {
            api_url: api_url.to_string(),
            client,
        })
    }
    
    fn quote_url(&self) -> String {
        format!("{}/quote", self.api_url)
    }
    
    fn swap_url(&self) -> String {
        format!("{}/swap", self.api_url)
    }
}

#[async_trait::async_trait]
impl DexClient for JupiterClient {
    async fn get_quote(
        &self,
        input_mint: &Pubkey,
        output_mint: &Pubkey,
        amount: u64,
    ) -> Result<SwapQuote> {
        let start = Instant::now();
        
        let url = format!(
            "{}?inputMint={}&outputMint={}&amount={}&slippageBps=50",
            self.quote_url(),
            input_mint,
            output_mint,
            amount
        );
        
        debug!("Jupiter quote request: {}", url);
        
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Jupiter API error: {}", response.status()));
        }
        
        let json: serde_json::Value = response.json().await?;
        
        let output_amount = json["outAmount"]
            .as_str()
            .and_then(|s| s.parse::<u64>().ok())
            .ok_or_else(|| anyhow::anyhow!("Invalid output amount from Jupiter"))?;
        
        let price_impact_pct = json["priceImpactPct"]
            .as_f64()
            .unwrap_or(0.0);
        
        let route = SwapRoute {
            hops: vec![RouteHop {
                dex: Dex::Jupiter,
                input_mint: *input_mint,
                output_mint: *output_mint,
                input_amount: amount,
                output_amount,
                pool_address: None,
            }],
            total_input: amount,
            total_output: output_amount,
            price_impact_pct,
            estimated_fee: 0, // Jupiter fees are included in output
        };
        
        let latency = start.elapsed().as_millis() as u64;
        debug!("Jupiter quote latency: {}ms", latency);
        
        Ok(SwapQuote {
            input_mint: *input_mint,
            output_mint: *output_mint,
            input_amount: amount,
            output_amount,
            price: Decimal::from(output_amount) / Decimal::from(amount),
            price_impact_pct,
            route,
            dex: Dex::Jupiter,
            valid_for_seconds: 30,
        })
    }
    
    async fn execute_swap(
        &self,
        input_mint: &Pubkey,
        output_mint: &Pubkey,
        amount: u64,
        _keypair_path: &str,
        slippage_bps: u16,
    ) -> Result<String> {
        info!("Executing Jupiter swap: {} -> {} (amount: {})", 
              input_mint, output_mint, amount);
        
        // In a real implementation, we'd:
        // 1. Load keypair from file
        // 2. Build swap transaction
        // 3. Sign and send transaction
        // 4. Return transaction signature
        
        // For now, return a mock signature
        Ok("mock_jupiter_signature_1234567890".to_string())
    }
    
    async fn check_status(&self) -> Result<DexStatus> {
        let start = Instant::now();
        
        // Simple health check - try to get a quote for SOL/USDC
        let sol_mint = Pubkey::from_str("So11111111111111111111111111111111111111112")?;
        let usdc_mint = Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v")?;
        
        match self.get_quote(&sol_mint, &usdc_mint, 1_000_000_000).await {
            Ok(_) => {
                let latency = start.elapsed().as_millis() as u64;
                Ok(DexStatus {
                    available: true,
                    latency_ms: Some(latency),
                    last_error: None,
                    last_success: Some(Utc::now()),
                })
            }
            Err(e) => {
                Ok(DexStatus {
                    available: false,
                    latency_ms: None,
                    last_error: Some(e.to_string()),
                    last_success: None,
                })
            }
        }
    }
}
