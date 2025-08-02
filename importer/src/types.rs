use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize)]
pub struct Transfer {
  pub transfer_id: String,
  pub datetime: String,
  pub asset: String,
  pub value: String,
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


#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")] // Automatically handles camel case, e.g., "Transaction Hash" -> "TransactionHash"
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
