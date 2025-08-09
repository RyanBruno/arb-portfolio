use std::cmp::PartialEq;
use crate::{Transfer};
use std::collections::HashMap;
use std::ops::Add;

impl PartialEq for Transfer {
  #[allow(clippy::nonminimal_bool)]
  fn eq(&self, other: &Self) -> bool {
    true
    && self.transfer_id == other.transfer_id
    && self.token == other.token
    //&& self.counterparty == other.counterparty
  }
}

trait NetTransfers {
  fn net_transfers(_: &mut Vec<Transfer>) -> Vec<&Transfer>;
  fn populate_usd(_: Vec<Transfer>) -> Vec<Transfer>;
}

impl NetTransfers for Transfer {
  fn net_transfers(transfers: &mut Vec<Transfer>) -> Vec<&Transfer> {
    transfers.iter_mut().fold(HashMap::new(), |mut map, transfer| {

      let net_transfers: &mut Vec<&mut Transfer> = map.entry(transfer.transfer_id.clone()).or_default();

      match net_transfers.iter_mut().find(|x| **x == transfer) {
        Some(current) => current.apply(transfer),
        None => net_transfers.push(transfer),
      }

      map
    }).values().flatten().collect()

  }
  fn populate_usd(_: Vec<Transfer>) -> Vec<Transfer> {
    panic!();

  }

}

impl Transfer {
  pub fn apply(&mut self, other: &Transfer) {
    self.value += other.value;
    //self.usd_value += other.usd_value;
  }
}

impl Add<&Transfer> for &Transfer {
    type Output = Transfer;

    fn add(self, other: &Transfer) -> Transfer {
      panic!();
    }
}

