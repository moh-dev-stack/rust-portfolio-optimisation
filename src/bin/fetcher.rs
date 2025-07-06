use clap::{Arg, Command};
use std::process::Command as SysCommand;

fn main() {
    let matches = Command::new("pyfetch")
        .about("Fetches price data by invoking the Python downloader")
        .arg(
            Arg::new("tickers")
                .short('t')
                .long("tickers")
                .required(true)
                .num_args(1..)   // one or more
                .help("Tickers to download"),
        )
        .arg(
            Arg::new("start")
                .short('s')
                .long("start")
                .required(true)
                .help("Start date YYYY-MM-DD"),
        )
        .arg(
            Arg::new("end")
                .short('e')
                .long("end")
                .required(false)
                .help("End date YYYY-MM-DD"),
        )
        .arg(
            Arg::new("interval")
                .short('i')
                .long("interval")
                .required(false)
                .default_value("1d")
                .help("Interval (1d,1wk,1mo,1h,5m,15m,30m,60m)"),
        )
        .arg(
            Arg::new("out")
                .short('o')
                .long("out")
                .required(false)
                .default_value("data/raw/prices.csv")
                .help("Output CSV path"),
        )
        .get_matches();

    // collect values
    let tickers: Vec<&str> = matches
        .get_many::<String>("tickers")
        .unwrap()
        .map(String::as_str)
        .collect();
    let start = matches.get_one::<String>("start").unwrap();
    let end = matches.get_one::<String>("end");
    let interval = matches.get_one::<String>("interval").unwrap();
    let out = matches.get_one::<String>("out").unwrap();

    // build the python command
    let mut cmd = SysCommand::new("python3");
    cmd.arg("./scripts/get_data.py")
        .arg("--tickers")
        .args(&tickers)
        .arg("--start")
        .arg(start)
        .arg("--interval")
        .arg(interval)
        .arg("--out")
        .arg(out);

    if let Some(e) = end {
        cmd.arg("--end").arg(e);
    }

    // run it
    let status = cmd
        .status()
        .expect("Failed to spawn python process");

    if !status.success() {
        eprintln!("Python script exited with: {}", status);
        std::process::exit(1);
    }
}
