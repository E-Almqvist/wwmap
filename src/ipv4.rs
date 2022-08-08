use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use anyhow::{Result, anyhow};
use convert_base::Convert;
use crate::util;

/*
Algorithm: O(n)

let i = 0 .. u32:max_value()

# Convert each i to base 256 and we get all the ipv4 addresses
# This is waaaay better than a stupid loop
*/

#[derive(Debug)]
pub struct IPv4 {
    pub id: u64,
    pub ip: Vec<u8>
}

impl IPv4 {
    pub fn new(id: u64) -> Self {
        let mut base = Convert::new(10, 256);

        let id_vec = util::number_to_vec(id); // push all digits into a vec
        let mut ip = base.convert::<u8, u8>(&id_vec);

        // In case we are missing some digits
        if ip.len() < 4 {
            for i in 0..(4-ip.len()) {
                ip.insert(0, 0);
            }
        }

        // Reverse it so that we start from the top
        ip = ip.into_iter().rev().collect();

        Self { id, ip }
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


pub fn get_all(blacklist: Option<Vec<[u8; 4]>>) -> Result<Vec<[u8; 4]>> {
    let blacklist = blacklist.unwrap_or(Vec::new());
    let ips: Vec<u32> = (0..u32::max_value()).collect();  // 32 bit max value is the last IP

    //if combos.len() <= 0 {
    Err(anyhow!("Unable to generate IPv4 permutations"))
//     } else {
//         Ok(combos)
//     }
}