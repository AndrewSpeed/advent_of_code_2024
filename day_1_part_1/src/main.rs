use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// our input
    #[arg(value_name = "FILE")]
    input_filepath: PathBuf,
}

fn main() {
    let args = Cli::parse();
    let input_filepath = args.input_filepath;

    println!("Hello, world!");
    println!(
        "Operating on {}",
        input_filepath
            .to_str()
            .expect("Failed to parse input filepath to string")
    );
}
