use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(name = "World Wide Mapping", version, about = "Scan the world wide web for a certian port.", long_about = None)]
pub struct Args {
    #[clap(help = "Which port to scan for.", short = 'p', long = "port")]
    pub port: u16,

    #[clap(
        help = "Amount of threads that will be used when scanning for the specified port.",
        short = 'n',
        long = "threads",
        default_value_t = 1
    )]
    pub threads: u8,

    #[clap(
        help = "A file containing ignored IPv4 addresses (seperated by linebreaks).",
        short = 'i',
        long = "ignore-ip-list",
        default_value = "ignore-ips-list.txt"
    )]
    pub ignorelist: PathBuf,

    #[clap(help = "From IPv4 -", short = 'f', long = "from", default_value_t = 0)]
    pub from: u32,

    #[clap(
        help = "To IPv4 -",
        short = 't',
        long = "to",
        default_value_t = 4294967295
    )]
    pub to: u32,

    #[clap(
        help = "Enable verbose (debug) output",
        short = 'v',
        long = "verbose",
        takes_value = false,
        required = false
    )]
    pub verbose: bool,
}
