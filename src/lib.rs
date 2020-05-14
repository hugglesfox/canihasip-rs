//! # CanIHazIP
//! CanIHazIP gets your current public ip address using https://canihazip.com
//!
//! # Example
//! ```rust
//! extern crate canihazip;
//! # use std::net::Ipv4Addr;
//!
//! # fn main() -> Result<(), reqwest::Error> {
//! let ip = canihazip::plz_ip()?;
//!
//! assert_eq!(ip.is_private(), false);
//! # Ok(())
//! # }
//! ```
use std::net::Ipv4Addr;

/// Get your current public IP.
///
/// Panics if the returned IP is not a valid IPv4 address.
pub fn plz_ip() -> Result<Ipv4Addr, reqwest::Error> {
    Ok(reqwest::blocking::get("https://icanhazip.com/")?
        .text()?
        .trim()
        .parse::<Ipv4Addr>()
        .expect("Error parsing IP address"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ip = plz_ip().unwrap();

        assert_eq!(ip.is_private(), false);
        assert_eq!(ip.is_loopback(), false);
        assert_eq!(ip.is_link_local(), false);
        assert_eq!(ip.is_broadcast(), false);
        assert_eq!(ip.is_documentation(), false);
        assert_eq!(ip.is_unspecified(), false);
    }
}
