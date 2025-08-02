use csv::ReaderBuilder;
use serde::Deserialize;
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