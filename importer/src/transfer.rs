use crate::{Transfer, TransferDirection};
use rust_decimal::Decimal;
use std::cmp::PartialEq;

impl Transfer {
    /// Signed token amount applying [`TransferDirection`].
    pub fn signed_value(&self) -> Decimal {
        match self.direction {
            TransferDirection::Incoming => self.value.unwrap_or_default(),
            TransferDirection::Outgoing => -self.value.unwrap_or_default(),
        }
    }

    /// Signed USD value applying [`TransferDirection`].
    pub fn signed_usd_value(&self) -> Decimal {
        match self.direction {
            TransferDirection::Incoming => self.usd_value.unwrap_or_default(),
            TransferDirection::Outgoing => -self.usd_value.unwrap_or_default(),
        }
    }
}

impl PartialEq for Transfer {
    #[allow(clippy::nonminimal_bool)]
    fn eq(&self, other: &Self) -> bool {
        true && self.transfer_id == other.transfer_id && self.token == other.token
        //&& self.counterparty == other.counterparty
    }
}
