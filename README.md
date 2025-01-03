# Ethereum ETL in Rust (ethereum-etl.rs)

[![Rust](https://img.shields.io/badge/rust-lang-blue.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

This project was developed with the assistance of [gemini-exp-1206](https://aistudio.google.com/app/prompts/new_chat?model=gemini-exp-1206).

A high-performance Rust implementation of the [Ethereum ETL](https://github.com/blockchain-etl/ethereum-etl) project. This command-line tool allows you to extract and export Ethereum blockchain data, such as blocks and transactions, to CSV files.

## Current Status

This project is currently in its **Minimum Viable Product (MVP)** stage. It supports exporting blocks and transactions for a given block range.

**Supported Functionality:**

*   Export blocks and transactions to CSV files.

**Planned Features:**

*   Export of other data types (receipts, logs, token transfers, contracts, tokens).
*   Support for additional output formats (JSON, databases, cloud storage).
*   Streaming capabilities for real-time data extraction.
*   Improved error handling and robustness.
*   More comprehensive testing.

## Why Rust?

This project is written in Rust to leverage its key advantages:

*   **Performance:** Rust offers performance comparable to C/C++, making it ideal for data-intensive ETL tasks.
*   **Memory Safety:** Rust's strict compiler prevents memory leaks and other common programming errors, leading to more reliable software.
*   **Concurrency:** Rust's built-in concurrency features enable efficient parallel processing, crucial for handling large datasets.

## Installation

1. **Install Rust:**

   If you don't have Rust installed, follow the instructions on the official Rust website: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

2. **Clone the Repository:**

    ```bash
    git clone https://github.com/blockchain-etl/ethereum-etl.rs
    cd ethereum-etl.rs
    ```

3. **Build the Project:**

    ```bash
    cargo build --release
    ```

## Usage

The main command is `export_blocks_and_transactions`.

**Command:**

```bash
RUST_LOG=debug ./target/release/ethereum-etl export_blocks_and_transactions --start-block <start_block> --end-block <end_block> --provider-uri <provider_uri> --blocks-output <blocks_output_file> --transactions-output <transactions_output_file>