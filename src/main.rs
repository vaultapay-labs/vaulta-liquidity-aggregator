use clap::{Parser, Subcommand};
use tracing::{info, error};
use vaulta_liquidity_aggregator::{
    aggregator::LiquidityAggregator,
    config::Config,
    types::*,
};

#[derive(Parser)]
#[command(name = "vaulta-liquidity-aggregator")]
#[command(about = "Cross-protocol liquidity routing aggregator for Solana", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// RPC endpoint URL
    #[arg(long, default_value = "https://api.mainnet-beta.solana.com")]
    rpc_url: String,
    
    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Find optimal swap path
    FindPath {
        /// Input token mint address
        #[arg(short, long)]
        input_mint: String,
        /// Output token mint address
        #[arg(short, long)]
        output_mint: String,
        /// Input amount (in smallest unit)
        #[arg(short, long)]
        amount: u64,
        /// Slippage tolerance (basis points, e.g., 50 = 0.5%)
        #[arg(short, long, default_value = "50")]
        slippage_bps: u16,
    },
    /// Execute a swap
    Swap {
        /// Input token mint address
        #[arg(short, long)]
        input_mint: String,
        /// Output token mint address
        #[arg(short, long)]
        output_mint: String,
        /// Input amount (in smallest unit)
        #[arg(short, long)]
        amount: u64,
        /// Wallet keypair path
        #[arg(short, long)]
        keypair: String,
        /// Slippage tolerance (basis points)
        #[arg(short, long, default_value = "50")]
        slippage_bps: u16,
    },
    /// Get quote for a swap
    Quote {
        /// Input token mint address
        #[arg(short, long)]
        input_mint: String,
        /// Output token mint address
        #[arg(short, long)]
        output_mint: String,
        /// Input amount (in smallest unit)
        #[arg(short, long)]
        amount: u64,
    },
    /// Check DEX status
    Status,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    // Initialize tracing
    let filter = if cli.verbose {
        "vaulta_liquidity_aggregator=debug"
    } else {
        "vaulta_liquidity_aggregator=info"
    };
    
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| filter.into()),
        )
        .init();

    info!("🚀 Vaulta Liquidity Aggregator v{}", env!("CARGO_PKG_VERSION"));
    
    let config = Config::new(&cli.rpc_url)?;
    let aggregator = LiquidityAggregator::new(config).await?;

    match cli.command {
        Commands::FindPath {
            input_mint,
            output_mint,
            amount,
            slippage_bps,
        } => {
            info!("Finding optimal path: {} -> {} (amount: {})", 
                  input_mint, output_mint, amount);
            
            let path = aggregator
                .find_optimal_path(&input_mint, &output_mint, amount, slippage_bps)
                .await?;
            
            println!("Optimal path found:");
            println!("  Route: {:?}", path.route);
            println!("  Expected output: {}", path.expected_output_amount);
            println!("  Price impact: {:.4}%", path.price_impact_pct);
            println!("  Slippage: {:.4}%", path.slippage_pct);
            println!("  MEV protected: {}", path.mev_protected);
        }
        
        Commands::Swap {
            input_mint,
            output_mint,
            amount,
            keypair,
            slippage_bps,
        } => {
            info!("Executing swap: {} -> {} (amount: {})", 
                  input_mint, output_mint, amount);
            
            let result = aggregator
                .execute_swap(
                    &input_mint,
                    &output_mint,
                    amount,
                    &keypair,
                    slippage_bps,
                )
                .await?;
            
            println!("Swap executed successfully!");
            println!("  Transaction signature: {}", result.signature);
            println!("  Output amount: {}", result.output_amount);
            println!("  Price impact: {:.4}%", result.price_impact_pct);
        }
        
        Commands::Quote {
            input_mint,
            output_mint,
            amount,
        } => {
            info!("Getting quote: {} -> {} (amount: {})", 
                  input_mint, output_mint, amount);
            
            let quote = aggregator
                .get_quote(&input_mint, &output_mint, amount)
                .await?;
            
            println!("Quote:");
            println!("  Input: {} {}", amount, input_mint);
            println!("  Output: {} {}", quote.output_amount, output_mint);
            println!("  Price: {}", quote.price);
            println!("  Price impact: {:.4}%", quote.price_impact_pct);
            println!("  Route: {:?}", quote.route);
        }
        
        Commands::Status => {
            let status = aggregator.get_status().await?;
            println!("DEX Status:");
            for (dex, dex_status) in status {
                println!("  {}: {}", dex, if dex_status.available { "✅ Available" } else { "❌ Unavailable" });
                if let Some(latency) = dex_status.latency_ms {
                    println!("    Latency: {}ms", latency);
                }
            }
        }
    }

    Ok(())
}
