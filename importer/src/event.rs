use crate::{Transfer, Event};
use std::collections::HashMap;

// Implement a custom trait for conversion
pub trait ToEvents {
    fn to_events(self) -> Vec<Event>;
}

// Implement From<Transaction> for Transfer
impl ToEvents for Vec<Transfer> {
    fn to_events(self) -> Vec<Event> {
      let mut events_map: HashMap<String, Vec<Transfer>> = HashMap::new();

      for transfer in self {
          events_map
              .entry(transfer.transfer_id.clone())
              .or_insert_with(Vec::new)
              .push(transfer);
      }

      // Create a Vec of events from the grouped transfers
      events_map
          .into_iter()
          .map(|(transfer_id, transfer)| {
            let category = transfer.iter()
              .map(|x| vec![&x.to, &x.from])  // Collect both `to` and `from` as a vector of references
              .flatten()                      // Flatten the nested Vec<Vec<&String>> into a single Vec<&String>
              .chain(std::iter::once(&transfer_id))
              .collect::<Vec<&String>>()      // Collect into a Vec<&String>
              .into();  
            /*let tokens = transfer.iter()
              .map(|x| x.asset.clone())
              .collect::<Vec<String>>()      // Collect into a Vec<&String>
              .join("|");*/
            Event {
              transfer_id,
              category,
              //tokens,
              transfer,
            }
          })
          .collect()
    }
}