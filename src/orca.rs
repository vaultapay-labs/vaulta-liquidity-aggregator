use crate::dex::DexClient;
use crate::types::{Dex, DexStatus, RouteHop, SwapQuote, SwapRoute};
use anyhow::Result;
use chrono::Utc;
use rust_decimal::Decimal;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use std::time::Instant;
use tracing::debug;

/// Orca DEX client
pub struct OrcaClient {
    api_url: String,
    client: reqwest::Client,
}

impl OrcaClient {
    pub async fn new(api_url: &str) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;
        
        Ok(Self {
            api_url: api_url.to_string(),
            client,
        })
    }
}

#[async_trait::async_trait]
impl DexClient for OrcaClient {
    async fn get_quote(
        &self,
        input_mint: &Pubkey,
        output_mint: &Pubkey,
        amount: u64,
    ) -> Result<SwapQuote> {
        let start = Instant::now();
        
        // Orca API endpoint for quotes
        let url = format!(
            "{}/quote?inputMint={}&outputMint={}&amount={}",
            self.api_url,
            input_mint,
            output_mint,
            amount
        );
        
        debug!("Orca quote request: {}", url);
        
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Orca API error: {}", response.status()));
        }
        
        let json: serde_json::Value = response.json().await?;
        
        let output_amount = json["estimatedAmountOut"]
            .as_str()
            .and_then(|s| s.parse::<u64>().ok())
            .or_else(|| json["estimatedAmountOut"].as_u64())
            .ok_or_else(|| anyhow::anyhow!("Invalid output amount from Orca"))?;
        
        let price_impact_pct = json["priceImpact"]
            .as_f64()
            .unwrap_or(0.0);
        
        let route = SwapRoute {
            hops: vec![RouteHop {
                dex: Dex::Orca,
                input_mint: *input_mint,
                output_mint: *output_mint,
                input_amount: amount,
                output_amount,
                pool_address: json["poolAddress"]
                    .as_str()
                    .and_then(|s| Pubkey::from_str(s).ok()),
            }],
            total_input: amount,
            total_output: output_amount,
            price_impact_pct,
            estimated_fee: 0,
        };
        
        let latency = start.elapsed().as_millis() as u64;
        debug!("Orca quote latency: {}ms", latency);
        
        Ok(SwapQuote {
            input_mint: *input_mint,
            output_mint: *output_mint,
            input_amount: amount,
            output_amount,
            price: Decimal::from(output_amount) / Decimal::from(amount),
            price_impact_pct,
            route,
            dex: Dex::Orca,
            valid_for_seconds: 30,
        })
    }
    
    async fn execute_swap(
        &self,
        _input_mint: &Pubkey,
        _output_mint: &Pubkey,
        _amount: u64,
        _keypair_path: &str,
        _slippage_bps: u16,
    ) -> Result<String> {
        // Implementation would build and send Orca swap transaction
        Ok("mock_orca_signature_1234567890".to_string())
    }
    
    async fn check_status(&self) -> Result<DexStatus> {
        let start = Instant::now();
        
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
