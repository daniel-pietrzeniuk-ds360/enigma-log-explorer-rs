use clap::Parser;

mod cli;
use cli::*;

mod cli_commands;

fn main() {
    let args: Cli = Cli::parse();

    cli_commands::handle(args);
}
