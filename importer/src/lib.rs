//! Library utilities for reading, normalizing and classifying blockchain data.

mod read_csv;
pub use read_csv::{read_csv, write_csv};
pub mod category;
pub mod token;

mod types;
pub use types::{SwapSubCategory, Token, Transaction, TransactionCategory, Transfer};

pub mod ingest;
pub use ingest::token::read_tokens;
pub use ingest::transaction::read_transactions;

pub mod transaction;
pub mod transfer;