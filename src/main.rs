use anyhow::{Context, Result};
use chrono::NaiveDate;
use clap::Parser;

#[derive(Parser)]
#[command(name = "strava-cli")]
#[command(about = "A CLI tool to fetch kilometers from Strava since a given date")]
struct Args {
    /// Start date in YYYY-MM-DD format
    #[arg(short, long)]
    date: String,

    /// Strava access token (will be required for actual API calls)
    #[arg(short, long)]
    token: Option<String>,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if args.verbose {
        println!("Starting Strava CLI tool...");
    }

    // Parse the input date
    let start_date = parse_date(&args.date).context("Failed to parse the provided date")?;

    if args.verbose {
        println!("Parsed start date: {start_date}");
    }

    // For now, just simulate fetching data
    let total_km = fetch_strava_data_since(start_date, args.token).await?;

    println!("Total kilometers since {}: {:.2} km", args.date, total_km);

    Ok(())
}

/// Parse a date string in YYYY-MM-DD format
fn parse_date(date_str: &str) -> Result<NaiveDate> {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d").context("Date must be in YYYY-MM-DD format")
}

/// Fetch Strava data since the given date
/// For now, this is a mock implementation that returns a dummy value
async fn fetch_strava_data_since(start_date: NaiveDate, _token: Option<String>) -> Result<f64> {
    // Mock implementation - in reality this would call Strava API
    println!("Fetching Strava data since {start_date}...");

    // Simulate some processing time
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Return a dummy value for now
    Ok(42.5)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    #[test]
    fn test_parse_valid_date() {
        let result = parse_date("2024-01-15");
        assert!(result.is_ok());
        let date = result.unwrap();
        assert_eq!(date.year(), 2024);
        assert_eq!(date.month(), 1);
        assert_eq!(date.day(), 15);
    }

    #[test]
    fn test_parse_invalid_date_format() {
        let result = parse_date("15-01-2024");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_date() {
        let result = parse_date("2024-13-45");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_empty_date() {
        let result = parse_date("");
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_fetch_strava_data_returns_value() {
        let start_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let result = fetch_strava_data_since(start_date, None).await;
        assert!(result.is_ok());
        assert!(result.unwrap() > 0.0);
    }
}
