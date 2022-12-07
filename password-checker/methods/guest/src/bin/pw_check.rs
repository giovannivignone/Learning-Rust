#![no_main]

use risc0_zkvm_guest::{env, sha};

risc0_zkvm_guest::entry!(main);

pub fn main() {
    let private_password: String = env::read();

    let mut valid = false;

    for ch in private_password.chars() {
        if ch.is_ascii_punctuation() {
            valid = true;
        }
    }
    if !valid {
        panic!("Password must contain a punctuation character!");
    }

    let digest = sha::digest_u8_slice(private_password.as_bytes());

    env::commit(&digest);
}
