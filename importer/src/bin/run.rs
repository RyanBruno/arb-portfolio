use clap::Parser;
use std::error::Error;
use arb_portfolio::{Transaction, Token, Transfer, read_csv, write_csv, Event};
use arb_portfolio::event::ToEvents;

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
    let transactions: Vec<Transaction> = read_csv("data/ingest/transactions.csv")?;

    // Reading token data from a CSV file
    let tokens: Vec<Token> = read_csv("data/ingest/tokens.csv")?;

    /*let mut transfers: Vec<Transfer> = transactions.into_iter().map(|x| (ADDRESS, x).into()).collect();
    transfers.extend(tokens.into_iter().map(|x| (ADDRESS, x).into()));*/
    let transfers: Vec<Transfer> = tokens.into_iter().map(|x| (ADDRESS, x).into()).collect();

    //let events: Vec<Event> = transfers.to_events();
    write_csv(&transfers, "transfers.csv")?;

    Ok(())
}
