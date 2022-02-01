extern crate csv;

use csv::{ReaderBuilder, Trim};
use std::error::Error;
use std::fmt::Result as FmtResult;
use std::fmt::{Display, Formatter};
use std::io;
use std::sync::mpsc::Sender;

use crate::structs::transaction::Transaction;

// CSV Error definition
#[derive(Debug)]
pub enum CSVReaderError {
    CSVReadingError,
}

impl CSVReaderError {
    // Returns the message from the Error type
    pub fn message(&self) -> &str {
        match self {
            CSVReaderError::CSVReadingError => "error reading from csv",
        }
    }
}

impl Display for CSVReaderError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for CSVReaderError {}

/// Reads a CSV entry from the STDIN and send it to the Sender
///
/// # Arguments
///
/// * `tx_channel` - A Sender channel that the entries will be sent
///
pub fn read(tx_channel: Sender<Transaction>) -> Result<(), CSVReaderError> {
    let mut rdr = ReaderBuilder::new()
        .trim(Trim::All)
        .from_reader(io::stdin());
    for tx in rdr.deserialize() {
        match tx {
            Ok(_) => {
                tx_channel.send(tx.unwrap()).unwrap();
            }
            Err(_) => {
                return Err(CSVReaderError::CSVReadingError);
            }
        }
    }
    Ok(())
}
