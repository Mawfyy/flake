use crate::cli::process;
use crate::cli::Args;
use clap::Parser;

mod cli;

fn main() {
    let args: Args = Args::parse();
    let result = process(args);

    match result {
        Err(error) => println!("{:?}", error),
        Ok(_) => (),
    }
}
