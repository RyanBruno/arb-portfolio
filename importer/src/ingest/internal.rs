//! Functions for ingesting normal transaction CSVs exported from Etherscan.

use serde::Deserialize;
use crate::{read_csv, Token, Transfer, TransferDirection};
use rust_decimal::Decimal;
use std::error::Error;
use std::str::FromStr;

/// Converts a CSV transaction row into a [`Transfer`] capturing its ETH movement.
impl From<(&str, Internal)> for Transfer {
    fn from((address, tx): (&str, Internal)) -> Self {
        let (value, counterparty, direction) = match tx.from.to_lowercase() == address.to_lowercase() {
          true => (
            Decimal::from_str(&tx.value_out_eth).ok(),
            tx.tx_to.clone(),
            TransferDirection::Outgoing
          ),
          false => (
            Decimal::from_str(&tx.value_in_eth).ok(),
            tx.from.clone(),
            TransferDirection::Incoming
          ),
        };
        let usd_value = match (Decimal::from_str(&tx.historical_price_eth), value) {
          (Ok(price), Some(value)) => Some(price * value),
          _ => None
        };

        Transfer {
            transfer_id: tx.transaction_hash,
            datetime: tx.datetime_utc.to_string(),
            token: Token {
              asset: "ETH".to_string(),
              symbol: "ETH".to_string(),
              address: "ETH".to_string(),
              stable_usd_value: None,
              is_usd: false,
              is_debt: false,
            },
            value,
            usd_value,
            direction,
            counterparty: vec![counterparty],
        }
    }
}

/// Reads a transaction CSV and converts each row into a [`Transfer`] for the
/// supplied address.
pub fn read_internals(file_path: &str, address: &'static str) -> Result<Vec<Transfer>, Box<dyn Error>> {
    Ok(read_csv::<Internal>(file_path)?.into_iter().map(|x| (address, x).into()).collect())
}

#[derive(Debug, Deserialize)]
/// Raw representation of an Etherscan transaction export.
pub struct Internal {
    #[serde(rename = "Transaction Hash")]
    pub transaction_hash: String,

    #[serde(rename = "Blockno")]
    pub blockno: u64,

    #[serde(rename = "UnixTimestamp")]
    pub unix_timestamp: u64,

    #[serde(rename = "DateTime (UTC)")]
    pub datetime_utc: String, // Consider chrono's `NaiveDateTime` if you want date parsing

    #[serde(rename = "ParentTxFrom")]
    pub parent_tx_from: String,

    #[serde(rename = "ParentTxTo")]
    pub parent_tx_to: String,

    #[serde(rename = "ParentTxETH_Value")]
    pub parent_tx_eth_value: String,

    #[serde(rename = "From")]
    pub from: String,

    #[serde(rename = "TxTo")]
    pub tx_to: String,

    #[serde(rename = "ContractAddress")]
    pub contract_address: String,

    #[serde(rename = "Value_IN(ETH)")]
    pub value_in_eth: String,

    #[serde(rename = "Value_OUT(ETH)")]
    pub value_out_eth: String,

    #[serde(rename = "CurrentValue @ $4045.59366105672/ETH")]
    pub current_value: String,

    #[serde(rename = "Historical $Price/ETH")]
    pub historical_price_eth: String,

    #[serde(rename = "Status")]
    pub status: String,

    #[serde(rename = "ErrCode")]
    pub err_code: String,

    #[serde(rename = "Type")]
    pub tx_type: String,

    #[serde(rename = "PrivateNote")]
    pub private_note: String,
}
