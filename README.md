# canihasip-rs

[![Build Status](https://travis-ci.com/hugglesfox/canihasip-rs.svg?branch=master)](https://travis-ci.com/hugglesfox/canihasip-rs) [![Crates.io](https://img.shields.io/crates/v/canihazip)](https://crates.io/crates/canihazip) [![docs.rs](https://docs.rs/canihazip/badge.svg)](https://docs.rs/canihazip)

A [unoffical] rust crate for [icanhazip](https://icanhazip.com)

## Usage

```rust
extern crate canihazip;

let ip = canihazip::plz_ip()?;

assert_eq!(ip.is_loopback(), false);
```

## Contributing

Please feel free to open a PR if there is something that can be improved!

## License

canihazip-rs is licenced under the MIT licence.
