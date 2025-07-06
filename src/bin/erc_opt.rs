// src/bin/erc_opt.rs
// =================================
// Equal-Risk-Contribution (ERC) Portfolio Optimizer

use clap::Parser;
use nalgebra::{DMatrix, DVector};
use pyfetcher::read_price_csv;
use std::error::Error;

/// CLI for Equal-Risk-Contribution portfolio optimization
#[derive(Parser)]
#[command(name = "erc-opt", version)]
struct Opt {
    /// Path to CSV file containing price data (first column Date, others tickers)
    #[arg(help = "CSV file with Date + tickers")]
    path: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::parse();

    // Load prices and compute simple returns
    let data = read_price_csv(&opt.path)?;
    let ret = data.simple_returns();

    let m = ret.n_assets();
    let n = ret.n_obs() as f64;

    // Compute covariance matrix
    let cov = {
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

    // Convert to DMatrix
    let mut cov_mat = DMatrix::zeros(m, m);
    for i in 0..m {
        for j in 0..m {
            cov_mat[(i, j)] = cov[i][j];
        }
    }

    // Initialize weights equally
    let mut w = DVector::from_element(m, 1.0 / (m as f64));
    let tol = 1e-8;
    let max_iters = 1000;

    // Iteratively solve ERC via scaling
    for iter in 0..max_iters {
        // Sigma * w
        let sigma_w = &cov_mat * &w;
        // Portfolio variance = w^T * sigma_w
        let port_var = w.dot(&sigma_w);
        let target_rc = port_var / (m as f64);

        // Update weights
        let mut w_new = w.clone();
        for i in 0..m {
            let rc_i = w[i] * sigma_w[i];
            w_new[i] = w[i] * (target_rc / rc_i);
        }
        // Renormalize to sum to 1
        let sum_w_new: f64 = w_new.iter().sum();
        w_new /= sum_w_new;

        // Check convergence
        let err = w_new.iter()
            .zip(w.iter())
            .map(|(a, b)| (a - b).abs())
            .fold(0.0, f64::max);
        w = w_new;
        if err < tol {
            println!("Converged in {} iterations", iter + 1);
            break;
        }
        if iter == max_iters - 1 {
            println!("Warning: ERC solver did not converge (err={:.2e})", err);
        }
    }

    // Print ERC weights
    println!("Equal-Risk-Contribution portfolio weights:");
    for (ticker, &weight) in ret.tickers.iter().zip(w.iter()) {
        println!("  {}: {:.2}%", ticker, weight * 100.0);
    }

    Ok(())
}
