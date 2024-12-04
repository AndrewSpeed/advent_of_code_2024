use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// our input
    #[arg(value_name = "FILE")]
    input_filepath: PathBuf,
}

fn get_file_contents(filepath: PathBuf) -> Result<String> {
    Ok(fs::read_to_string(filepath)?)
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let input_filepath = args.input_filepath;

    println!("Hello, world!");
    println!(
        "Operating on {}",
        input_filepath
            .to_str()
            .expect("Failed to parse input filepath to string")
    );
    let _contents = get_file_contents(input_filepath)?;
    Ok(())
}
