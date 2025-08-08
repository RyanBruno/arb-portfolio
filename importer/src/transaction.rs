//! Conversion logic for grouping raw [`Transfer`]s into higher level [`Transaction`] records.

use crate::{Transaction, TransactionCategory, Transfer};
use std::collections::HashMap;
use std::str::FromStr;

use rust_decimal::Decimal;

/// Convert intermediate types into a collection of [`Transaction`]s.
pub trait ToTransaction {
    /// Consumes the implementor and returns a set of [`Transaction`] values.
    fn to_transaction(self) -> Vec<Transaction>;
}

/// Groups a list of [`Transfer`]s by their identifier to build [`Transaction`]s.
impl ToTransaction for Vec<Transfer> {
    fn to_transaction(self) -> Vec<Transaction> {
      let mut transaction_map: HashMap<String, Vec<Transfer>> = HashMap::new();

      for transfer in self {
          transaction_map
              .entry(transfer.transfer_id.clone())
              .or_default()
              .push(transfer);
      }

      // Create a Vec of transaction from the grouped transfers
      transaction_map
          .into_iter()
          .map(|(transfer_id, transfers)| {

            // Combine transfers that are equal ignoring value to compute net transfers
            let mut net_transfers: Vec<Transfer> = Vec::new();
            for t in transfers.into_iter() {
              if let Some(existing) = net_transfers.iter_mut().find(|e| *e == &t) {
                let sum = existing.value.unwrap_or_default() + t.value.unwrap_or_default();
                existing.value = Some(sum);
              } else {
                net_transfers.push(t);
              }
            }
            // Remove any transfers that net to zero before classification
            let net_transfers: Vec<Transfer> = net_transfers
              .into_iter()
              .filter(|x| x.value.unwrap_or_default() != Decimal::from_str("0").unwrap())
              .collect();

            let datetime = net_transfers.first().unwrap().datetime.clone();
            let category: TransactionCategory = (&net_transfers).into();

            /*let mut seen = HashSet::new();
            let assets = net_transfers.iter()
              .filter(|x| x.token.stable_usd_value.is_none())
              .map(|x| x.token.asset.clone())
              .filter(|x| seen.insert(x.clone()))
              .collect::<Vec<String>>()      // Collect into a Vec<&String>
              .join("|");*/

            /*let value = net_transfers.iter()
              .filter(|x| x.token.stable_usd_value.is_none())
              .map(|x| x.value.unwrap_or_default())
              .sum();*/

            /*let (cost_basis, c) = net_transfers.iter()
              .filter_map(|x| x.usd_value)
              .fold((Decimal::ZERO, 0u32), |(s, c), x| (s + x, c + 1));

            let cost_basis = if c > 0 {
              Some(cost_basis / Decimal::from(c))
            } else {
              None
            };*/

            Transaction {
              transfer_id,
              datetime,
              category,
              //cost_basis: cost_basis.unwrap_or_default(),
              //assets,
              //value,
              net_transfers,
            }
          })
          .collect()
    }
}