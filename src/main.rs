#![allow(
    dead_code,
    unused_variables,
    //unused_imports, // TODO: rm
)]

mod util;
mod scanner;
mod permutations;

fn main() {
    //    scanner::start_scan(100);
    // permutations::ipv4(None);

    let ip = permutations::IPv4::new(0);
}
