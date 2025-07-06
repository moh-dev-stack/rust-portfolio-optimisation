// src/bin/portfolio_opt_long.rs
// =================================
// Long-only minimum-variance portfolio optimizer

use clap::Parser;
use nalgebra::{DMatrix, DVector};
use pyfetcher::read_price_csv;
use std::error::Error;

/// CLI for long-only portfolio optimization
#[derive(Parser)]
#[command(name = "portfolio-opt-long", version)]
struct Opt {
    /// CSV file path containing price data
    #[arg(help = "Path to CSV with Date + tickers")]
    path: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::parse();

    // Load raw price data via shared loader
    let data = read_price_csv(&opt.path)?;
    let ret = data.simple_returns();

    // Build covariance matrix
    let m = ret.n_assets();
    let cov = {
        let n = ret.n_obs() as f64;
        let mut means = vec![0.0; m];
        for row in &ret.prices {
            for (j, &v) in row.iter().enumerate() { means[j] += v; }
        }
        for x in &mut means { *x /= n; }
        let mut cov = vec![vec![0.0; m]; m];
        for row in &ret.prices {
            for i in 0..m {
                for j in 0..m {
                    cov[i][j] += (row[i] - means[i]) * (row[j] - means[j]);
                }
            }
        }
        for i in 0..m {
            for j in 0..m {
                cov[i][j] /= n - 1.0;
            }
        }
        cov
    };

    // Convert covariance to DMatrix and invert
    let mut cm = DMatrix::zeros(m, m);
    for i in 0..m {
        for j in 0..m {
            cm[(i, j)] = cov[i][j];
        }
    }
    let inv = cm.try_inverse().ok_or("Covariance matrix not invertible")?;

    // Compute unconstrained weights: w_un = Σ⁻¹·1
    let ones = DVector::from_element(m, 1.0);
    let numer = &inv * ones.clone();
    let denom = ones.transpose() * &numer;
    let mut w = numer / denom[(0, 0)];

    // Enforce long-only: set negative weights to zero
    for wi in w.iter_mut() {
        if *wi < 0.0 {
            *wi = 0.0;
        }
    }
    // Renormalize to sum to 1
    let sum_pos: f64 = w.iter().sum();
    if sum_pos <= 0.0 {
        return Err("All weights non-positive under long-only constraint".into());
    }
    for wi in w.iter_mut() {
        *wi /= sum_pos;
    }

    // Print long-only min-variance weights
    println!("Long-only min-variance portfolio weights:");
    for (ticker, weight) in ret.tickers.iter().zip(w.iter()) {
        println!("  {}: {:.2}%", ticker, weight * 100.0);
    }

    Ok(())
}
