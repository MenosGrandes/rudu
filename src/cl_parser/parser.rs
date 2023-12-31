use clap::{arg, command, value_parser, Arg, ArgAction, Command};
use std::{env, path::PathBuf};

//https://man7.org/linux/man-pages/man1/du.1.html
pub struct CliOption {
    pub file: PathBuf,
    pub max_depth: u8,
}

impl CliOption {
    pub fn new(file: PathBuf, max_depth: u8) -> Self {
        Self { file, max_depth }
    }
}

impl Default for CliOption {
    fn default() -> Self {
        Self {
            file: env::current_dir().unwrap(),
            max_depth: u8::MAX,
        }
    }
}

pub struct CliParser {}
impl CliParser {
    pub fn parse() -> CliOption {
        let matches = command!() // requires `cargo` feature
            .arg(
                arg!([FILE] "root file to start rudu")
                    .value_parser(value_parser!(PathBuf))
            )
            .arg(
                arg!([max_depth] "print the total for a directory (or file, with --all) only
              if it is N or fewer levels below the command line
              argument;  --max-depth=0 is the same as --summarize")
                    .value_parser(value_parser!(u8))
                    .long("max_depth")
            )
            .get_matches();

        let mut options = CliOption::default();
        if let Some(path) = matches.get_one::<PathBuf>("FILE") {
            options.file = path.to_path_buf();
        }
        if let Some(max_depth) = matches.get_one::<u8>("max_depth") {
            options.max_depth = *max_depth;
        }
        options
    }
}
