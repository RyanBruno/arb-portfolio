//! Core data structures shared across the importer.

use serde::Serialize;
use rust_decimal::Decimal;

#[derive(Default, Debug, Serialize, PartialEq)]
/// Granular classification for swap transactions.
pub enum SwapSubCategory {
  /// A simple two-leg swap where one side has a stable USD value.
  Simple,
  /// Swaps involving multiple assets without a stable USD leg.
  MultiAsset,
  /// Unable to determine the swap type.
  #[default]
  UnknownSwap,
}

#[derive(Default, Debug, Serialize, PartialEq)]
/// High level category describing the nature of a [`Transaction`].
pub enum TransactionCategory {
  /// Automated market-maker swap (e.g. AAVE).
  Swap(SwapSubCategory),
  /// Perpetual or spot trade (e.g. GMX).
  Trade,
  /// Simple transfer of value between accounts.
  Transfer,
  /// Airdropped tokens received without cost.
  Airdrop,
  /// Activity that should be ignored in reports.
  Ignore,
  /// Unknown or unclassified activity.
  #[default]
  Unknown,
}

#[derive(Debug, Serialize)]
/// Grouped representation of on-chain activity consisting of one or more [`Transfer`]s.
pub struct Transaction {
  /// Identifier shared across the underlying transfers.
  pub transfer_id: String,
  /// ISO8601 timestamp of the transaction.
  pub datetime: String,
  /// Classification of the transaction.
  pub category: TransactionCategory,
  /// Total USD cost basis for the transaction.
  pub cost_basis: Decimal,
  /// Pipe-separated list of unique assets involved.
  pub assets: String,
  /// Net value moved by the transaction.
  pub value: Decimal,
  /// Source transfers that compose this transaction.
  #[serde(skip_serializing)]
  pub transfer: Vec<Transfer>,
  /// Number of individual transfers contained.
  pub n: usize,
}

#[derive(Debug, Serialize, Clone)]
/// Normalized representation of an ERC-20 token.
pub struct Token {
  /// Human readable name of the asset.
  pub asset: String,
  /// Short ticker symbol.
  pub symbol: String,
  /// Optional USD value for one token.
  pub stable_usd_value: Option<Decimal>,
  /// Contract address for the token.
  #[serde(skip_serializing)]
  pub address: String,
}

#[derive(Debug, Serialize)]
/// Individual movement of a token value between two addresses.
pub struct Transfer {
  /// Unique identifier used to group related transfers.
  pub transfer_id: String,
  /// ISO8601 timestamp of the transfer.
  pub datetime: String,
  /// Token being transferred.
  pub token: Token,
  /// Amount of token moved, positive for incoming and negative for outgoing.
  pub value: Option<Decimal>,
  /// Sender address.
  #[serde(skip_serializing)]
  pub from: String,
  /// Recipient address.
  #[serde(skip_serializing)]
  pub to: String,
}

