use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

/// Utility functions for the aggregator

/// Parse a pubkey from string, handling common formats
pub fn parse_pubkey(s: &str) -> anyhow::Result<Pubkey> {
    Pubkey::from_str(s)
        .map_err(|e| anyhow::anyhow!("Invalid pubkey '{}': {}", s, e))
}

/// Format a pubkey for display
pub fn format_pubkey(pubkey: &Pubkey) -> String {
    pubkey.to_string()
}

/// Calculate slippage in basis points
pub fn calculate_slippage_bps(expected: u64, actual: u64) -> u16 {
    if expected == 0 {
        return 0;
    }
    
    let slippage = ((expected as f64 - actual as f64) / expected as f64 * 10000.0).abs();
    slippage as u16
}

/// Format amount with decimals
pub fn format_amount(amount: u64, decimals: u8) -> String {
    let divisor = 10_u64.pow(decimals as u32);
    let whole = amount / divisor;
    let fractional = amount % divisor;
    
    if fractional == 0 {
        format!("{}", whole)
    } else {
        format!("{}.{:0width$}", whole, fractional, width = decimals as usize)
    }
}
