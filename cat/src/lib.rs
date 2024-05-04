use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::{App, Arg};

type GenericResult<T> = Result<T, Box<dyn Error>>;

pub struct Config {
    files: Vec<String>,
    number: bool,
    number_non_blank_line: bool,
}

pub fn run(config: Config) -> GenericResult<()> {
    for file in config.files {
        match open(&file) {
            Ok(file) => {
                let mut number = 1;
                for line in file.lines() {
                    let line = line?;
                    if config.number {
                        println!("{:>6}\t{}", number, line);
                        number += 1;
                    } else if config.number_non_blank_line {
                        if line.is_empty() {
                            println!();
                        } else {
                            println!("{:>6}\t{}", number, line);
                            number += 1;
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to open file: {}, error: {}", file, e)
            }
        }
    }
    Ok(())
}

pub fn parse_args() -> GenericResult<Config> {
    let matches = App::new("cat")
        .version("0.0.1")
        .author("max")
        .about("simplified cat for rust learning")
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .help("number lines")
                .takes_value(false)
                .conflicts_with("number_non_blank"),
        )
        .arg(
            Arg::with_name("number_non_blank")
                .short("b")
                .long("number-non-blank")
                .help("number non-blank lines")
                .takes_value(false)
                .conflicts_with("number"),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("file").unwrap(),
        number: matches.is_present("number"),
        number_non_blank_line: matches.is_present("number_non_blank"),
    })
}

fn open(filename: &str) -> GenericResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
