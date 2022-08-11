use crate::util;
use anyhow::{anyhow, Result};
use convert_base::Convert;
use std::convert::TryInto;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[derive(Debug, Copy, Clone)]
pub struct IPv4 {
    pub id: u64,
    pub ip: [u8; 4],
    pub ignore: bool,
}

impl IPv4 {
    pub fn new(id: u64) -> Self {
        let mut base = Convert::new(10, 256);

        let id_vec = util::number_to_vec(id); // push all digits into a vec
        let mut ip = base.convert::<u8, u8>(&id_vec);

        // In case we are missing some digits
        if ip.len() < 4 {
            for _ in 0..(4 - ip.len()) {
                ip.insert(0, 0);
            }
        }

        // Reverse it so that we start from the top
        ip = ip.into_iter().rev().collect();

        // convert to array
        let ip: [u8; 4] = ip
            .try_into()
            .unwrap_or_else(|_: Vec<u8>| panic!("Unable to convert Vec to [u8; 4] for IPv4!"));

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

pub struct IPv4Range {
    pub id_start: u32,
    pub id_end: u32,
    pub id_ignore: Vec<u32>,
}

impl IPv4Range {
    pub fn new(from: u32, to: u32, id_ignore: Option<Vec<u32>>) -> Self {
        to = to.clamp(0, u32::max_value());

        if from >= to {
            panic!("Range size must be >= 1! from: {} >= to: {}", from, to);
        }

        Self {
            id_start: from,
            id_end: to,
            id_ignore: id_ignore.unwrap_or(Vec::new()),
        }
    }
}

impl Iterator for IPv4Range {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.id_start == self.id_end {
            None
        } else {
            let res = Some(self.id_start);
            self.id_start += 1;

            if self.id_ignore.contains(&res.unwrap()) {
                return self.next();
            }
            res
        }
    }
}

pub fn get_all(ignorelist: Option<Vec<u64>>) -> Result<Vec<IPv4>> {
    // Ignore those that we know
    let ignorelist = ignorelist.unwrap_or(Vec::new());

    // Get all of the "ids"
    let mut ips: Vec<IPv4> = Vec::new();

    for id in 0..u32::max_value() {
        // Make IP
        let mut ip = IPv4::new(id as u64);

        // Make the IP "ignored" if it is in the ignorelist
        ip.ignore = ignorelist.contains(&ip.id);

        ips.push(ip);
    }

    Ok(ips)
}
