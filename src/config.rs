use crate::types::AggregatorConfig;
use anyhow::Result;

pub struct Config {
    inner: AggregatorConfig,
}

impl Config {
    pub fn new(rpc_url: &str) -> Result<Self> {
        let mut config = AggregatorConfig::default();
        config.rpc_url = rpc_url.to_string();
        
        // Override with environment variables if present
        if let Ok(jupiter_url) = std::env::var("JUPITER_API_URL") {
            config.jupiter_api_url = jupiter_url;
        }
        
        if let Ok(raydium_url) = std::env::var("RAYDIUM_API_URL") {
            config.raydium_api_url = raydium_url;
        }
        
        if let Ok(orca_url) = std::env::var("ORCA_API_URL") {
            config.orca_api_url = orca_url;
        }
        
        Ok(Self { inner: config })
    }
    
    pub fn inner(&self) -> &AggregatorConfig {
        &self.inner
    }
}
