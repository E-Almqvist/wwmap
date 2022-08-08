#![allow(
    dead_code,
    unused_variables,
    //unused_imports, // TODO: rm
)]

mod scanner;
mod permutations;

fn main() {
    //    scanner::start_scan(100);
    permutations::ipv4(None);
}
