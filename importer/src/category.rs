use crate::{EventCategory};

impl From<Vec<&String>> for EventCategory {
    fn from(_addrs: Vec<&String>) -> Self {
      Default::default()
    }
}