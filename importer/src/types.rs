//! Core data structures shared across the importer.

use serde::Serialize;
use rust_decimal::Decimal;

#[derive(Default, Debug, Serialize, PartialEq, Clone)]
pub struct PortfolioItem {
  /// Token being transferred.
  pub token: Token,
  pub cost_basis: Decimal,
  pub amount: Decimal,
}

pub struct CostBasisTransfer {
  /// Identifier shared across the underlying transfers.
  pub transfer_id: String,
  /// ISO8601 timestamp of the transfer.
  pub datetime: String,
  /// Token being transferred.
  pub token: Token,
  /// Signed Amount of token moved
  pub value: Decimal,

  pub sale_price: Decimal,
  pub cost_basis: Decimal,
  pub PnL: Decimal,
}

#[derive(Default, Debug, Serialize, PartialEq, Clone)]
pub enum SwapDirection {
  Purchase,
  Sale,
  #[default]
  Unknown,
}

#[derive(Default, Debug, Serialize, PartialEq, Clone)]
pub enum DebtDirection {
  Borrow,
  Repayment,
  #[default]
  Unknown,
}

#[derive(Default, Debug, Serialize, PartialEq, Clone)]
pub struct DebtSwap {
  /// The direction (Borrow or Repayment) of the debt
  pub direction: DebtDirection,
  /// Debt token
  pub debt_token: Token,
  /// Value of the debt
  pub debt_value: Decimal,
  /// The token borrowed or used to repay
  pub token: Token,
  /// Value of the token borrowed or repayment
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
  /// Swaps involving multiple assets without a stable USD leg.
  TwoAsset(TwoAssetSwap),
  /// Swap involving debt repayments or borrowing
  Debt(DebtSwap),
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
  pub assets: String,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash)]
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
  #[serde(skip_serializing)]
  pub is_debt: bool,
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
  pub value: Decimal,
  /// USD value of the transfer at the time of the transaction.
  pub usd_value: Option<Decimal>,
  /// Direction of the transfer.
  pub direction: TransferDirection,
  /// Address on the other side of the transfer relative to the observed account.
  #[serde(skip_serializing)]
  pub counterparty: Vec<String>,
}

