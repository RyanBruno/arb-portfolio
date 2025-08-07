//! Functions for ingesting normal transaction CSVs exported from Etherscan.

use crate::{read_csv, Token, Transfer, TransferDirection};
use rust_decimal::Decimal;
use serde::Deserialize;
use std::error::Error;
use std::str::FromStr;

/// Converts a CSV transaction row into a [`Transfer`] capturing its ETH movement.
impl From<(&str, Transaction)> for Transfer {
    fn from((address, tx): (&str, Transaction)) -> Self {
        let in_eth = Decimal::from_str(&tx.value_in_eth).unwrap_or_default();
        let out_eth = Decimal::from_str(&tx.value_out_eth).unwrap_or_default();
        let (value, direction) = if in_eth > Decimal::ZERO {
            (Some(in_eth), TransferDirection::Incoming)
        } else if out_eth > Decimal::ZERO {
            (Some(out_eth), TransferDirection::Outgoing)
        } else {
            (None, TransferDirection::Incoming)
        };
        let usd_value = match (Decimal::from_str(&tx.historical_price_eth), value) {
            (Ok(price), Some(value)) => Some(price * value),
            _ => None,
        };

        let counterparty = if tx.from.to_lowercase() == address.to_lowercase() {
            tx.to.clone()
        } else {
            tx.from.clone()
        };

        Transfer {
            transfer_id: tx.txhash,
            datetime: tx.datetime_utc.to_string(),
            token: Token {
                asset: "ETH".to_string(),
                symbol: "ETH".to_string(),
                address: "ETH".to_string(),
                stable_usd_value: None,
            },
            direction,
            value,
            usd_value,
            counterparty,
        }
    }
}

/// Reads a transaction CSV and converts each row into a [`Transfer`] for the
/// supplied address.
pub fn read_transactions(
    file_path: &str,
    address: &'static str,
) -> Result<Vec<Transfer>, Box<dyn Error>> {
    Ok(read_csv::<Transaction>(file_path)?
        .into_iter()
        .map(|x| (address, x).into())
        .collect())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
/// Raw representation of an Etherscan transaction export.
pub struct Transaction {
    #[serde(rename = "Txhash")]
    pub txhash: String,

    pub blockno: u64,

    #[serde(rename = "UnixTimestamp")]
    pub unix_timestamp: u64,

    #[serde(rename = "DateTime (UTC)")]
    pub datetime_utc: String, // You could use chrono for better handling of time

    pub from: String,

    pub to: String,

    #[serde(rename = "ContractAddress")]
    pub contract_address: String,

    #[serde(rename = "Value_IN(ETH)")]
    pub value_in_eth: String,

    #[serde(rename = "Value_OUT(ETH)")]
    pub value_out_eth: String,

    #[serde(rename = "CurrentValue @ $3525.11740105424/ETH")]
    pub current_value: String,

    #[serde(rename = "TxnFee(ETH)")]
    pub txn_fee_eth: String,

    #[serde(rename = "TxnFee(USD)")]
    pub txn_fee_usd: String,

    #[serde(rename = "Historical $Price/ETH")]
    pub historical_price_eth: String,

    pub status: String,

    #[serde(rename = "ErrCode")]
    pub err_code: String,

    pub method: String,
}
