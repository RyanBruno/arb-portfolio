//! Core data structures shared across the importer.

use serde::Serialize;
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash)]
/// Normalized representation of an ERC-20 token.
pub struct Token {
  /// Short ticker symbol.
  pub symbol: String,
  /// Optional USD value for one token.
  pub stable_usd_value: Option<Decimal>,
  /// Contract address for the token.
  #[serde(skip_serializing)]
  pub address: String,
  #[serde(skip_serializing)]
  pub is_usd: bool,
}

#[derive(Debug, Serialize, Clone)]
/// Individual movement of a token value between two addresses.
pub struct Transfer {
  /// Unique identifier used to group related transfers.
  pub transfer_id: String,
  /// ISO8601 timestamp of the transfer.
  pub datetime: String,
  /// Token being transferred.
  pub token: Token,
  /// Signed amount of token moved.
  pub value: Decimal,
  /// Signed USD value of the transfer at the time of the transaction.
  pub usd_value: Option<Decimal>,
}

