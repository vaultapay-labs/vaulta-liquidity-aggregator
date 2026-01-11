//! # Vaulta Liquidity Aggregator
//!
//! Cross-protocol liquidity routing aggregator for Solana with Jupiter, Raydium, Orca integration
//! and MEV protection.
//!
//! ## Features
//!
//! - **Multi-DEX Aggregation**: Jupiter, Raydium, Orca integration
//! - **Optimal Path Finding**: Best execution path across all DEXs
//! - **MEV Protection**: Front-running and sandwich attack prevention
//! - **Slippage Protection**: Configurable slippage tolerance
//! - **Atomic Execution**: All-or-nothing swap execution
//!
//! ## Example
//!
//! ```rust,no_run
//! use vaulta_liquidity_aggregator::{LiquidityAggregator, Config};
//!
//! # async fn example() -> anyhow::Result<()> {
//! let config = Config::new("https://api.mainnet-beta.solana.com")?;
//! let aggregator = LiquidityAggregator::new(config).await?;
//!
//! // Find optimal path
//! let path = aggregator.find_optimal_path(
//!     "So11111111111111111111111111111111111111112", // SOL
//!     "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v", // USDC
//!     1_000_000_000, // 1 SOL
//!     50, // 0.5% slippage
//! ).await?;
//!
//! println!("Optimal path: {:?}", path.route);
//! # Ok(())
//! # }
//! ```

pub mod aggregator;
pub mod config;
pub mod dex;
pub mod jupiter;
pub mod mev;
pub mod orca;
pub mod pathfinder;
pub mod raydium;
pub mod types;
pub mod utils;

pub use aggregator::LiquidityAggregator;
pub use config::Config;
pub use types::*;
