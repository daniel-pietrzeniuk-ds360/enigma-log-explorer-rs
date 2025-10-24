use clap::Parser;

mod if_session_can_be_run_in_parallel;
use if_session_can_be_run_in_parallel::*;

#[derive(Parser, Debug)]
pub enum CliCheckArgs {
    /// [alias= '1']
    #[command(alias = "1")]
    IfSessionsCanBeRunInParallel(CliCheckIfSessionsCanBeRunInParallelArgs),
}

pub fn check_handler(args: CliCheckArgs) {
    match args {
        CliCheckArgs::IfSessionsCanBeRunInParallel(args) => {
            if_session_can_be_run_in_parallel::if_session_can_be_run_in_parallel(args);
        }
    }
}

fn log_question(text: &str) {
    println!("Question: {text}");
}

fn log_answer(text: &str) {
    println!("Answer: {text}");
}

fn log_evidence(text: &str) {
    println!("Evidence:\n{text}");
}
