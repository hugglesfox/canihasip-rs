//! # CanIHazIP
//! CanIHazIP gets your current public ip address using [icanhazip](https://icanhazip.com)
//!
//! # Example
//! ```rust
//! # async fn run() -> Result<(), reqwest::Error> {
//! let ip = canihazip::plz_ip().await?;
//! # Ok(())
//! # }
//! ```

#[cfg(test)]
extern crate tokio;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

pub type Error = reqwest::Error;


/// Get your current public IP address.
///
/// Panics if the returned IP is not a valid IP address.
pub async fn plz_ip() -> Result<IpAddr, Error> {
    Ok(reqwest::get("https://icanhazip.com/")
        .await?
        .text()
        .await?
        .trim()
        .parse::<IpAddr>()
        .expect("Error parsing IP address"))
}

/// Get your current public IPv4 address.
///
/// Panics if the returned IP is not a valid IPv4 address.
pub async fn plz_ipv4() -> Result<Ipv4Addr, Error> {
    Ok(reqwest::get("https://4.icanhazip.com/")
        .await?
        .text()
        .await?
        .trim()
        .parse::<Ipv4Addr>()
        .expect("Error parsing IPv4 address"))
}

/// Get your current public IPv6 address.
///
/// Errors if there is no Ipv6 connectivity.
/// Panics if the returned IP is not a valid IPv6 address.
pub async fn plz_ipv6() -> Result<Ipv6Addr, Error> {
    Ok(reqwest::get("https://6.icanhazip.com/")
        .await?
        .text()
        .await?
        .trim()
        .parse::<Ipv6Addr>()
        .expect("Error parsing IPv6 address"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_plz_ip() {
        let ip = plz_ip().await.unwrap();

        assert_eq!(ip.is_loopback(), false);
        assert_eq!(ip.is_unspecified(), false);
    }

    #[tokio::test]
    async fn test_plz_ipv4() {
        let ip = plz_ipv4().await.unwrap();

        assert_eq!(ip.is_private(), false);
        assert_eq!(ip.is_loopback(), false);
        assert_eq!(ip.is_link_local(), false);
        assert_eq!(ip.is_broadcast(), false);
        assert_eq!(ip.is_documentation(), false);
        assert_eq!(ip.is_unspecified(), false);
    }

    #[tokio::test]
    #[ignore]
    async fn test_plz_ipv6() {
        let ip = plz_ipv6().await.unwrap();

        assert_eq!(ip.is_loopback(), false);
        assert_eq!(ip.is_unspecified(), false);
        assert_eq!(ip.is_multicast(), false);
    }
}
