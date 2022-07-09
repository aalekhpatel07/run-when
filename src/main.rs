use notify::{Watcher, RecursiveMode, watcher};
use std::sync::mpsc::channel;
use clap::Parser;
use anyhow::Error;
use std::process::Command;
use parse_duration;

#[derive(Parser, Debug)]
#[clap(
    author = "Aalekh Patel", 
    version = "0.1.0", 
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
    pub command_file: String
}

/// The main function.
fn main() -> Result<(), Error>{
    
    let args = Args::parse();
    let duration = parse_duration::parse(&args.debounce_period)?;

    let (tx, rx) = channel();
    let mut watcher = watcher(tx, duration)?;
    
    watcher.watch(
        args.file.clone(),
        match args.recursive {
            true => RecursiveMode::Recursive,
            false => RecursiveMode::NonRecursive
        }
    )?;

    loop {
        match rx.recv() {
            Ok(_) => {
                if let Err(e) = Command::new(args.command_file.clone())
                    .spawn()
                {
                    eprintln!(
                        "Error occurred when command {:?} was executed: {:?}",
                        e,
                        &args.command_file
                    );
                }
            },
            Err(e) => {
                eprintln!("Failed to watch {e:?}");
            }
        }
    }
}