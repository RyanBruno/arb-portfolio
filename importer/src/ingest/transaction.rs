use serde::{Deserialize};
use crate::{Transfer, Token, read_csv};
use rust_decimal::Decimal;
use std::error::Error;
use std::str::FromStr;

impl From<(&str, Transaction)> for Transfer {
    fn from((_address, tx): (&str, Transaction)) -> Self {
        let value = Decimal::from_str(&tx.value_in_eth).ok();
        Transfer {
            transfer_id: tx.txhash,
            datetime: tx.datetime_utc.to_string(),
            token: Token {
              asset: "ETH".to_string(),
              symbol: "ETH".to_string(),
              address: "ETH".to_string(),
            },
            value,
            from: tx.from,
            to: tx.to,
        }
    }
}

pub fn read_transactions(file_path: &str, address: &'static str) -> Result<Vec<Transfer>, Box<dyn Error>> {
    Ok(read_csv::<Transaction>(file_path)?.into_iter().map(|x| (address, x).into()).collect())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Transaction {
    #[serde(rename = "Txhash")]
    pub txhash: String,

    pub blockno: u64,

    #[serde(rename = "UnixTimestamp")]
    pub unix_timestamp: u64,

    #[serde(rename = "DateTime (UTC)")]
    pub datetime_utc: String,  // You could use chrono for better handling of time

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
