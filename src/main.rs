use anyhow::Error;
use clap::Parser;
use notify::{watcher, RecursiveMode, Watcher};
use std::process::Command;
use std::sync::mpsc::channel;

#[derive(Parser, Debug)]
#[clap(
    name = "run-when",
    author = "Aalekh Patel", 
    version = "1.0.2", 
    about = "Run a (debounced) command upon changes to the filesystem.",
    long_about = None
)]
pub struct Args {
    /// The debounce period (i.e. wait for a duration of X before running the specified executable).
    #[clap(short = 't', long, value_parser, default_value = "600ms")]
    pub debounce_period: String,

    /// Whether to watch a directory recursively.
    #[clap(short = 'r', long, value_parser, default_value_t = false)]
    pub recursive: bool,

    /// The file/directory to watch. If a directory is specified, will watch all files in it (but not recursively, unless -r is also specified).
    #[clap(short = 'f', long, value_parser)]
    pub file: String,

    /// An executable to run once a change is detected.
    #[clap(short = 'c', long, value_parser)]
    pub command_file: String,
}

/// The main function.
fn main() -> Result<(), Error> {
    let args = Args::parse();
    let duration = parse_duration::parse(&args.debounce_period)?;

    let (tx, rx) = channel();
    let mut watcher = watcher(tx, duration)?;

    watcher.watch(
        args.file.clone(),
        match args.recursive {
            true => RecursiveMode::Recursive,
            false => RecursiveMode::NonRecursive,
        },
    )?;

    loop {
        match rx.recv() {
            Ok(_) => {
                if let Err(e) = Command::new(args.command_file.clone()).output() {
                    eprintln!(
                        "Error occurred when command {:?} was executed: {:?}",
                        e, &args.command_file
                    );
                } else {
                    println!("Changes detected in {:?}. Running '{:?}'", args.file.clone(), args.command_file.clone());
                }
            }
            Err(e) => {
                eprintln!("Failed to watch {e:?}");
            }
        }
    }
}
