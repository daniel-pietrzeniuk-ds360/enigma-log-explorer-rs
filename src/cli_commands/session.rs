use std::{fs::{self, File}, io::{self, Write}};

use clap::Parser;

#[derive(Parser, Debug)]
pub struct CliSessionArgs {
    pub input_file_path: String,
    pub session_id: u32,
    /// Print output to convinient file
    #[arg(short, long)]
    pub file: bool,
    /// Overrides output file
    #[arg(short, long)]
    pub r#override: bool,
    // /// Watches for file changes (implicitly sets and requires `end_pattern_mode` to be true)
    // #[arg(short, long)]
    // pub watch: bool,
    /// Alternatively to default mode, writes output until `end_pattern` is encountered
    #[arg(short, long)]
    pub end_pattern_mode: bool,
    /// (implicitly sets and requires `end_pattern_mode` to be true)
    #[arg(long, default_value_t=CliSessionArgs::default_end_pattern())]
    pub end_pattern: String,
}

impl CliSessionArgs {
    fn default_end_pattern() -> String {
        "RUN TASK ENDED".to_string()
    }
}


pub fn session_handler(args: CliSessionArgs) {
    // if args.watch {
    //     args.end_pattern_mode = true;
    // }

    let bytes = fs::read(&args.input_file_path).expect("Failed to read file");

    let file_content = String::from_utf8_lossy(&bytes);
    let file_content: Vec<&str> = file_content.lines().collect();

    let session_id_string = format!("SessionId: {}", args.session_id);

    let mut output_stream: Box<dyn Write> = if args.file
    {
        let output_file_name = format!("{}.session-{}.log", &args.input_file_path, &args.session_id);

        if fs::exists(&output_file_name).unwrap() && !args.r#override {
            panic!("Output file \"{output_file_name}\" already exists")
        }

        Box::new(File::create(output_file_name).unwrap())
    }
    else {
        Box::new(io::stdout())
    };


    if args.end_pattern_mode {
        let mut has_found_session_id = false;

        for line in file_content {
            if !has_found_session_id && line.contains(&session_id_string) {
                has_found_session_id = true;
            }

            if has_found_session_id {
                writeln!(output_stream, "{}", line).expect("Be able to write output");
                // output_stream.write_all(line.as_bytes()).expect("Be able to write output");

                if line.contains(&args.end_pattern) {
                    break;
                }
            }
        }
    } else {
        let mut start_line_idx: Option<usize> = None;
        let mut end_line_idx: Option<usize> = None;

        for (idx, line) in file_content.iter().enumerate() {
            if line.contains(&session_id_string) {
                end_line_idx = Some(idx);

                if start_line_idx == None {
                    start_line_idx = Some(idx);
                }
            }
        }

        if start_line_idx == None {
            panic!("Given file does not contain string (\"{session_id_string}\")");
        } else {
            let skip = start_line_idx.expect("`start_line_idx` should be Some at this point");
            let take = if let Some(end_line_idx) = end_line_idx {
                end_line_idx + 1 - skip
            } else {
                usize::MAX
            };
    
            let result: String = file_content
                .iter()
                .skip(skip)
                .take(take)
                .copied()
                .collect::<Vec<&str>>()
                .join("\n");

            output_stream.write_all(result.as_bytes()).expect("Be able to write output");
        }
    }
}
