//! Utilities for classifying transfers into [`TransactionCategory`] values.

use crate::{SwapSubCategory, TransactionCategory, Transfer};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// Mapping of identifiers to known transaction categories.
struct CategoryMapping {
    pub category: String,
    //pub description: String,
}

/// Convenience alias for the category configuration file.
type CategoryConfig = HashMap<String, CategoryMapping>;

/// Determines whether a set of transfers represents a simple two-token swap.
fn is_simple_swap(transfers: &[Transfer]) -> bool {
  transfers.len() == 2 && transfers.iter().filter(|x| x.token.stable_usd_value.is_some()).count() == 1
}

/// Derives a [`TransactionCategory`] for a group of transfers by consulting
/// `data/ref/categories.toml` and falling back to heuristics when needed.
impl From<&Vec<Transfer>> for TransactionCategory {
    fn from(transfers: &Vec<Transfer>) -> Self {
      // Load categories from the TOML file
      let path = Path::new("data/ref/categories.toml");
      let toml_str = fs::read_to_string(path).unwrap();
      let config: CategoryConfig = toml::de::from_str(&toml_str).unwrap();

      let category: Option<&CategoryMapping> = transfers
        .iter()
        .flat_map(|x| vec![&x.transfer_id, &x.counterparty])
        .find_map(|key| config.get(key));

      match category.map(|x| x.category.as_str()) {
          Some("Swap") if is_simple_swap(transfers) => TransactionCategory::Swap(SwapSubCategory::Simple),
          Some("Swap") => TransactionCategory::Swap(Default::default()),
          Some("Trade") => TransactionCategory::Trade,
          Some("Transfer") => TransactionCategory::Transfer,
          Some("Airdrop") => TransactionCategory::Airdrop,
          Some("Ignore") => TransactionCategory::Ignore,
          None => TransactionCategory::Unknown,
          _ => panic!(),
      }
    }
}

/*impl From<Vec<&String>> for TransactionCategory {
    fn from(addrs: Vec<&String>) -> Self {
      // Load categories from the TOML file
      let path = Path::new("data/ref/categories.toml");
      let toml_str = fs::read_to_string(path).unwrap();
      let config: CategoryConfig = toml::de::from_str(&toml_str).unwrap();

      // Iterate over the addresses and check for matches
      for addr in addrs.clone() {
          for (address, mapping) in &config {
              if address.to_lowercase() == addr.to_lowercase() {
                  match mapping.category.as_str() {
                      "Swap" => return TransactionCategory::Swap,
                      "Trade" => return TransactionCategory::Trade,
                      "Transfer" => return TransactionCategory::Transfer,
                      "Airdrop" => return TransactionCategory::Airdrop,
                      "Ignore" => return TransactionCategory::Ignore,
                      _ => panic!(),
                  }
              }
          }
      }
      Default::default()
    }
}*/