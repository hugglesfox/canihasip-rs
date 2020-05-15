# canihasip-rs

[![Build Status](https://travis-ci.com/hugglesfox/canihasip-rs.svg?branch=master)](https://travis-ci.com/hugglesfox/canihasip-rs)

A [unoffical] rust crate for https://icanhazip.com/

## Usage

```rust
extern crate canihazip;

let ip = canihazip::plz_ip()?;

assert_eq!(ip.is_loopback(), false);
```

## Contributing

Please feel free to open a PR if there is something that can be improved!

## License

canihazip is licenced under the MIT licence.
