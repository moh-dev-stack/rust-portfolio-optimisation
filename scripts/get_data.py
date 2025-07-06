#!/usr/bin/env python3
import argparse
from datetime import datetime

import sys
print("▶️ Running under Python:", sys.executable)
# sys.exit(0)
import yfinance as yf

def main():
    print("start")
    parser = argparse.ArgumentParser(
        description="Download historical price data via yfinance"
    )
    parser.add_argument(
        "--tickers", "-t",
        nargs="+", required=True,
        help="List of tickers, e.g. AAPL MSFT GC=F"
    )
    parser.add_argument(
        "--start", "-s",
        required=True,
        help="Start date (YYYY-MM-DD)"
    )
    parser.add_argument(
        "--end", "-e",
        default=datetime.today().strftime("%Y-%m-%d"),
        help="End date (YYYY-MM-DD), default today"
    )
    parser.add_argument(
        "--interval", "-i",
        default="1d",
        choices=["1d","1wk","1mo","1h","5m","15m","30m","60m"],
        help="Data interval"
    )
    parser.add_argument(
        "--out", "-o",
        default="data/raw/prices.csv",
        help="Output CSV path"
    )

    args = parser.parse_args()

    data = yf.download(
        args.tickers,
        start=args.start,
        end=args.end,
        interval=args.interval,
        progress=False
    )
    print('Downloaded the data')
    # pick the adjusted close if available
    if "Adj Close" in data:
        prices = data["Adj Close"]
    elif "Close" in data:
        prices = data["Close"]
    else:
        raise ValueError(f"No price column found! Columns: {data.columns}")

    prices.to_csv(args.out)
    print(f"Wrote {len(prices)} rows × {len(prices.columns)} tickers to {args.out}")

if __name__ == "__main__":
    main()
