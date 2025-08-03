use crate::{TransactionCategory};
use std::fs;
use std::path::Path;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct CategoryMapping {
    pub category: String,
    //pub description: String,
}

type CategoryConfig = HashMap<String, CategoryMapping>;

impl From<Vec<&String>> for TransactionCategory {
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
}