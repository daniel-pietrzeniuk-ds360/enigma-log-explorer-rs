use std::fs;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct CliSimpleArgs {
    pub input_file_path: String,
    pub start_line_contains: String,
    /// If ommited uses `start_line_contains` as this arg
    pub end_line_contains: Option<String>,
}

pub fn simple_handler(args: CliSimpleArgs) {
    let bytes = fs::read(&args.input_file_path).expect("Failed to read file");

    // Converts bytes to UTF-8, replacing invalid sequences with �
    let file_content = String::from_utf8_lossy(&bytes);

    let file_content: Vec<&str> = file_content.lines().collect();

    let end_line_pattern = args
        .end_line_contains
        .unwrap_or(args.start_line_contains.clone());
    let start_line_pattern = args.start_line_contains;

    let mut is_input_file_containing_start_line_pattern = false;

    for line in file_content.iter() {
        if !is_input_file_containing_start_line_pattern
            && line.contains(&start_line_pattern)
        {
            is_input_file_containing_start_line_pattern = true;
        }

        if is_input_file_containing_start_line_pattern {
            println!("{line}")
        }

        if line.contains(&end_line_pattern) {
            break;
        }
    }

    if !is_input_file_containing_start_line_pattern {
        panic!("Given file does not contain string (\"{start_line_pattern}\")");
    }
}
