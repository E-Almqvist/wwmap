mod cli;
mod ipv4;
mod scanner;
mod util;

use clap::Parser;
use cli::Args;
use ipv4::IPv4Range;
use scanner::start_scan;

fn main() {
    // Init the logger
    env_logger::init();

    // Get the CLI arguments
    let args = Args::parse();

    // Get the IP range
    let range = IPv4Range::from_cidr(args.cidr, None);

    // Start the scan
    let _results = start_scan(range, args.port, args.threads);
}
