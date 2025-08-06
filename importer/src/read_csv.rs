//! Lightweight wrappers around the [`csv`] crate for reading and writing data.

use csv::ReaderBuilder;
use csv::WriterBuilder;
use serde::Deserialize;
use serde::Serialize;
use std::error::Error;

/// Generic function to read CSV into a vector of structs.
pub fn read_csv<T>(file_path: &str) -> Result<Vec<T>, Box<dyn Error>>
where
    T: for<'de> Deserialize<'de>, // T must implement Deserialize
{
    let mut rdr = ReaderBuilder::new()
        .has_headers(true) // Assumes the CSV has headers
        .from_path(file_path)?;

    let mut records: Vec<T> = Vec::new();

    // Iterate through the CSV and deserialize each record into type T
    for result in rdr.deserialize() {
        let record: T = result?;
        records.push(record);
    }

    Ok(records)
}

/// Writes a slice of serializable records to a CSV file without headers.
pub fn write_csv<T>(t: &[T], file_path: &str) -> Result<(), Box<dyn Error>>
where
    T: Serialize, // Ensures that T can be serialized
{
    // Create a CSV writer
    let mut wtr = WriterBuilder::new()
        .has_headers(false)  // Disable headers
        .from_path(file_path)?;

    // Write the data (each element in t)
    for item in t {
        wtr.serialize(item)?; // Serialize each item to a CSV row
    }

    // Flush and finalize the CSV writing
    wtr.flush()?;

    Ok(())
}