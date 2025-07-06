# Rust Portfolio Optimisation Toolkit

A modular toolkit for fetching historical market data (via Python & yfinance) and running a variety of portfolio-optimization algorithms in Rust:

- **Data Ingestion**: `fetcher` binary wraps a Python script to download price data via yfinance.  
- **Data Loading**: `load_prices` reads CSVs, computes simple returns.  
- **Optimizers**: long-only min-variance, risk-parity (inverse volatility), equal-risk-contribution (ERC), and long-only maximum-Sharpe (tangency).

---

## Setup

1. **Clone & enter** your project directory:
   ```bash
   git clone git@github.com:moh-dev-stack/rust-portfolio-optimisation.git
   cd rust-portfolio-optimisation
   ```

2. **Python venv & deps** (for the fetcher):
   ```bash
   python3 -m venv venv
   source venv/bin/activate
   pip install -r requirements.txt
   ```

3. **Rust deps** are in `Cargo.toml`. Build all binaries with:
   ```bash
   cargo build --release
   ```

---

## Available Binaries

| Binary                | Purpose                                   |
|-----------------------|-------------------------------------------|
| `fetcher`             | Download price data via yfinance          |
| `load_prices`         | Load CSV & compute simple returns         |
| `min_variance_opt`    | Long-only minimum-variance portfolio      |
| `risk_parity_opt`     | Risk-parity (inverse-volatility)          |
| `erc_opt`             | Equal-Risk-Contribution (ERC)             |
| `sharpe_opt`          | Unconstrained maximum-Sharpe (tangency)   |
| `sharpe_opt_long`     | Long-only maximum-Sharpe (tangency)       |

---

## Helper Ticker Universes & Commands

### 1. Commodities Only
```text
GC=F   # Gold
CL=F   # Crude Oil
NG=F   # Natural Gas
SI=F   # Silver
ZC=F   # Corn
ZW=F   # Wheat
```
```bash
cargo run --bin fetcher -- \
  --tickers GC=F CL=F NG=F SI=F ZC=F ZW=F \
  --start 2015-01-01 --interval 1d \
  --out data/commodities.csv
```

### 2. Global Stock Indices
```text
^GSPC   # S&P 500
^IXIC   # NASDAQ Composite
^DJI    # Dow Jones Industrial Average
^FTSE   # FTSE 100
^GDAXI  # DAX (Germany)
^N225   # Nikkei 225 (Japan)
^HSI    # Hang Seng (Hong Kong)
```
```bash
cargo run --bin fetcher -- \
  --tickers ^GSPC ^IXIC ^DJI ^FTSE ^GDAXI ^N225 ^HSI \
  --start 2015-01-01 --interval 1d \
  --out data/indices.csv
```

### 3. Energy Companies Only
```text
XOM    # Exxon Mobil
CVX    # Chevron
BP     # BP Plc
COP    # ConocoPhillips
SHEL   # Shell Plc
EOG    # EOG Resources
```
```bash
cargo run --bin fetcher -- \
  --tickers XOM CVX BP COP SHEL EOG \
  --start 2015-01-01 --interval 1d \
  --out data/energy.csv
```

### 4. High-Net-Worth “Rich” Portfolio
```text
SPY    # S&P 500 ETF
AAPL   # Apple (mega-cap tech)
MSFT   # Microsoft (mega-cap tech)
EEM    # MSCI Emerging Markets ETF
VNQ    # US Real Estate ETF
TLT    # 20+ Year Treasury ETF
GLD    # Gold ETF
```
```bash
# Fetch the data:
cargo run --bin fetcher -- \
  --tickers SPY AAPL MSFT EEM VNQ TLT GLD \
  --start 2015-01-01 --interval 1d \
  --out data/rich_portfolio.csv

# Load & inspect returns:
cargo run --bin load_prices -- data/rich_portfolio.csv
```

---

## Running Optimizers

After fetching & loading, choose an optimizer:

```bash
# Long-only min-variance
cargo run --bin min_variance_opt -- data/rich_portfolio.csv

# Risk-Parity (Inverse-Vol)
cargo run --bin risk_parity_opt -- data/rich_portfolio.csv

# Equal-Risk-Contribution (ERC)
cargo run --bin erc_opt -- data/rich_portfolio.csv

# Long-only Maximum Sharpe (Tangency)
cargo run --bin sharpe_opt_long -- data/rich_portfolio.csv --risk-free 0.02
```

Each binary prints asset weights (as percentages) to stdout.



