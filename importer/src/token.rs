use crate::{Token};
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use serde::Deserialize;
use rust_decimal::Decimal;
use std::str::FromStr;

type TokenConfig = HashMap<String, TokenMeta>;

#[derive(Debug, Deserialize, Clone)]
pub struct TokenMeta {
  pub asset: String,
  pub symbol: String,
  pub stable_usd_value: Option<String>,
}

impl Default for Token {
    fn default() -> Self {
      Self { 
        asset: String::from("Unknown"),
        symbol: String::from("Unknown"),
        address: String::from("Unknown"),
        stable_usd_value: None,
      }
    }
}

impl From<&String> for Token {
    fn from(address: &String) -> Self {
      // Load categories from the TOML file
      let path = Path::new("data/ref/tokens.toml");
      let toml_str = fs::read_to_string(path).unwrap();
      let config: TokenConfig = toml::de::from_str(&toml_str).unwrap();

      // Iterate over the addresses and check for matches
      match config.get(&address.to_lowercase()) {
        Some(meta) => Token {
          asset: meta.asset.clone(),
          symbol: meta.symbol.clone(),
          address: address.to_string(),
          stable_usd_value: meta.stable_usd_value.as_ref().map(|x| Decimal::from_str(x).unwrap()),
        },
        None => Token {
          address: address.to_string(),
          ..Default::default()
        },
      }
    }
}