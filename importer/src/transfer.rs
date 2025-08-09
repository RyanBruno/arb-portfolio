use std::cmp::PartialEq;
use crate::{Transfer};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::str::FromStr;
use rust_decimal::Decimal;
use std::ops::Neg;
use serde::Deserialize;

impl PartialEq for Transfer {
  #[allow(clippy::nonminimal_bool)]
  fn eq(&self, other: &Self) -> bool {
    true
    && self.transfer_id == other.transfer_id
    && self.token == other.token
    //&& self.counterparty == other.counterparty
  }
}

pub trait NetTransfers {
  fn net_transfers(_: Vec<Transfer>) -> Vec<Transfer>;
  fn populate_usd(_: &mut Vec<Transfer>) -> &mut Vec<Transfer>;
  /// Applies manual USD overrides from `data/ref/manual_usd.toml`.
  fn apply_manual_usd(_: &mut Vec<Transfer>) -> &mut Vec<Transfer>;
}

#[derive(Debug, Deserialize)]
/// USD value override loaded from configuration.
struct ManualUsd {
  /// Absolute USD value associated with a `transfer_id`.
  usd_value: String,
}

impl NetTransfers for Transfer {
  fn net_transfers(transfers: Vec<Transfer>) -> Vec<Transfer> {
    transfers.into_iter().fold(HashMap::new(), |mut map: HashMap<String, Vec<Transfer>>, transfer| {

      let net_transfers: &mut Vec<Transfer> = map.entry(transfer.transfer_id.clone()).or_default();

      match net_transfers.into_iter().find(|x| **x == transfer) {
        Some(current) => current.apply(&transfer),
        None => net_transfers.push(transfer),
      }

      map
    }).into_values().flatten().collect()

  }
  fn populate_usd(transfers: &mut Vec<Transfer>) -> &mut Vec<Transfer> {
    transfers.iter_mut().fold(HashMap::new(), |mut map: HashMap<String, (Option<Decimal>, Vec<&mut Transfer>)>, transfer| {
      let item = map.entry(transfer.transfer_id.clone()).or_default();

      if let Some(stable_value) = transfer.token.stable_usd_value {
        let usd_value = transfer.value * stable_value;
        transfer.usd_value = Some(usd_value);
        item.0 = Some(usd_value.abs())
      }

      item.1.push(transfer);

      map
    })
    .into_iter().for_each(|(_transfer_id, (usd_value, mut transfers))| {
      if let Some(usd_value) = usd_value {
        if transfers.len() == 2 {
          transfers.iter_mut().for_each(|transfer| {
            if transfer.value.is_sign_positive() {
              transfer.usd_value = Some(usd_value);
            } else {
              transfer.usd_value = Some(usd_value.neg());
            }
          });
        }
      }

    });
    transfers
  }

  fn apply_manual_usd(transfers: &mut Vec<Transfer>) -> &mut Vec<Transfer> {
    // Load manual USD overrides keyed by transfer_id.
    let path = Path::new("data/ref/manual_usd.toml");
    let toml_str = fs::read_to_string(path).unwrap_or_else(|_| String::new());
    let overrides: HashMap<String, ManualUsd> = toml::de::from_str(&toml_str).unwrap_or_default();

    transfers.iter_mut().for_each(|transfer| {
      if let Some(item) = overrides.get(&transfer.transfer_id) {
        let usd = Decimal::from_str(&item.usd_value).unwrap();
        // Preserve sign of original value when applying override.
        transfer.usd_value = Some(if transfer.value.is_sign_positive() { usd } else { usd.neg() });
      }
    });

    transfers
  }

}

impl Transfer {
  pub fn apply(&mut self, other: &Transfer) {
    self.value += other.value;
    self.usd_value = self.usd_value.zip(other.usd_value).map(|(x, y)| x + y);
  }
}
