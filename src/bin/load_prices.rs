use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::error::Error;

/// Holds time-series price data in a matrix form for easy optimization.
#[derive(Debug)]
struct PriceData {
    /// Dates corresponding to each row
    pub dates: Vec<String>,
    /// Asset identifiers corresponding to each column
    pub tickers: Vec<String>,
    /// Matrix of prices: prices[row][col]
    pub prices: Vec<Vec<f64>>,
}

impl PriceData {
    /// Number of assets (columns)
    fn n_assets(&self) -> usize {
        self.tickers.len()
    }

    /// Number of observations (rows)
    fn n_obs(&self) -> usize {
        self.dates.len()
    }

    /// Access price by row and column index
    fn price(&self, date_idx: usize, asset_idx: usize) -> f64 {
        self.prices[date_idx][asset_idx]
    }

    /// Compute simple returns: (p_t / p_{t-1}) - 1, returns a PriceData of returns with one fewer observation
    fn simple_returns(&self) -> PriceData {
        let n_obs = self.n_obs();
        let n_assets = self.n_assets();
        let mut ret_dates = Vec::with_capacity(n_obs - 1);
        let mut ret_prices = Vec::with_capacity(n_obs - 1);
        for i in 1..n_obs {
            ret_dates.push(self.dates[i].clone());
            let mut row = Vec::with_capacity(n_assets);
            for j in 0..n_assets {
                let prev = self.prices[i - 1][j];
                let curr = self.prices[i][j];
                row.push((curr / prev) - 1.0);
            }
            ret_prices.push(row);
        }
        PriceData { dates: ret_dates, tickers: self.tickers.clone(), prices: ret_prices }
    }
}

/// Reads a CSV where the first column is `Date` and remaining are tickers.
/// Returns a PriceData struct containing dates, tickers, and a price matrix.
fn read_price_csv(path: &str) -> Result<PriceData, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    // Parse header
    let header_line = lines.next().ok_or("CSV is empty")??;
    let mut headers = header_line.split(',').map(|s| s.trim().to_string());
    let first = headers.next().ok_or("No header columns found")?;
    if first != "Date" {
        return Err("Expected first column to be 'Date'".into());
    }
    let tickers: Vec<String> = headers.collect();

    // Parse rows
    let mut dates = Vec::new();
    let mut prices: Vec<Vec<f64>> = Vec::new();
    for line in lines {
        let line = line?;
        let fields: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
        if fields.len() != tickers.len() + 1 {
            // skip malformed or blank lines
            continue;
        }
        dates.push(fields[0].to_string());
        let mut row: Vec<f64> = Vec::with_capacity(tickers.len());
        for field in &fields[1..] {
            row.push(field.parse()?);
        }
        prices.push(row);
    }

    Ok(PriceData { dates, tickers, prices })
}

fn main() -> Result<(), Box<dyn Error>> {
    // Expect path to CSV as sole argument
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <CSV_PATH>", args[0]);
        std::process::exit(1);
    }
    let path = &args[1];

    let data = read_price_csv(path)?;
    println!("Loaded {} assets × {} observations",
             data.n_assets(), data.n_obs());

    // Compute returns for optimization routines
    let returns = data.simple_returns();
    println!("Computed simple returns: {} assets × {} observations", returns.n_assets(), returns.n_obs());

    // Sample print for first few dates of returns
    for (i, date) in returns.dates.iter().take(5).enumerate() {
        println!("Returns on {}:", date);
        for (j, ticker) in returns.tickers.iter().enumerate() {
            println!("  {}: {:.6}", ticker, returns.price(i, j));
        }
    }

    Ok(())
}
