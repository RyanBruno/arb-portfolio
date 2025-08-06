use std::cmp::PartialEq;
use crate::{Transfer};

impl PartialEq for Transfer {
  fn eq(&self, other: &Self) -> bool {
    true
    && self.transfer_id == other.transfer_id
    && self.token == other.token
    //&& self.counterparty == other.counterparty
  }
}
