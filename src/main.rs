use crate::cli::CLIArgs;
use clap::Parser;

mod checkers;
mod cli;
mod common;
mod configs;
mod constants;
mod rules;
mod utils;

fn main() {
    let args: CLIArgs = CLIArgs::parse();
    cli::run_cli(args);
}
