mod cli;
mod ipv4;
mod scanner;
mod util;

use clap::Parser;
use cli::Args;

use scanner::start_scan;

fn main() {
    let args = Args::parse();

    let _results = start_scan(args.port, args.threads as u32, None).unwrap();
}
