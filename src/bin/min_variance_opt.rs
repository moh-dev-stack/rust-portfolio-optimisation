// src/bin/portfolio_opt.rs
// ========================
use clap::Parser;
use nalgebra::{DMatrix, DVector};
use pyfetcher::read_price_csv;
use std::error::Error;

/// CLI for portfolio optimization
#[derive(Parser)]
#[command(name = "portfolio-opt", version)]
struct Opt {
    /// Path to price CSV
    #[arg(help = "CSV file with price data")]
    path: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::parse();
    // Load and compute returns
    let data = read_price_csv(&opt.path)?;
    let ret = data.simple_returns();

    // Compute covariance matrix
    let cov = {
        let n = ret.n_obs() as f64;
        let m = ret.n_assets();
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

    // Min-variance weights
    let m = cov.len();
    let mut cm = DMatrix::zeros(m, m);
    for i in 0..m {
        for j in 0..m {
            cm[(i, j)] = cov[i][j];
        }
    }
    let inv = cm.try_inverse().ok_or("Cov matrix not invertible")?;
    let ones = DVector::from_element(m, 1.0);
    let numer = &inv * ones.clone();
    let denom = ones.transpose() * &numer;
    let w = numer / denom[(0, 0)];

    println!("Min-variance portfolio weights:");
    for (t, wi) in ret.tickers.iter().zip(w.iter()) {
        println!("  {}: {:.2}%", t, wi * 100.0);
    }
    Ok(())
}
