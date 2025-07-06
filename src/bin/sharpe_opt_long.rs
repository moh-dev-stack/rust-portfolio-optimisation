// src/bin/sharpe_opt_long.rs
// =================================
// Long-Only Maximum Sharpe (Tangency) Portfolio Optimizer

use clap::Parser;
use nalgebra::{DMatrix, DVector};
use pyfetcher::read_price_csv;
use std::error::Error;

/// CLI for Long-Only Maximum Sharpe (Tangency) portfolio optimization
#[derive(Parser)]
#[command(name = "sharpe-opt-long", version)]
struct Opt {
    /// Path to CSV file containing price data (first column Date, others tickers)
    #[arg(help = "CSV file with Date + tickers")]
    path: String,

    /// Annualized risk-free rate (e.g., 0.02 for 2%)
    #[arg(short, long, default_value_t = 0.0)]
    risk_free: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::parse();

    // Load prices and compute simple returns
    let data = read_price_csv(&opt.path)?;
    let ret = data.simple_returns();

    let m = ret.n_assets();
    let n = ret.n_obs() as f64;

    // Compute mean returns for each asset
    let mut means = vec![0.0; m];
    for row in &ret.prices {
        for (j, &r) in row.iter().enumerate() {
            means[j] += r;
        }
    }
    for x in &mut means {
        *x /= n;
    }

    // Excess returns vector μ - rf (converted to per-period)
    let daily_rf = opt.risk_free / 252.0;
    let excess_data: Vec<f64> = means.iter().map(|&mu| mu - daily_rf).collect();
    let excess: DVector<f64> = DVector::from_vec(excess_data);

    // Build covariance matrix
    let mut cov_mat = DMatrix::zeros(m, m);
    for i in 0..m {
        for j in 0..m {
            let mut cov = 0.0;
            for row in &ret.prices {
                cov += (row[i] - means[i]) * (row[j] - means[j]);
            }
            cov_mat[(i, j)] = cov / (n - 1.0);
        }
    }

    // Invert covariance
    let inv = cov_mat.try_inverse().ok_or("Covariance matrix not invertible")?;

    // Tangency weights: w_un ∝ Σ⁻¹ * excess
    let unscaled = &inv * excess;
    let mut w = unscaled.map(|wi| wi.max(0.0));  // enforce non-negative

    // Renormalize to sum to 1
    let sum_w: f64 = w.iter().sum();
    if sum_w <= 0.0 {
        return Err("All weights non-positive after enforcing long-only".into());
    }
    w /= sum_w;

    // Print long-only maximum Sharpe weights
    println!("Long-Only Maximum Sharpe (Tangency) Portfolio Weights:");
    for (ticker, &weight) in ret.tickers.iter().zip(w.iter()) {
        println!("  {}: {:.2}%", ticker, weight * 100.0);
    }

    Ok(())
}
