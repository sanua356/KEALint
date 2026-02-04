use crate::modes::{CLIArgs, KEALintModeTypes, run_cli, run_standalone};
use clap::Parser;

mod checkers;
mod common;
mod configs;
mod constants;
mod modes;
mod rules;
mod utils;

fn main() {
    let args: CLIArgs = CLIArgs::parse();
    match args.mode {
        KEALintModeTypes::cli => {
            run_cli(args);
        }
        KEALintModeTypes::standalone => {
            run_standalone(args);
        }
    }
}
