//! Command line interface for converting raw CSV exports into normalized transactions.

use clap::Parser;
use std::error::Error;
use arb_portfolio::{
  read_tokens, write_csv, Transaction, Transfer,
  //read_transactions,
  read_internals,
};
use arb_portfolio::transaction::ToTransaction;

/// Command line arguments for the backend tool
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Arbitrum address
    #[arg(long)]
    address: String,
}

const ADDRESS: &str = "0x0A8Dd68E974C371A6a6Efe95cfA22a200eb7AfCc";

/// Runs the importer CLI.
fn main() -> Result<(), Box<dyn Error>> {
    // initialize logging from log4rs config file
    log4rs::init_file("log4rs.yml", Default::default()).expect("failed to init logger");
  
    let mut transfers: Vec<Transfer> = read_tokens("data/ingest/tokens.csv", ADDRESS)?;
    //transfers.extend(read_transactions("data/ingest/transactions.csv", ADDRESS)?);
    transfers.extend(read_internals("data/ingest/internal.csv", ADDRESS)?);

   let transactions: Vec<Transaction> = transfers.clone().to_transaction();

   let net_transfers: Vec<Transfer> = transactions.iter().flat_map(|x| x.net_transfers.clone()).collect();

    write_csv(&transactions, "transactions.csv")?;
    write_csv(&net_transfers, "transfers.csv")?;

    Ok(())
}
