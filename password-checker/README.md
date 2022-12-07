# RISC Zero Password Checker

A zkVM implementation of a password checker written in Rust and using the [RISC Zero zkVM](https://www.github.com/risc0/risc0-zkvm) crate. 

## Quick Start

First, make sure [rustup](https://rustup.rs) is installed. This project uses a [nightly](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html) version of [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html). The [`rust-toolchain`](rust-toolchain) file will be used by `cargo` to automatically install the correct version.

To build all methods and execute the method within the zkVM, run the following command:

```
cargo run
```

You should expect to see "Receipt verified!" in your terminal output. If you modify the password string by removing the ! in the `main.rs` file in host, you should see a verification failure.
