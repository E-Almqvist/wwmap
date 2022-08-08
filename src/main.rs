#![allow(
    dead_code,
    unused_variables,
    //unused_imports, // TODO: rm
)]

mod ipv4;
mod scanner;
mod util;

use ipv4::IPv4;

fn main() {
    //    scanner::start_scan(100);
    // permutations::ipv4(None);

    let ip = IPv4::new((u32::max_value()) as u64);
    println!("{:?}", ip);
    let ip = IPv4::new(256);
    println!("{:?}", ip);
}
