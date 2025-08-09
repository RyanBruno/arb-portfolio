//! Conversion logic for grouping raw [`Transfer`]s into higher level [`Transaction`] records.

use crate::{TransferDirection, Transaction, Transfer};
use std::collections::HashMap;
use std::ops::Add;
use rust_decimal::Decimal;

/// Convert intermediate types into a collection of [`Transaction`]s.
pub trait ToTransaction {
    /// Consumes the implementor and returns a set of [`Transaction`] values.
    fn to_transaction(self) -> Vec<Transaction>;
}

impl Add for Transfer {
    type Output = Option<Transfer>;

    fn add(self, other: Transfer) -> Option<Transfer> {
      match (&self.direction, &other.direction) {
        (TransferDirection::Incoming, TransferDirection::Incoming) => Some(Transfer {
          transfer_id: self.transfer_id,
          datetime: self.datetime,
          token: self.token,
          counterparty: self.counterparty.into_iter().chain(other.counterparty).collect(),
          value: self.value + other.value,
          usd_value: self.usd_value.and_then(|x| other.usd_value.map(|y| x+y)),
          direction: TransferDirection::Incoming,
        }),
        (TransferDirection::Outgoing, TransferDirection::Outgoing) => Some(Transfer {
          transfer_id: self.transfer_id,
          datetime: self.datetime,
          token: self.token,
          counterparty: self.counterparty.into_iter().chain(other.counterparty).collect(),
          value: self.value + other.value,
          usd_value: self.usd_value.and_then(|x| other.usd_value.map(|y| x+y)),
          direction: TransferDirection::Outgoing,
        }),
        (TransferDirection::Incoming, TransferDirection::Outgoing) if self.value > other.value => Some(Transfer {
          transfer_id: self.transfer_id,
          datetime: self.datetime,
          token: self.token,
          counterparty: self.counterparty.into_iter().chain(other.counterparty).collect(),
          value: self.value + other.value,
          usd_value: self.usd_value.and_then(|x| other.usd_value.map(|y| x-y)),
          direction: TransferDirection::Incoming,
        }),
        (TransferDirection::Outgoing, TransferDirection::Incoming) if self.value > other.value => Some(Transfer {
          transfer_id: self.transfer_id,
          datetime: self.datetime,
          token: self.token,
          counterparty: self.counterparty.into_iter().chain(other.counterparty).collect(),
          value: self.value + other.value,
          usd_value: self.usd_value.and_then(|x| other.usd_value.map(|y| x-y)),
          direction: TransferDirection::Outgoing,
        }),
        (TransferDirection::Incoming, TransferDirection::Outgoing) if self.value < other.value => Some(Transfer {
          transfer_id: self.transfer_id,
          datetime: self.datetime,
          token: self.token,
          counterparty: self.counterparty.into_iter().chain(other.counterparty).collect(),
          value: self.value + other.value,
          usd_value: self.usd_value.and_then(|x| other.usd_value.map(|y| y-x)),
          direction: TransferDirection::Outgoing,
        }),
        (TransferDirection::Outgoing, TransferDirection::Incoming) if self.value < other.value => Some(Transfer {
          transfer_id: self.transfer_id,
          datetime: self.datetime,
          token: self.token,
          counterparty: self.counterparty.into_iter().chain(other.counterparty).collect(),
          value: self.value + other.value,
          usd_value: self.usd_value.and_then(|x| other.usd_value.map(|y| y-x)),
          direction: TransferDirection::Incoming,
        }),
        _ => None,
      }
    }
}


impl Add<&[Transfer]> for Transfer {
    type Output = Vec<Transfer>;

    fn add(self, net_transfers: &[Transfer]) -> Vec<Transfer> {
      match net_transfers.iter().position(|x| *x == self) {
        Some(idx) => {
          let mut net_transfers: Vec<Transfer> = net_transfers.into();
          let current = net_transfers.remove(idx);

          if let Some(transfer) = self + current {
            net_transfers.push(transfer);
          }
          net_transfers
        },
        None => {
          let mut net_transfers: Vec<Transfer> = net_transfers.into();
          net_transfers.push(self);
          net_transfers
        }
      }
    }
}

impl Add<Transfer> for Transaction {
    type Output = Self;

    fn add(self, transfer: Transfer) -> Self {
      let slice: &[Transfer] = &self.net_transfers;
      Self {
        transfer_id: transfer.transfer_id.clone(),
        datetime: transfer.datetime.clone(),
        category: Default::default(),
        assets: self.assets + "|" + &transfer.token.symbol,
        net_transfers: transfer + slice,
      }
    }
}

/// Groups a list of [`Transfer`]s by their identifier to build [`Transaction`]s.
impl ToTransaction for Vec<Transfer> {
    fn to_transaction(self) -> Vec<Transaction> {
      let mut transaction_map: HashMap<String, Transaction> = HashMap::new();

      for transfer in self {
          let current = transaction_map
            .entry(transfer.transfer_id.clone())
            .or_default();
          *current = current.clone() + transfer;
      }

      transaction_map
          .into_values()
          .map(|mut transaction| {
              transaction.net_transfers.retain(|t| {
                  t.value != Decimal::ZERO
              });
              transaction.assets = transaction.net_transfers.iter().map(|x| x.token.symbol.clone()).collect::<Vec<String>>().join("|");
              transaction.category = (&transaction.net_transfers).into();
              transaction
          })
          .collect()
    }
}
