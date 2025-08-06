use serde::Serialize;
use rust_decimal::Decimal;

#[derive(Default, Debug, Serialize, PartialEq)]
pub enum SwapSubCategory {
  Simple, // Single Asset, Single USD(C)
  MultiAsset, // Multi Asset, No USD(C)
  #[default]
  UnknownSwap,
}

#[derive(Default, Debug, Serialize, PartialEq)]
pub enum TransactionCategory {
  Swap(SwapSubCategory), // AAVE and Swap
  Trade, // GMX
  Transfer, // Transfers
  Airdrop,
  Ignore,
  #[default]
  Unknown,
}

#[derive(Debug, Serialize)]
pub struct Transaction {
  pub transfer_id: String,
  pub datetime: String,
  pub category: TransactionCategory,
  pub cost_basis: Decimal,
  pub assets: String,
  pub value: Decimal,
  #[serde(skip_serializing)]
  pub transfer: Vec<Transfer>,
  pub n: usize,
}


#[derive(Debug, Serialize, Clone)]
pub struct Token {
  pub asset: String,
  pub symbol: String,
  pub stable_usd_value: Option<Decimal>,
  #[serde(skip_serializing)]
  pub address: String
}

#[derive(Debug, Serialize)]
pub struct Transfer {
  pub transfer_id: String,
  pub datetime: String,
  pub token: Token,
  pub value: Option<Decimal>,
  #[serde(skip_serializing)]
  pub from: String,
  #[serde(skip_serializing)]
  pub to: String,
}
