mod cli;
mod ipv4;
mod scanner;
mod util;

use clap::Parser;
use cli::Args;

fn main() {
    let _args = Args::parse();

    //let _results = start_scan(args.port, args.threads as u32, None).unwrap();
}
