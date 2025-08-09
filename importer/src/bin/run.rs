//! Command line interface for converting raw CSV exports into normalized transactions.

use clap::Parser;
use std::error::Error;
use arb_portfolio::{
  read_tokens, write_csv, Transfer,
  //read_transactions,
  read_internals,
  NetTransfers,
};

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
    transfers.extend(read_internals("data/ingest/internal.csv", ADDRESS)?);

    let mut transfers = Transfer::net_transfers(transfers);
    let transfers = Transfer::populate_usd(&mut transfers);
    let transfers = Transfer::apply_manual_usd(transfers);

    write_csv(&transfers, "transfers.csv")?;

    Ok(())
}
