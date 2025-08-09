//! Functions for ingesting token transfer CSVs exported from Etherscan.

use crate::{read_csv, Token as TokenMeta, Transfer, TransferDirection};
use rust_decimal::Decimal;
use serde::Deserialize;
use std::error::Error;
use std::str::FromStr;

/// Converts a raw CSV token transfer and the account address into a normalized [`Transfer`].
impl From<(&str, Token)> for Transfer {
    fn from((address, event): (&str, Token)) -> Self {

        let token: TokenMeta = (&event.contract_address).into();

        let value = Decimal::from_str(&event.token_value.replace(",", "")).unwrap();
        let mut usd_value = Decimal::from_str(&event.usd_value_day_of_tx.replace(",", "").replace("$", "")).ok();

        if let Some(stable) = token.stable_usd_value {
            usd_value = Some(value * stable);
        }

        let direction = match event.from.to_lowercase() == address.to_lowercase() {
          true => TransferDirection::Outgoing,
          false => TransferDirection::Incoming,
        };

        let counterparty = if event.from.to_lowercase() == address.to_lowercase() {
            event.to.clone()
        } else {
            event.from.clone()
        };

        Transfer {
            transfer_id: event.transaction_hash,
            datetime: event.datetime_utc.to_string(),
            token,
            /*asset: asset.asset,
            address: event.contract_address,
            symbol: event.token_symbol,*/
            value,
            usd_value,
            direction,
            counterparty: vec![counterparty],
        }
    }
}

/// Reads a token transfer CSV and converts each row into a [`Transfer`] for the
/// provided address.
pub fn read_tokens(
    file_path: &str,
    address: &'static str,
) -> Result<Vec<Transfer>, Box<dyn Error>> {
    Ok(read_csv::<Token>(file_path)?
        .into_iter()
        .map(|x| (address, x).into())
        .collect())
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
