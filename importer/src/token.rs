use crate::{TokenMeta};
use std::fs;
use std::path::Path;
use std::collections::HashMap;

type TokenConfig = HashMap<String, TokenMeta>;

impl Default for TokenMeta {
    fn default() -> Self {
      Self { 
        asset: String::from("Unknown"),
        symbol: String::from("Unknown"),
      }
    }
}

impl TryFrom<&String> for TokenMeta {
    type Error = &'static str;
    fn try_from(address: &String) -> Result<Self, Self::Error> {
      // Load categories from the TOML file
      let path = Path::new("data/ref/tokens.toml");
      let toml_str = fs::read_to_string(path).unwrap();
      let config: TokenConfig = toml::de::from_str(&toml_str).unwrap();

      // Iterate over the addresses and check for matches
      match config.get(&address.to_lowercase()) {
        Some(meta) => Ok(meta.clone()),
        None => Err("No Token Found"),
      }
    }
}