pub mod session;
pub use session::*;

pub mod simple;
pub use simple::*;

pub mod check;
pub use check::*;

use crate::cli::Cli;

pub fn handle(args: Cli) {
    match args {
        Cli::Session(args) => {
            session_handler(args);
        }
        Cli::Simple(args) => {
            simple_handler(args);
        }
        Cli::Check(args) => {
            check_handler(args);
        }
    }
}
