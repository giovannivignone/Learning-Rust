#![no_main]

use risc0_zkvm_guest::{env, sha};
use std::fs::File;
use serde_json::{from_reader, Value};
use whisperer_core::Inputs;

risc0_zkvm_guest::entry!(main);

pub fn main() {
    // Read private data from prover
    let private_inputs: Inputs = env::read();

    // Define private data vars
    let private_data: String = private_inputs.data;
    let private_storage_location: String = private_inputs.storage_location;

    // Get file from storage location
    let stored_file = File::open(private_storage_location).unwrap();

    // Deserialize the JSON data from the file
    let data: Value = from_reader(stored_file).unwrap();

    // Check if data exists in storage location
    let mut stored = false;
    if data["critical_data"] == private_data {
        stored = true;
    }

    // Panic if data does not exist in storage location
    if !stored {
        panic!("Data does not exist in storage location");
    }

    // Commit to the data
    let digest = sha::digest(&stored);
    env::commit(&digest);
}
