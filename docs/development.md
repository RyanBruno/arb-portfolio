# Development Setup

## Prerequisites

- [Rust](https://www.rust-lang.org/): This project uses the Rust toolchain. Install using [`rustup`](https://rustup.rs/).

## Building the Project

Clone the repository and build the workspace:

```bash
git clone <repo-url>
cd arb-portfolio
cargo build
```

## Running Tests

Before submitting changes, make sure all tests pass:

```bash
cargo test
```

## Running the Importer CLI

The repository contains a CLI for processing token and transaction data. To run it:

```bash
cargo run --bin run -- --address <ARBITRUM_ADDRESS>
```

Replace `<ARBITRUM_ADDRESS>` with the address you would like to analyze. Input CSV files are expected under the `data/ingest` directory.
