// src/bin/risk_parity_opt.rs
// =================================
// Risk-Parity (inverse-volatility) portfolio optimizer

use clap::Parser;
use pyfetcher::read_price_csv;
use std::error::Error;

/// CLI for risk-parity portfolio optimization
#[derive(Parser)]
#[command(name = "risk-parity-opt", version)]
struct Opt {
    /// Path to CSV file containing price data (Date + tickers)
    #[arg(help = "CSV file with Date + tickers")]
    path: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::parse();

    // Load price data and compute simple returns
    let data = read_price_csv(&opt.path)?;
    let ret = data.simple_returns();

    let m = ret.n_assets();
    let n = ret.n_obs() as f64;

    // Compute asset volatilities (std dev of returns)
    let mut vols = Vec::with_capacity(m);
    for j in 0..m {
        let returns_j: Vec<f64> = ret.prices.iter().map(|row| row[j]).collect();
        let mean = returns_j.iter().sum::<f64>() / n;
        let variance = returns_j
            .iter()
            .map(|&r| (r - mean).powi(2))
            .sum::<f64>()
            / (n - 1.0);
        vols.push(variance.sqrt());
    }

    // Compute inverse volatilities and normalize to sum=1
    let inv_vols: Vec<f64> = vols.iter().map(|&v| 1.0 / v).collect();
    let sum_inv: f64 = inv_vols.iter().sum();
    let weights: Vec<f64> = inv_vols.iter().map(|&iv| iv / sum_inv).collect();

    // Print weights
    println!("Risk-Parity (Inverse-Vol) portfolio weights:");
    for (ticker, &w) in ret.tickers.iter().zip(&weights) {
        println!("  {}: {:.2}%", ticker, w * 100.0);
    }

    Ok(())
}
