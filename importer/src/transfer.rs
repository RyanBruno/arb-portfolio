use crate::{Transaction, Token, Transfer};
use csv::Writer;
use std::error::Error;
use rust_decimal::Decimal;
use std::str::FromStr;

// Implement From<Transaction> for Transfer
impl From<(&str, Transaction)> for Transfer {
    fn from((_address, tx): (&str, Transaction)) -> Self {
        let value = Decimal::from_str(&tx.value_in_eth).ok();
        Transfer {
            transfer_id: tx.txhash,
            datetime: tx.datetime_utc.to_string(),
            asset: "ETH".to_string(),
            value,
            from: tx.from,
            to: tx.to,
        }
    }
}

// Implement From<Token> for Transfer
impl From<(&str, Token)> for Transfer {
    fn from((address, token): (&str, Token)) -> Self {
        let value = match Decimal::from_str(&token.token_value.replace(",", "")) {
          Ok(value) if token.from.to_lowercase() != address.to_lowercase() => Some(value),
          Ok(value) if token.from.to_lowercase() == address.to_lowercase() => Some(value * Decimal::from_str("-1").unwrap()),
          Err(_) => None,
          _ => panic!(),
        };

        Transfer {
            transfer_id: token.transaction_hash,
            datetime: token.datetime_utc.to_string(),
            //asset: token.token_symbol, // TODO Normalize and filter bad assets
            asset: token.contract_address,
            value,
            from: token.from,
            to: token.to,
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