use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader, Seek, SeekFrom, Write},
    sync::mpsc::channel,
};

use clap::Parser;
use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher};

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

    /// Watches for file changes (implicitly sets `end_pattern_mode`)
    #[arg(short, long)]
    pub watch: bool,
    /// Skips old changes and watches only for new ones (implicitly sets `watch`)
    #[arg(long)]
    pub watch_new: bool,
    /// Alternatively to default mode, writes output until `end_pattern` is encountered
    #[arg(short, long)]
    pub end_pattern_mode: bool,
    #[arg(long, default_value_t=CliSessionArgs::default_end_pattern())]
    pub end_pattern: String,
}

impl CliSessionArgs {
    fn default_end_pattern() -> String {
        "RUN TASK ENDED".to_string()
    }
}

pub fn session_handler(mut args: CliSessionArgs) {
    if args.watch_new {
        args.watch = true;
    }

    if args.watch {
        args.end_pattern_mode = true;
    }

    let bytes = fs::read(&args.input_file_path).expect("Failed to read file");

    let file_content = String::from_utf8_lossy(&bytes);
    let file_content: Vec<&str> = file_content.lines().collect();

    let session_id_string = format!("SessionId: {}", args.session_id);

    let mut output_stream: Box<dyn Write> = if args.file {
        let output_file_name =
            format!("{}.session-{}.log", &args.input_file_path, &args.session_id);

        if fs::exists(&output_file_name).unwrap() && !args.r#override {
            panic!("Output file \"{output_file_name}\" already exists")
        }

        Box::new(File::create(output_file_name).unwrap())
    } else {
        Box::new(io::stdout())
    };

    if args.end_pattern_mode {
        if !args.watch_new {
            let mut has_found_session_id = false;

            for line in file_content {
                if !has_found_session_id && line.contains(&session_id_string) {
                    has_found_session_id = true;
                }

                if has_found_session_id {
                    writeln!(output_stream, "{line}").expect("Be able to write output");

                    if line.contains(&session_id_string) && line.contains(&args.end_pattern) {
                        dbg!(
                            "end_pattern_mode prepass finished due finding end_pattern on line: \"{:?}\"",
                            line
                        );
                        return;
                    }
                }
            }
        }

        if args.watch {
            let (tx, rx) = channel();

            let mut watcher = RecommendedWatcher::new(tx, notify::Config::default()).unwrap();
            watcher
                .watch(
                    std::path::Path::new(&args.input_file_path),
                    RecursiveMode::NonRecursive,
                )
                .unwrap();

            let file = File::open(&args.input_file_path).unwrap();
            let mut pos = file.metadata().unwrap().len();
            let path = args.input_file_path;

            loop {
                if let Ok(event) = rx.recv().expect("To receive an filesystem event") {
                    if let EventKind::Modify(_) = event.kind {
                        let mut file = File::open(&path).unwrap();
                        file.seek(SeekFrom::Start(pos)).unwrap();
                        let reader = BufReader::new(file);

                        for line in reader.lines() {
                            let line = line.unwrap();

                            writeln!(output_stream, "{line}").expect("Be able to write output");

                            if line.contains(&session_id_string) && line.contains(&args.end_pattern)
                            {
                                return;
                            }
                        }

                        pos = std::fs::metadata(&path).unwrap().len();
                    }
                }
            }
        }
    } else {
        let mut start_line_idx: Option<usize> = None;
        let mut end_line_idx: Option<usize> = None;

        for (idx, line) in file_content.iter().enumerate() {
            if line.contains(&session_id_string) {
                end_line_idx = Some(idx);

                if start_line_idx.is_none() {
                    start_line_idx = Some(idx);
                }
            }
        }

        if start_line_idx.is_none() {
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

            output_stream
                .write_all(result.as_bytes())
                .expect("Be able to write output");
        }
    }
}
