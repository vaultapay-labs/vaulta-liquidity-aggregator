use crate::types::{PathFindingParams, SwapQuote};
use anyhow::Result;
use tracing::debug;

/// Path finding algorithm for optimal swap routing
pub struct PathFinder {
    params: PathFindingParams,
}

impl PathFinder {
    pub fn new(params: PathFindingParams) -> Self {
        Self { params }
    }
    
    /// Find the best quote from multiple DEX quotes
    pub fn find_best_quote(&self, quotes: &[SwapQuote]) -> Result<&SwapQuote> {
        if quotes.is_empty() {
            return Err(anyhow::anyhow!("No quotes provided"));
        }
        
        // Filter quotes based on constraints
        let valid_quotes: Vec<&SwapQuote> = quotes
            .iter()
            .filter(|q| {
                // Check price impact limit
                if self.params.consider_price_impact {
                    if q.price_impact_pct > self.params.max_price_impact_pct {
                        return false;
                    }
                }
                
                // Check minimum output amount
                if let Some(min_output) = self.params.min_output_amount {
                    if q.output_amount < min_output {
                        return false;
                    }
                }
                
                true
            })
            .collect();
        
        if valid_quotes.is_empty() {
            return Err(anyhow::anyhow!("No valid quotes after filtering"));
        }
        
        // Score quotes based on multiple factors
        let best_quote = valid_quotes
            .iter()
            .max_by(|a, b| {
                self.score_quote(a).partial_cmp(&self.score_quote(b)).unwrap()
            })
            .ok_or_else(|| anyhow::anyhow!("Failed to find best quote"))?;
        
        debug!("Best quote selected: DEX={}, output={}, price_impact={:.4}%",
               best_quote.dex.name(),
               best_quote.output_amount,
               best_quote.price_impact_pct);
        
        Ok(best_quote)
    }
    
    /// Score a quote based on multiple factors
    fn score_quote(&self, quote: &SwapQuote) -> f64 {
        let mut score = quote.output_amount as f64;
        
        // Penalize high price impact
        if self.params.consider_price_impact {
            let impact_penalty = quote.price_impact_pct * 0.01;
            score *= (1.0 - impact_penalty);
        }
        
        // Consider fees if available
        if self.params.consider_fees {
            // Lower fees = higher score
            // This is simplified - in real implementation, we'd have actual fee data
            score *= 0.999; // Assume small fee
        }
        
        score
    }
}
