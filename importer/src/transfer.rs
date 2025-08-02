use crate::{Transaction, Token, Transfer};
use csv::Writer;
use std::error::Error;

// Implement From<Transaction> for Transfer
impl From<Transaction> for Transfer {
    fn from(tx: Transaction) -> Self {
        Transfer {
            transfer_id: tx.txhash,
            datetime: tx.datetime_utc.to_string(),
            asset: "ETH".to_string(),
            value: tx.value_in_eth.to_string(),
        }
    }
}

// Implement From<Token> for Transfer
impl From<Token> for Transfer {
    fn from(token: Token) -> Self {
        Transfer {
            transfer_id: token.transaction_hash,
            datetime: token.datetime_utc.to_string(),
            asset: token.token_symbol, // TODO Normalize and filter bad assets
            value: token.token_value.to_string(),
        }
    }
}

pub fn write_transfers_to_csv(transfers: &[Transfer], file_path: &str) -> Result<(), Box<dyn Error>> {
    // Create a CSV writer
    let mut wtr = Writer::from_path(file_path)?;

    // Write headers (this is optional if you want to include headers in your CSV)
    wtr.write_record(&["transfer_id", "datetime", "asset", "value"])?;

    // Write the data (each Transfer object)
    for transfer in transfers {
        wtr.serialize(transfer)?; // Serialize each Transfer object to CSV row
    }

    // Flush and finalize the CSV writing
    wtr.flush()?;

    Ok(())
}