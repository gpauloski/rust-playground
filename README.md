# Rust Playground

Playground for exploring and experimenting with Rust features and crates.

## Usage

Each top-level directory is a library crate for a different topic (e.g., feature or crate).
To create a new topic crate:
```bash
cargo new <topic> --lib
```
Examples are self contained files within the `examples/` directory of each topic crate.
Library code can be added as necessary.

To run an example, `cd` to the topic crate and execute an example.
```bash
cd <topic>
cargo run --example <name>
```
