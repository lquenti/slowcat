use clap::Parser;
use std::{fs::File, io::{self, BufRead, BufReader}, thread, time::Duration};

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

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let delay_duration = Duration::from_secs_f64(cli.delay);

    for file in cli.files {
        print_file_slowly(&file, delay_duration)?;
    }

    Ok(())
}

fn print_file_slowly(file_path: &str, delay: Duration) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
        thread::sleep(delay);
    }

    Ok(())
}

