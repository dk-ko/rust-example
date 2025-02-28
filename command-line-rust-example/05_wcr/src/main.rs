use std::fs::File;
use std::io::{self, BufRead, BufReader};

use anyhow::Result;
use clap::{Arg, ArgAction, Command, Parser};

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(mut args: Args) -> Result<()> {
    println!("{args:#?}");

    for filename in &args.files {
        match open(filename) {
            Err(err) => eprintln!("{filename}: {err}"),
            Ok(_) => println!("Opened {filename}"),
        }
    }
    Ok(())
}

#[derive(Debug, Parser)]
struct Args {
    files: Vec<String>,
    lines: bool, 
    words: bool,
    bytes: bool,
    chars: bool,
}

fn get_args() -> Args {
    let matches = Command::new("wcr")
    .version("0.1.0")
    .author("jade.ko")
    .about("Rust version of `wc`")
    .arg(
        Arg::new("files")
        .value_name("FILE")
        .help("Input file(s)")
        .default_value("-")
        .num_args(0..),
    )
    .arg(
        Arg::new("lines")
        .short('l')
        .long("lines")
        .action(ArgAction::SetTrue)
        .help("Show line count"),
    )
    .arg(
        Arg::new("words")
        .short('w')
        .long("words")
        .action(ArgAction::SetTrue)
        .help("Show word count"),
    )
    .arg(
        Arg::new("bytes")
        .short('c')
        .long("bytes")
        .action(ArgAction::SetTrue)
        .help("Show byte count"),
    )
    .arg(
        Arg::new("chars")
        .short('m')
        .long("chars")
        .action(ArgAction::SetTrue)
        .help("Show character count")
        .conflicts_with("bytes"),
    )
    .get_matches();

    Args{
        files: matches.get_many("files").unwrap().cloned().collect(),
        lines: matches.get_flag("lines"),
        words: matches.get_flag("words"),
        bytes: matches.get_flag("bytes"),
        chars: matches.get_flag("chars"),
    }
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}