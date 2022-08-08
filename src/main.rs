#![allow(
    dead_code,
    unused_variables,
    //unused_imports, // TODO: rm
)]

mod util;
mod scanner;
mod ipv4;

use ipv4::IPv4;

fn main() {
    //    scanner::start_scan(100);
    // permutations::ipv4(None);

    let ip = IPv4::new((u32::max_value() - 256 - 255) as u64);
    println!("{:?}", ip);
    let ip = IPv4::new(256);
    println!("{:?}", ip);
}
