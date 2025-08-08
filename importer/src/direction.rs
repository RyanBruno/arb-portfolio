
use crate::{SwapDirection, TransferDirection};

impl From<TransferDirection> for SwapDirection {
    fn from(direction: TransferDirection) -> Self {
      match direction {
        TransferDirection::Incoming => SwapDirection::Purchase,
        TransferDirection::Outgoing => SwapDirection::Sale,
      }
    }
}