# canihasip-rs
An [unoffical] rust crate for https://icanhazip.com/

## Usage

```rust
extern crate canihazip;

let ip = canihazip::plz_ip()?;

assert_eq!(ip.is_private(), false);
```

## Contributing

Found a bug in my 1 line of application code!? Feel free to make a PR to fix it!
