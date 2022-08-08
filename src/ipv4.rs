use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use crate::util;
use anyhow::{anyhow, Result};
use convert_base::Convert;

/*
Algorithm: O(n)

let i = 0 .. u32:max_value()

# Convert each i to base 256 and we get all the ipv4 addresses
# This is waaaay better than a stupid loop
*/

#[derive(Debug, PartialEq)]
pub struct IPv4 {
    pub id: u64,
    pub ip: Vec<u8>,
    pub ignore: bool,
}

impl IPv4 {
    pub fn new(id: u64) -> Self {
        let mut base = Convert::new(10, 256);

        let id_vec = util::number_to_vec(id); // push all digits into a vec
        let mut ip = base.convert::<u8, u8>(&id_vec);

        // In case we are missing some digits
        if ip.len() < 4 {
            for i in 0..(4 - ip.len()) {
                ip.insert(0, 0);
            }
        }

        // Reverse it so that we start from the top
        ip = ip.into_iter().rev().collect();

        Self {
            id,
            ip,
            ignore: false,
        }
    }

    pub fn to_ipaddr(self: &mut Self) -> Result<IpAddr> {
        if let [a, b, c, d] = self.ip[0..3] {
            Ok(IpAddr::V4(Ipv4Addr::new(a, b, c, d)))
        } else {
            Err(anyhow!("Unable to unpack IPv4 address"))
        }
    }

    pub fn to_socketaddr(self: &mut Self, port: u16) -> Result<SocketAddr> {
        let ip_addr = self.to_ipaddr()?;
        Ok(SocketAddr::new(ip_addr, port))
    }
}

pub fn get_all(ignorelist: Option<Vec<u64>>) -> Result<Vec<IPv4>> {
    // Ignore those that we know
    let ignorelist = ignorelist.unwrap_or(Vec::new());

    // Get all of the "ids"
    let ids: Vec<u32> = (0..u32::max_value()).collect();

    let ips: Vec<IPv4> = ids
        .iter()
        .map(|&ip| {
            // Make IP
            let mut ip = IPv4::new(ip as u64);

            // Make the IP "ignored" if it is in the ignorelist
            if ignorelist.len() > 0 && ignorelist.contains(&ip.id) {
                ip.ignore = true;
            }

            ip
        })
        .collect();

    Ok(ips)
}
