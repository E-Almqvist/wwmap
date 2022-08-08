use anyhow::{Result, anyhow};
use convert_base::Convert;
use crate::util;

/*
Algorithm: O(n)

let i = 0 .. u32:max_value()

# Convert each i to base 256 and we get all the ipv4 addresses
# This is waaaay better than a stupid loop
*/

pub struct IPv4 {
    pub id: u32,
    pub ip: Vec<u8>
}

impl IPv4 {
    pub fn new(id: u32) -> Self {
        let mut base = Convert::new(10, 256);

        let id_vec = util::number_to_vec(id); // push all digits into a vec
        println!("########## {:?}", id_vec);
        let ip = base.convert::<u8, u8>(&id_vec);

        Self { id, ip }
    }
}


pub fn ipv4(blacklist: Option<Vec<[u8; 4]>>) -> Result<Vec<[u8; 4]>> {
    let blacklist = blacklist.unwrap_or(Vec::new());
    let ips: Vec<u32> = (0..u32::max_value()).collect();  // 32 bit max value is the last IP

    //if combos.len() <= 0 {
    Err(anyhow!("Unable to generate IPv4 permutations"))
//     } else {
//         Ok(combos)
//     }
}
