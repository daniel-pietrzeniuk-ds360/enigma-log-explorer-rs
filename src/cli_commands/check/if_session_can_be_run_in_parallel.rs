use std::{collections::HashSet, fs};

use clap::Parser;

use crate::cli_commands::check::{log_answer, log_evidence, log_question};

#[derive(Parser, Debug)]
pub struct CliCheckIfSessionsCanBeRunInParallelArgs {
    pub input_file_path: String,
}

pub fn if_session_can_be_run_in_parallel(args: CliCheckIfSessionsCanBeRunInParallelArgs) {
    log_question("Whether sessions may be ran in parallel?");

    let bytes = fs::read(&args.input_file_path).expect("Failed to read file");

    // Converts bytes to UTF-8, replacing invalid sequences with �
    let file_content = String::from_utf8_lossy(&bytes);

    // let file_content = read_to_string(&session_args.input_file_path)
    //     .expect(&format!("\"{}\" should be a valid file", &session_args.input_file_path));
    let file_content: Vec<&str> = file_content.lines().collect();

    let mut curr_session_id = i32::MIN;

    let mut evidence = Vec::<String>::new();
    let mut changes = HashSet::<(i32, i32)>::new();

    for (idx, line) in file_content.iter().enumerate() {
        const SEARCHED_STRING: &str = "SessionId";

        if let Some(col) = line.find(SEARCHED_STRING) {
            let session_id_start = col + SEARCHED_STRING.len() + 2;
            let session_id_end = session_id_start
                + line[session_id_start..]
                    .find(',')
                    .expect("session_id pattern ends with ',' char");

            let session_id = str::parse(&line[session_id_start..session_id_end]).unwrap();

            if session_id > curr_session_id {
                curr_session_id = session_id;
            } else if session_id < curr_session_id
                && !changes.contains(&(session_id, curr_session_id))
            {
                evidence.push(format!("Line: {idx} - changes from SessionId({curr_session_id}) to SessionId({session_id})"));
                changes.insert((session_id, curr_session_id).clone());
                curr_session_id = session_id;
            }
        }
    }

    if !evidence.is_empty() {
        log_answer("Probably true");
        log_evidence(&format!(
            r#"
Unique SessionId changes to previous number
{}
"#,
            evidence
                .iter()
                .map(|t| { format!("- {t}") })
                .collect::<Vec<String>>()
                .join("\n")
        ));
    } else {
        log_answer("Inconclusive (there are no sessions ran in parallel)");
    }
}
