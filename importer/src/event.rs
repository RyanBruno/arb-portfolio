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
            let category = transfer.iter().map(|x| &x.to).collect::<Vec<&String>>().into();
            Event {
              transfer_id,
              category,
              transfer,
            }
          }
          )
          .collect()
    }
}