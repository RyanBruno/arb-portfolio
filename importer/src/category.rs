//! Utilities for classifying transfers into [`TransactionCategory`] values.

use crate::{TransactionCategory, Transfer, SwapSubCategory};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::{Serialize, Serializer};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// Mapping of identifiers to known transaction categories.
struct CategoryMapping {
    pub category: String,
    //pub description: String,
}

/// Convenience alias for the category configuration file.
type CategoryConfig = HashMap<String, CategoryMapping>;

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
        .flat_map(|x| vec![vec![&x.transfer_id], x.counterparty.iter().collect()].concat())
        .find_map(|key| config.get(key));

      match category.map(|x| x.category.as_str()) {
          //Some("Swap") if is_simple_swap(transfers) => TransactionCategory::Swap(SwapSubCategory::Simple),
          //Some("Swap") => TransactionCategory::Swap(Default::default()),
          Some("Swap") => TransactionCategory::Swap(transfers.into()),
          Some("Trade") => TransactionCategory::Trade,
          Some("Transfer") => TransactionCategory::Transfer,
          Some("Airdrop") => TransactionCategory::Airdrop,
          Some("Ignore") => TransactionCategory::Ignore,
          None => TransactionCategory::Unknown,
          _ => panic!(),
      }
    }
}

impl Serialize for TransactionCategory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let name = match self {
            TransactionCategory::Swap(sub) => match sub {
              SwapSubCategory::Simple(_) => "SwapSimple",
              SwapSubCategory::TwoAsset(_) => "SwapTwoAsset",
              SwapSubCategory::UnknownSwap => "SwapUnknown",
              SwapSubCategory::Debt(_) => "SwapDebt",
            },
            TransactionCategory::Trade => "Trade",
            TransactionCategory::Transfer => "Transfer",
            TransactionCategory::Airdrop => "Airdrop",
            TransactionCategory::Ignore => "Ignore",
            TransactionCategory::Unknown => "Unknown",
        };
        serializer.serialize_str(name)
    }
}