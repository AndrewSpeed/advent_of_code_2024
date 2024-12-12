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

#[derive(PartialEq, Eq)]
enum ReportLevelDirection {
    Unknown,
    Increasing,
    Decreasing,
}

fn get_file_contents(filepath: PathBuf) -> Result<String> {
    Ok(fs::read_to_string(filepath)?)
}

fn report_is_safe(report_input: &str) -> Option<()> {
    let report = String::from(report_input);
    let mut report_direction = ReportLevelDirection::Unknown;

    for entry in report
        .split_whitespace()
        .map(|value| {
            value
                .parse::<u32>()
                .expect("Unable to convert non-whitespace value to u32")
        })
        .collect::<Vec<u32>>()
        .windows(2)
    {
        let [first_value, second_value]: [_; 2] = entry.try_into().ok()?;

        if first_value.abs_diff(second_value) > 3u32 {
            return None;
        }

        match first_value.cmp(&second_value) {
            std::cmp::Ordering::Greater => {
                if report_direction == ReportLevelDirection::Increasing {
                    return None;
                } else if report_direction == ReportLevelDirection::Unknown {
                    report_direction = ReportLevelDirection::Decreasing;
                }
            }
            std::cmp::Ordering::Equal => {
                return None;
            }
            std::cmp::Ordering::Less => {
                if report_direction == ReportLevelDirection::Decreasing {
                    return None;
                } else if report_direction == ReportLevelDirection::Unknown {
                    report_direction = ReportLevelDirection::Increasing
                }
            }
        }
    }
    Some(())
}

fn count_safe_reports(input: &str) -> Result<u32> {
    let mut safe_report_count = 0;

    for report in input.lines() {
        if report_is_safe(report).is_some() {
            safe_report_count += 1;
        }
    }

    Ok(safe_report_count)
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let input_filepath = args.input_filepath;

    println!(
        "Operating on {}",
        input_filepath
            .to_str()
            .expect("Failed to parse input filepath to string")
    );
    let contents = get_file_contents(input_filepath)?;
    let safe_level_count = count_safe_reports(&contents);

    println!("Safe level count: {:?}", safe_level_count);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_is_safe_all_decreasing() {
        let input = "7 6 4 2 1";

        assert_eq!(report_is_safe(input), Some(()));
    }

    #[test]
    fn test_report_is_safe_too_big_an_increase() {
        let input = "1 2 7 8 9";

        assert_eq!(report_is_safe(input), None);
    }

    #[test]
    fn test_report_is_safe_too_big_a_decrease() {
        let input = "9 7 6 2 1";

        assert_eq!(report_is_safe(input), None);
    }

    #[test]
    fn test_report_is_safe_increase_then_decrease() {
        let input = "1 3 2 4 5";

        assert_eq!(report_is_safe(input), None);
    }

    #[test]
    fn test_report_is_safe_neither_increase_or_decrease() {
        let input = "8 6 4 4 1";

        assert_eq!(report_is_safe(input), None);
    }

    #[test]
    fn test_report_is_safe_all_increasing() {
        let input = "1 3 6 7 9";

        assert_eq!(report_is_safe(input), Some(()));
    }

    #[test]
    fn test_report_all_increasing_within_3_of_previous_value() {
        let input = "61 64 67 68 70 73";

        assert_eq!(report_is_safe(input), Some(()));
    }
}
