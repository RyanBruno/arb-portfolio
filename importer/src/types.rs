//! Core data structures shared across the importer.

use serde::Serialize;
use rust_decimal::Decimal;

#[derive(Default, Debug, Serialize, PartialEq, Clone)]
pub enum SwapDirection {
  Purchase,
  Sale,
  #[default]
  Unknown,
}

#[derive(Default, Debug, Serialize, PartialEq, Clone)]
pub struct SimpleSwap {
  /// Total USD cost basis for the transaction.
  pub cost_basis: Decimal,
  /// The direction (Purchase or Sale) of the swap
  pub direction: SwapDirection,
  /// Token Swapped
  pub token: Token,
  /// Net value moved by the transaction.
  pub value: Decimal,
}

#[derive(Default, Debug, Serialize, PartialEq, Clone)]
pub struct TwoAssetSwap {
  /// Total USD cost basis for the transaction.
  pub cost_basis: Decimal,
  /// Token purchased
  pub token_purchased: Token,
  /// Token sold
  pub token_sold: Token,
  /// Value of the token purchased
  pub value_purchased: Decimal,
  /// Value of the token sold
  pub value_sold: Decimal,
}

#[derive(Default, Debug, Serialize, PartialEq, Clone)]
/// Granular classification for swap transactions.
pub enum SwapSubCategory {
  /// A simple two-leg swap where one side has a stable USD value.
  Simple(SimpleSwap),
  /// Swaps involving multiple assets without a stable USD leg.
  TwoAsset(TwoAssetSwap),
  /// Unable to determine the swap type.
  #[default]
  UnknownSwap,
}

#[derive(Default, Debug, PartialEq, Clone)]
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

#[derive(Debug, Serialize, Default, Clone)]
/// Grouped representation of on-chain activity consisting of one or more [`Transfer`]s.
pub struct Transaction {
  /// Identifier shared across the underlying transfers.
  pub transfer_id: String,
  /// ISO8601 timestamp of the transaction.
  pub datetime: String,
  /// Classification of the transaction.
  pub category: TransactionCategory,
  /// Source transfers that compose this transaction.
  #[serde(skip_serializing)]
  pub net_transfers: Vec<Transfer>,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
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
  #[serde(skip_serializing)]
  pub is_usd: bool,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
/// Direction of value movement relative to the observed account.
pub enum TransferDirection {
    /// Tokens moving into the account.
    Incoming,
    /// Tokens moving out of the account.
    Outgoing,
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
  /// Amount of token moved.
  pub value: Option<Decimal>,
  /// USD value of the transfer at the time of the transaction.
  pub usd_value: Option<Decimal>,
  /// Direction of the transfer.
  pub direction: TransferDirection,
  /// Address on the other side of the transfer relative to the observed account.
  #[serde(skip_serializing)]
  pub counterparty: Vec<String>,
}

