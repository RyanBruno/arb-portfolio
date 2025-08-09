use std::cmp::PartialEq;
use crate::{Transfer};
use std::collections::HashMap;
use rust_decimal::Decimal;
use std::ops::Neg;

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

}

impl Transfer {
  pub fn apply(&mut self, other: &Transfer) {
    self.value += other.value;
    self.usd_value = self.usd_value.zip(other.usd_value).map(|(x, y)| x + y);
  }
}
