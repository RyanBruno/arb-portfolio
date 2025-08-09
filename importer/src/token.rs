//! Helpers for enriching raw token addresses with human readable metadata.

use crate::Token;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::str::FromStr;

use rust_decimal::Decimal;
use serde::Deserialize;

/// Mapping of token contract addresses to associated metadata loaded from
/// `data/ref/tokens.toml`.
type TokenConfig = HashMap<String, TokenMeta>;

#[derive(Debug, Deserialize, Clone)]
/// Raw token metadata read from configuration.
pub struct TokenMeta {
  /// Long-form asset name, e.g. `"Ether"`.
  pub asset: String,
  /// Short symbol representation, e.g. `"ETH"`.
  pub symbol: String,
  pub is_debt: bool,
  /// Optional USD value of a single token at import time.
  pub stable_usd_value: Option<String>,
}

impl Default for Token {
    /// Produces a placeholder [`Token`] used when no metadata could be resolved.
    fn default() -> Self {
      Self {
        symbol: String::from("Unknown"),
        address: String::from("Unknown"),
        stable_usd_value: None,
        is_usd: false,
      }
    }
}

impl From<&String> for Token {
    /// Attempts to construct a [`Token`] from a contract address by looking up
    /// metadata in `data/ref/tokens.toml`.
    fn from(address: &String) -> Self {
      // Load token metadata from the TOML file
      let path = Path::new("data/ref/tokens.toml");
      let toml_str = fs::read_to_string(path).unwrap();
      let config: TokenConfig = toml::de::from_str(&toml_str).unwrap();

      // Iterate over the addresses and check for matches
      match config.get(&address.to_lowercase()) {
        Some(meta) => Token {
          //asset: meta.asset.clone(),
          symbol: meta.symbol.clone(),
          address: address.to_string(),
          stable_usd_value: meta
            .stable_usd_value
            .as_ref()
            .map(|x| Decimal::from_str(x).unwrap()),
          is_usd: meta.stable_usd_value.is_some(),
          //is_debt: meta.is_debt,
        },
        None => Token {
          address: address.to_string(),
          ..Default::default()
        },
      }
    }
}