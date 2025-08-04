use crate::{Transfer, Transaction};
use std::collections::HashMap;
use std::collections::HashSet;

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
            let category = transfer.iter()
              .map(|x| vec![&x.to, &x.from])  // Collect both `to` and `from` as a vector of references
              .flatten()                      // Flatten the nested Vec<Vec<&String>> into a single Vec<&String>
              .chain(std::iter::once(&transfer_id))
              .collect::<Vec<&String>>()      // Collect into a Vec<&String>
              .into();  
            let mut seen = HashSet::new();
            let assets = transfer.iter()
              .filter(|x| (x.token.asset != "USDC" && x.token.asset != "Debt USDC"))
              .map(|x| x.token.asset.clone())
              .filter(|x| seen.insert(x.clone()))
              .collect::<Vec<String>>()      // Collect into a Vec<&String>
              .join("|");
            let cost_basis = transfer.iter()
              .filter(|x| (x.token.asset == "USDC" || x.token.asset == "Debt USDC"))
              .map(|x| x.value.unwrap_or_default())
              .sum();

            Transaction{
              transfer_id,
              category,
              cost_basis,
              assets,
              transfer,
            }
          })
          .collect()
    }
}