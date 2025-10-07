use clap::Parser;

use crate::cli_commands::{CliCheckArgs, CliSessionArgs, CliSimpleArgs};

/// Enigma Log Explorer - tool for analyzing Enigma+ logs
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub enum Cli {
    /// [alias= 'id'] Gets all lines between first and last mention of provided session_id
    #[command(aliases = ["id"])]
    Session(CliSessionArgs),
    /// Filter any lines between two lines containing some string
    Simple(CliSimpleArgs),
    /// Perform different checks on enigma log files, run `enigma-log-explorer check help` for more info
    #[command(subcommand)]
    Check(CliCheckArgs),
}
