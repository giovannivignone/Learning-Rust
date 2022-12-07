use methods::{DATA_STORAGE_CHECK_ID, DATA_STORAGE_CHECK_PATH};
use risc0_zkvm::host::Prover;
use risc0_zkvm::serde::{from_slice, to_vec};

fn main() {
    // Make the prover.
    let method_code = std::fs::read(DATA_STORAGE_CHECK_PATH)
        .expect("Method code should be present at the specified path; did you use the correct *_PATH constant?");
    let mut prover = Prover::new(&method_code, DATA_STORAGE_CHECK_ID).expect(
        "Prover should be constructed from valid method source code and corresponding method ID",
    );

    let data = String::from("An image that will live in permanence across distributed nodes.");
    let storage_location = String::from("zkp-data-whisperer/data/secret_data.json");

    // Adding private data into the prover
    prover.add_input(&to_vec(&data).unwrap()).unwrap();

    // Run prover & generate receipt
    let receipt = prover.run()
        .expect("Code should be provable unless it 1) had an error or 2) overflowed the cycle limit. See `embed_methods_with_options` for information on adjusting maximum cycle count.");

    // Optional: Verify receipt to confirm that recipients will also be able to verify your receipt
    receipt.verify(PW_CHECK_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct method ID?",
    );
    println!("Receipt verified!");
    // Implement code for transmitting or serializing the receipt for other parties to verify here
}
