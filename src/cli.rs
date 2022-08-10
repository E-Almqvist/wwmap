use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    name: String,

    #[clap(short, long, value_parser, defualt_value_t = 1)]
    count: u8
}
