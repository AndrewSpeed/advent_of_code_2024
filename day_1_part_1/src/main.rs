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

fn parse_split_entry(entry: &str) -> Option<i64> {
    let value = entry
        .parse::<i64>()
        .expect(&format!("Unable to parse line element: {:?}", entry));

    Some(value)
}

fn lines_to_lists(input: &str) -> (Vec<i64>, Vec<i64>) {
    let mut left_list: Vec<i64> = vec![];
    let mut right_list: Vec<i64> = vec![];

    for line in input.lines() {
        let mut split = line.split_whitespace();

        left_list.push(
            parse_split_entry(split.next().expect("Unable to retrieve first line element"))
                .expect("Unable to parse first split"),
        );

        right_list.push(
            parse_split_entry(
                split
                    .next()
                    .expect("Unable to retrieve second line element"),
            )
            .expect("Unable to parse second split"),
        );
    }

    (left_list, right_list)
}

fn list_distance(mut left_list: Vec<i64>, mut right_list: Vec<i64>) -> Result<u64> {
    left_list.sort();
    right_list.sort();

    let result = left_list
        .iter()
        .zip(right_list.iter())
        .fold(0, |acc, (left_elem, right_elem)| {
            let diff = (left_elem - right_elem).abs();

            acc + diff
        });

    Ok(result.try_into().unwrap())
}

fn total_list_distance(input: &str) -> Result<u64> {
    let (left_list, right_list) = lines_to_lists(input);
    list_distance(left_list, right_list)
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

    let contents = get_file_contents(input_filepath)?;
    let list_distance_total = total_list_distance(&contents)?;

    println!("List distance: {}", list_distance_total);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_list_distance() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";

        assert_eq!(total_list_distance(input).unwrap(), 11);
    }
}
