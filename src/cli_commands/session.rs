use std::fs;

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
}

pub fn session_handler(args: CliSessionArgs) {
    let bytes = fs::read(&args.input_file_path).expect("Failed to read file");

    // Converts bytes to UTF-8, replacing invalid sequences with �
    let file_content = String::from_utf8_lossy(&bytes);

    // let file_content = read_to_string(&session_args.input_file_path)
    //     .expect(&format!("\"{}\" should be a valid file", &session_args.input_file_path));
    let file_content: Vec<&str> = file_content.lines().collect();

    let mut start_line_idx: Option<usize> = None;
    let mut end_line_idx: Option<usize> = None;

    let session_id_string = format!("SessionId: {}", args.session_id);

    // let output_stream: Box<dyn Write> = if session_args.file
    // {
    //     let output_file_name = format!("{}.session-{}.log", &session_args.input_file_path, &session_args.session_id);

    //     if fs::exists(&output_file_name).unwrap() && !session_args.r#override {
    //         panic!("Output file \"{output_file_name}\" already exists")
    //     }

    //     // File::options().create(true).write(true).truncate(true).open(output_file_name).unwrap()

    //     Box::new(File::create(output_file_name).unwrap())

    //     // fs::write(output_file_name, result);
    // }
    // else {
    //     Box::new(io::stdout())
    //     // println!("{}", result);
    // };

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

        if args.file {
            let output_file_name =
                format!("{}.session-{}.log", &args.input_file_path, &args.session_id);

            if fs::exists(&output_file_name).unwrap() && !args.r#override {
                panic!(
                    "Output file \"{output_file_name}\" already exists (consider --override flag)"
                )
            }

            fs::write(output_file_name, result).unwrap();
        } else {
            println!("{}", result);
        }
    }
}
