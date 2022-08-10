mod cli;
mod ipv4;
mod scanner;
mod util;

use clap::Parser;
use cli::Args;

fn main() {
    let args = Args::parse();

    println!("{:?}", args);
}
