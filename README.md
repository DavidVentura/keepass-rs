# keepass

[![Crates.io](https://img.shields.io/crates/v/keepass.svg)](https://crates.io/crates/keepass)
[![Documentation](https://docs.rs/keepass/badge.svg)](https://docs.rs/keepass/)
[![Build Status](https://travis-ci.org/sseemayer/keepass-rs.svg?branch=master)](https://travis-ci.org/sseemayer/keepass-rs)
[![codecov](https://codecov.io/gh/sseemayer/keepass-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/sseemayer/keepass-rs)

KeePass .kdbx database file parser for Rust

## Example
```rust
extern crate keepass;

use keepass::{Database, NodeRef, Result, Error};
use std::fs::File;

fn main() -> Result<()> {
    // Open KeePass database
    let path = std::path::Path::new("tests/resources/test_db_with_password.kdbx");
    let db = Database::open(
        &mut File::open(path)?,         // the database
        Some("demopass"),               // password
        None                            // keyfile
    )?;

    // Iterate over all Groups and Nodes
    for node in &db.root {
        match node {
            NodeRef::Group(g) => {
                println!("Saw group '{0}'", g.name);
            },
            NodeRef::Entry(e) => {
                let title = e.get_title().unwrap();
                let user = e.get_username().unwrap();
                let pass = e.get_password().unwrap();
                println!("Entry '{0}': '{1}' : '{2}'", title, user, pass);
            }
        }
    }

    Ok(())
}
```

## Installation
Add the following to the `dependencies` section of your `Cargo.toml`:

```
[dependencies]
keepass = "*"
```

**Performance note:** Please set the `RUSTFLAGS` environment variable when compiling to enable CPU-specific optimizations (this greatly affects the speed of the AES key derivation):

```bash
export RUSTFLAGS='-C target-cpu=native'
```

For best results, also compile in Release mode.

Alternatively, you can add a `.cargo/config.toml` like in this project to ensure that rustflags are always set.

For AArch64 / ARMv8:

The `aes` optimizations are not yet enabled on stable rust. If you want a big performance boost you can build using nightly and enabling the `armv8` feature of the `aes` crate:

```
[dependencies.aes]
# Needs at least 0.7.5 for the feature
version = "0.7.5"
features = ["armv8"]
```

## [Documentation](https://docs.rs/keepass)

## Developer Tools

### `kp-dump-xml`
This library contains an optionally-compiled command line application to dump out the internal XML representation from a KDBX database. This can be useful for implementing additional features for the XML parser.

Since the tool depends on additional crates, it is not compiled until you specify the `utilities` feature, e.g.

```
cargo run --release --features "utilities" --bin kp-dump-xml -- path/to/database.kdbx
```

### `kp-show-otp`

This application can dump the OTP value of a given entry

```
cargo run --release --features "utilities" --bin kp-show-otp -- tests/resources/test_db_kdbx4_with_totp_sha512_entry.kdbx "sha512 totp"
```

## License
MIT
