// src/lib.rs
// ============
// Shared loader for price data
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;

/// Holds time-series price data in a matrix form for easy optimization.
#[derive(Debug, Clone)]
pub struct PriceData {
    pub dates: Vec<String>,
    pub tickers: Vec<String>,
    pub prices: Vec<Vec<f64>>,
}

impl PriceData {
    pub fn n_assets(&self) -> usize { self.tickers.len() }
    pub fn n_obs(&self) -> usize { self.dates.len() }
    pub fn price(&self, i: usize, j: usize) -> f64 { self.prices[i][j] }

    /// Compute simple returns: (p_t / p_{t-1}) - 1
    pub fn simple_returns(&self) -> PriceData {
        let n = self.n_obs();
        let m = self.n_assets();
        let mut dates = Vec::with_capacity(n - 1);
        let mut mat = Vec::with_capacity(n - 1);
        for i in 1..n {
            dates.push(self.dates[i].clone());
            let mut row = Vec::with_capacity(m);
            for j in 0..m {
                let prev = self.prices[i - 1][j];
                let curr = self.prices[i][j];
                row.push((curr / prev) - 1.0);
            }
            mat.push(row);
        }
        PriceData { dates, tickers: self.tickers.clone(), prices: mat }
    }
}

/// Reads a CSV where first column is `Date`, rest are tickers.
pub fn read_price_csv(path: &str) -> Result<PriceData, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    // Header
    let hdr = lines.next().ok_or("Empty CSV")??;
    let mut parts = hdr.split(',').map(str::trim);
    let first = parts.next().ok_or("Missing header")?;
    if first != "Date" { return Err("Expected 'Date' first".into()); }
    let tickers: Vec<String> = parts.map(String::from).collect();

    // Rows
    let mut dates = Vec::new();
    let mut prices = Vec::new();
    for line in lines {
        let l = line?;
        let fields: Vec<&str> = l.split(',').map(str::trim).collect();
        if fields.len() != tickers.len() + 1 { continue; }
        dates.push(fields[0].to_string());
        let row = fields[1..]
            .iter()
            .map(|x| x.parse::<f64>())
            .collect::<Result<Vec<_>, _>>()?;
        prices.push(row);
    }
    Ok(PriceData { dates, tickers, prices })
}
