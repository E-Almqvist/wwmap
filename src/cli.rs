use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    name: String,
    count: u8
}
