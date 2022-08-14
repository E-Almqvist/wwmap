mod cli;
mod ipv4;
mod scanner;
mod util;

use clap::Parser;
use cli::Args;
use ipv4::IPv4Range;
use scanner::start_scan;

fn main() {
    // Get CLI arguments
    let args = Args::parse();

    // Get the IP range
    let range = IPv4Range::from_cidr(args.cidr, None);

    // Start the scan
    let results = start_scan(range, args.port, args.threads);

    for result in results {
        println!("{:?}", result);
    }
}
