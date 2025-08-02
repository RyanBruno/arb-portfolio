use clap::Parser;
use std::error::Error;
use csv::ReaderBuilder;
use arb_portfolio::{Transaction, Token, Transfer, read_csv, write_transfers_to_csv};

/// Command line arguments for the backend tool
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Arbitrum address
    #[arg(long)]
    address: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    // initialize logging from log4rs config file
    log4rs::init_file("log4rs.yml", Default::default()).expect("failed to init logger");
  
    // Reading transaction data from a CSV file
    let transactions: Vec<Transaction> = read_csv("data/ingest/transactions.csv")?;

    // Reading token data from a CSV file
    let tokens: Vec<Token> = read_csv("data/ingest/tokens.csv")?;

    let mut transfers: Vec<Transfer> = transactions.into_iter().map(|x| x.into()).collect();
    transfers.extend(tokens.into_iter().map(|x| x.into()));

    write_transfers_to_csv(&transfers, "transfers.csv")?;

    Ok(())
}
