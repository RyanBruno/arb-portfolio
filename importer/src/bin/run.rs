use clap::Parser;
use std::error::Error;
use arb_portfolio::{
  Transfer, read_tokens, read_transactions, write_csv, Transaction,
  TransactionCategory,
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

fn main() -> Result<(), Box<dyn Error>> {
    // initialize logging from log4rs config file
    log4rs::init_file("log4rs.yml", Default::default()).expect("failed to init logger");
  
    // Reading transaction data from a CSV file
    //let mut transfers: Vec<Transfer> = read_transactions("data/ingest/transactions.csv", ADDRESS)?;
    let mut transfers: Vec<Transfer> = read_tokens("data/ingest/tokens.csv", ADDRESS)?;

    // Reading token data from a CSV file
    //transfers.extend(

    let transactions: Vec<Transaction> = transfers.to_transaction();

    /*let swaps: Vec<Transfer> = transactions.into_iter().filter(|x| x.category == TransactionCategory::Swap)
      .map(|x| x.transfer)
      .flatten()
      .collect();*/

    write_csv(&transactions, "transactions.csv")?;

    Ok(())
}
