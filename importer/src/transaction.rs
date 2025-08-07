//! Conversion logic for grouping raw [`Transfer`]s into higher level [`Transaction`] records.

use crate::{Transaction, TransactionCategory, Transfer, TransferDirection};
use std::collections::HashMap;
use std::collections::HashSet;
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
            .map(|(transfer_id, transfer)| {
                let datetime = transfer.first().unwrap().datetime.clone();

                // Combine transfers that are equal ignoring value to compute net transfers
                let mut net_transfers: Vec<Transfer> = Vec::new();
                for t in transfer.into_iter() {
                    if let Some(existing) = net_transfers.iter_mut().find(|e| *e == &t) {
                        let existing_usd_signed = existing.signed_usd_value();
                        let sum = existing.signed_value() + t.signed_value();
                        let usd_sum = existing_usd_signed + t.signed_usd_value();

                        existing.direction = if sum >= Decimal::ZERO {
                            TransferDirection::Incoming
                        } else {
                            TransferDirection::Outgoing
                        };
                        existing.value = Some(sum.abs());

                        if existing.usd_value.is_some() || t.usd_value.is_some() {
                            existing.usd_value = Some(usd_sum.abs());
                        }
                    } else {
                        net_transfers.push(t);
                    }
                }
                // Remove any transfers that net to zero before classification
                let transfer: Vec<Transfer> = net_transfers
                    .into_iter()
                    .filter(|x| x.value.unwrap_or_default() != Decimal::from_str("0").unwrap())
                    .collect();

                /*let category = transfer.iter()
                .map(|x| vec![&x.to, &x.from])  // Collect both `to` and `from` as a vector of references
                .flatten()                      // Flatten the nested Vec<Vec<&String>> into a single Vec<&String>
                .chain(std::iter::once(&transfer_id))
                .collect::<Vec<&String>>()      // Collect into a Vec<&String>
                .into();  */
                let category: TransactionCategory = (&transfer).into();

                let mut seen = HashSet::new();
                let assets = transfer
                    .iter()
                    .filter(|x| x.token.stable_usd_value.is_none())
                    .map(|x| x.token.asset.clone())
                    .filter(|x| seen.insert(x.clone()))
                    .collect::<Vec<String>>() // Collect into a Vec<&String>
                    .join("|");

                let value = transfer
                    .iter()
                    .filter(|x| x.token.stable_usd_value.is_none())
                    .map(|x| x.signed_value())
                    .sum();

                /*let cost_basis = transfer.iter()
                .filter_map(|x| x.token.stable_usd_value.map(|y| (x.value.unwrap_or_default(), y)))
                .map(|(x, y)| x * y)
                .sum();*/
                let (cost_basis, c) = transfer
                    .iter()
                    .filter_map(|x| x.usd_value.map(|_| x.signed_usd_value()))
                    .fold((Decimal::ZERO, 0u32), |(s, c), x| (s + x, c + 1));

                let cost_basis = if c > 0 {
                    Some(cost_basis / Decimal::from(c))
                } else {
                    None
                };

                Transaction {
                    transfer_id,
                    datetime,
                    category,
                    cost_basis: cost_basis.unwrap_or_default(),
                    assets,
                    value,
                    n: transfer.len(),
                    transfer,
                }
            })
            .collect()
    }
}
