use crate::{Transfer, Transaction};
use std::collections::HashMap;
use std::collections::HashSet;
use rust_decimal::Decimal;
use std::str::FromStr;

// Implement a custom trait for conversion
pub trait ToTransaction {
    fn to_transaction(self) -> Vec<Transaction>;
}

// Implement From<Transaction> for Transfer
impl ToTransaction for Vec<Transfer> {
    fn to_transaction(self) -> Vec<Transaction> {
      let mut transaction_map: HashMap<String, Vec<Transfer>> = HashMap::new();

      for transfer in self {
          transaction_map
              .entry(transfer.transfer_id.clone())
              .or_insert_with(Vec::new)
              .push(transfer);
      }

      // Create a Vec of transaction from the grouped transfers
      transaction_map
          .into_iter()
          .map(|(transfer_id, transfer)| {
            let datetime = transfer.first().unwrap().datetime.clone();
            let transfer: Vec<Transfer> = transfer.into_iter()
              .filter(|x| x.value.is_some() && x.value.unwrap() != Decimal::from_str("0").unwrap()).collect();
            let category = transfer.iter()
              .map(|x| vec![&x.to, &x.from])  // Collect both `to` and `from` as a vector of references
              .flatten()                      // Flatten the nested Vec<Vec<&String>> into a single Vec<&String>
              .chain(std::iter::once(&transfer_id))
              .collect::<Vec<&String>>()      // Collect into a Vec<&String>
              .into();  
            let mut seen = HashSet::new();
            let assets = transfer.iter()
              .filter(|x| x.token.stable_usd_value.is_none())
              .map(|x| x.token.asset.clone())
              .filter(|x| seen.insert(x.clone()))
              .collect::<Vec<String>>()      // Collect into a Vec<&String>
              .join("|");

            let value = transfer.iter()
              .filter(|x| x.token.stable_usd_value.is_none())
              .map(|x| x.value.unwrap_or_default())
              .sum();

            let cost_basis = transfer.iter()
              .filter_map(|x| x.token.stable_usd_value.map(|y| (x.value.unwrap_or_default(), y)))
              .map(|(x, y)| x * y)
              .sum();

            Transaction {
              transfer_id,
              datetime,
              category,
              cost_basis,
              assets,
              value,
              transfer,
            }
          })
          .collect()
    }
}