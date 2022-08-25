use clap::Parser;
// use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(name = "World Wide Mapping", version, about = "Scan the world wide web for a certian port.", long_about = None)]
pub struct Args {
    #[clap(help = "Which port to scan for.")]
    pub port: u16,

    #[clap(
        help = "Amount of threads that will be used when scanning for the specified port.",
        short = 'n',
        long = "threads",
        default_value_t = 1
    )]
    pub threads: u64,

    //     #[clap(
    //         help = "A file containing ignored IPv4 addresses (seperated by linebreaks).",
    //         short = 'i',
    //         long = "ignore-ip-list",
    //         default_value = "ignore-ips-list.txt"
    //     )]
    //     pub ignorelist: PathBuf,
    #[clap(
        help = "Enable verbose (debug) output",
        short = 'v',
        long = "verbose",
        takes_value = false,
        required = false
    )]
    pub verbose: bool,

    #[clap(
        help = "IPv4 subnet range (CIDR). Leave empty for the whole internet.",
        default_value = "0.0.0.0/0",
        required = false
    )]
    pub cidr: String,

    #[clap(
        help = "Timeout duration in nanoseconds.",
        default_value_t = 0,
        short = 't',
        long = "timeout-ns",
        required = false
    )]
    pub timeout_ns: u32,

    #[clap(
        help = "Timeout duration in seconds.",
        default_value_t = 1,
        short = 'T',
        long = "timeout",
        required = false
    )]
    pub timeout: u64,
}
