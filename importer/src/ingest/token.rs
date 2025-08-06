//! Functions for ingesting token transfer CSVs exported from Etherscan.

use serde::Deserialize;
use crate::{read_csv, Token as TokenMeta, Transfer};
use rust_decimal::Decimal;
use std::error::Error;
use std::str::FromStr;

/// Converts a raw CSV token transfer and the account address into a normalized [`Transfer`].
impl From<(&str, Token)> for Transfer {
    fn from((address, event): (&str, Token)) -> Self {
        let value = match Decimal::from_str(&event.token_value.replace(",", "")) {
          Ok(value) if event.from.to_lowercase() != address.to_lowercase() => Some(value),
          Ok(value) if event.from.to_lowercase() == address.to_lowercase() => Some(value * Decimal::from_str("-1").unwrap()),
          Err(_) => None,
          _ => panic!(),
        };

        let token: TokenMeta = (&event.contract_address).into();

        Transfer {
            transfer_id: event.transaction_hash,
            datetime: event.datetime_utc.to_string(),
            token,
            /*asset: asset.asset,
            address: event.contract_address,
            symbol: event.token_symbol,*/
            value,
            from: event.from,
            to: event.to,
        }
    }
}

/// Reads a token transfer CSV and converts each row into a [`Transfer`] for the
/// provided address.
pub fn read_tokens(file_path: &str, address: &'static str) -> Result<Vec<Transfer>, Box<dyn Error>> {
    Ok(read_csv::<Token>(file_path)?.into_iter().map(|x| (address, x).into()).collect())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")] // Automatically handles camel case, e.g., "Transaction Hash" -> "TransactionHash"
/// Raw representation of a token transfer row as exported by Etherscan.
pub struct Token {
    #[serde(rename = "Transaction Hash")]
    pub transaction_hash: String,

    pub blockno: u64,

    #[serde(rename = "UnixTimestamp")]
    pub unix_timestamp: u64,

    #[serde(rename = "DateTime (UTC)")]
    pub datetime_utc: String, // You could use `chrono::NaiveDateTime` here for better handling of time

    pub from: String,

    pub to: String,

    #[serde(rename = "TokenValue")]
    pub token_value: String,

    #[serde(rename = "USDValueDayOfTx")]
    pub usd_value_day_of_tx: String,

    #[serde(rename = "ContractAddress")]
    pub contract_address: String,

    #[serde(rename = "TokenName")]
    pub token_name: String,

    #[serde(rename = "TokenSymbol")]
    pub token_symbol: String,
}