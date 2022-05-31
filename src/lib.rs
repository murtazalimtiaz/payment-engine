#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate quick_error;

mod balance;
mod client;
mod constants;
mod error;
mod state;
mod transaction;
mod types;
mod utils;

use std::{fs::File, io, path::Path};

use clap::{app_from_crate, arg};
use client::Clients;
use constants::OUTPUT_HEADER;
use csv::{ByteRecord, Writer};

use crate::{
    client::Client,
    constants::{MINIMAL_INPUT_HEADER, STANDARD_INPUT_HEADER},
    state::State,
    transaction::Transaction,
    types::Result,
};

/// Reads arguments, builds and returns app's state
pub fn get_args() -> Result<State> {
    let matches = app_from_crate!()
        .arg(arg!(<file> "An input file representing a series of transactions").allow_invalid_utf8(true))
        .get_matches();

    // SAFETY: `file` is a required argument thus it will always be present
    State::new(matches.value_of_lossy("file").unwrap())
}

/// Opens CSV file and returns reader
fn open<T: AsRef<Path>>(filename: T) -> Result<csv::Reader<File>> {
    Ok(csv::ReaderBuilder::new()
        .flexible(true)
        .trim(csv::Trim::All)
        .from_path(filename)?)
}

/// Runs the app
pub fn process_transactions(mut state: State) -> Result<Clients> {
    let filename = state.filename();
    match open(filename) {
        Err(err) => Err(err),
        Ok(mut reader) => {
            // This ByteRecord allocation will be reused to gain higher throughput
            let mut raw_record = ByteRecord::new();

            while reader.read_byte_record(&mut raw_record)? {
                let tx: Transaction = raw_record.deserialize(match raw_record.len() {
                    4 => Some(&STANDARD_INPUT_HEADER),
                    3 => Some(&MINIMAL_INPUT_HEADER),
                    _ => {
                        return Err(format!(
                            "Error reading data from input file! Supported record length is 3 or 4 fields, found {}.",
                            raw_record.len()
                        )
                        .into())
                    },
                })?;

                state
                    .clients
                    .entry(tx.client_id)
                    .and_modify(|client| {
                        if let Err(err) = client.process_tx(tx.id, tx.get_data()) {
                            eprintln!("✘ Error processing transaction! {tx:?}\n{err}")
                        }
                    })
                    .or_insert_with(|| Client::new(tx.id, tx.get_data()));
            }
            Ok(state.clients)
        },
    }
}

/// Prints clients accounts to stdout as CSV
// TODO: Change to par_iter to speed up output
// NOTE: Use unix::io::AsRawFd for Nix targets
pub fn print_client_account_balances(clients: Clients) -> Result<()> {
    let mut writer = Writer::from_writer(io::stdout());

    writer.write_record(OUTPUT_HEADER)?;

    for (client_id, client) in clients {
        if let Err(err) = writer.write_byte_record(&ByteRecord::from(client.get_record(client_id))) {
            eprintln!("✘ {err}");
        }
    }

    Ok(())
}
