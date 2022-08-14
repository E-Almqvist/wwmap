use crate::util;
use anyhow::{anyhow, Result};
use cidr_utils::cidr::Ipv4Cidr;
use convert_base::Convert;
use std::convert::TryInto;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[derive(Debug, Copy, Clone)]
pub struct IPv4 {
    pub id: u64, // u32
    pub ip: [u8; 4],
}

impl IPv4 {
    pub fn new(id: u64) -> Self {
        if id > u32::max_value() as u64 {
            panic!(
                "IPv4 id is above the IPv4 range! id={id} > {}",
                u32::max_value()
            );
        }

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
        // ip = ip.into_iter().rev().collect();

        // convert to array
        let ip: [u8; 4] = ip
            .try_into()
            .unwrap_or_else(|_: Vec<u8>| panic!("Unable to convert Vec to [u8; 4] for IPv4!"));

        Self { id, ip }
    }

    pub fn to_ipaddr(self: &mut Self) -> Result<IpAddr> { // TODO: remove unneeded Result returns
        if let [a, b, c, d] = self.ip[0..4] {
            Ok(IpAddr::V4(Ipv4Addr::new(a, b, c, d)))
        } else {
            Err(anyhow!("Unable to unpack IPv4 address: {:?}", self.ip))
        }
    }

    pub fn to_socketaddr(self: &mut Self, port: u16) -> Result<SocketAddr> {
        let ip_addr = self.to_ipaddr()?;
        Ok(SocketAddr::new(ip_addr, port))
    }
}

#[derive(Debug)]
pub struct IPv4Range {
    pub id_start: u32,
    pub id_end: u32,
    pub id_ignore: Vec<u32>,
}

impl IPv4Range {
    pub fn new(from: u32, to: u32, id_ignore: Option<Vec<u32>>) -> Self {
        let to = to.clamp(0, u32::max_value());
        let id_ignore = id_ignore.unwrap_or(Vec::new());

        if from > to {
            panic!("Range size must be >= 1! from={} > to={}", from, to);
        }

        Self {
            id_start: from,
            id_end: to,
            id_ignore,
        }
    }

    pub fn from_cidr(cidr_string: String, id_ignore: Option<Vec<u32>>) -> Self {
        let cidr = Ipv4Cidr::from_str(cidr_string).unwrap(); 
        let (from, to) = (cidr.first(), cidr.last()); // TODO: fix forgotten "constants"

        println!("{:?}", cidr.last_as_u8_array());

        Self::new(from, to, id_ignore)
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
