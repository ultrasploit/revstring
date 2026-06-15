# Revstring
###### Created as a [BITS'26](https://bits.acicts.lk) project.
----

## Pre-built binaries
The binary files for both Windows and Linux are located in `dist` folder.

## How to run
- `git clone https://github.com/ultrasploit/revstring.git`
- `cd revstring`
- `cargo run`

## How to compile
- For Windows: `cargo build --release --target x86_64-pc-windows-gnu`
- For Linux:   `cargo build --release --target x86_64-unknown-linux-gnu`

## Cargo-generated crate documentation
Visit [crate documentation](docs/revstring) for more information.

## Used external creates
- [unicode_segmentation](https://docs.rs/unicode-segmentation/latest/unicode_segmentation/) - Since rust doesn't provide Unicode segmentation natively, I had to use this external crate to make unicode segmentation possible. Otherwise making a unicode segmentation lib would take days of work.
