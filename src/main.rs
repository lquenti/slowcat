use clap::Parser;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    thread,
    time::Duration,
};
use thiserror::Error;

/// Custom error types for slowcat
#[derive(Error, Debug)]
enum SlowCatError {
    #[error("Error opening file: \"{0}\"")]
    FileOpenError(io::Error),
    #[error("Error reading from file: \"{0}\"")]
    FileReadError(io::Error),
}

/// SlowCat prints the contents of files with a delay between lines
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Files to process
    files: Vec<String>,
    /// Delay in seconds between lines
    #[clap(short = 'n', long, default_value_t = 0.2)]
    delay: f64,
}

fn main() {
    let cli = Cli::parse();
    let delay_duration = Duration::from_secs_f64(cli.delay);

    for file in cli.files {
        if let Err(e) = print_file_slowly(&file, delay_duration) {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn print_file_slowly(file_path: &str, delay: Duration) -> Result<(), SlowCatError> {
    let file = File::open(file_path).map_err(SlowCatError::FileOpenError)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.map_err(SlowCatError::FileReadError)?;
        println!("{}", line);
        thread::sleep(delay);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_args_allowed() {
        let args = ["slowcat"];
        let _cli = Cli::parse_from(&args);
    }

    #[test]
    fn test_cli_args() {
        let args = ["slowcat", "test.txt", "-n", "0.5"];
        let cli = Cli::parse_from(&args);
        assert_eq!(cli.files, vec!["test.txt"]);
        assert_eq!(cli.delay, 0.5);
    }

    #[test]
    fn test_file_open_error() {
        let result = print_file_slowly("non_existent_file.txt", Duration::from_secs_f64(0.2));
        assert!(matches!(result, Err(SlowCatError::FileOpenError(_))));
    }
}
