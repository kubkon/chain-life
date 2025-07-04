use anyhow::{Context, Result};
use chrono::NaiveDate;
use clap::{Parser, Subcommand};
use colored::*;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use url::Url;
use uuid::Uuid;

// Common cycling activity types in Strava
const CYCLING_TYPES: &[&str] = &[
    "Ride",
    "VirtualRide", 
    "EBikeRide",
    "MountainBikeRide",
    "GravelRide",
    "Handcycle",
];

// Common running activity types in Strava
const RUNNING_TYPES: &[&str] = &[
    "Run",
    "TrailRun",
    "Treadmill",
    "VirtualRun",
];

// Other common activity types
const OTHER_TYPES: &[&str] = &[
    "Walk",
    "Hike", 
    "Swim",
    "Rowing",
    "Kayaking",
    "Canoeing",
    "StandUpPaddling",
    "Surfing",
    "Kitesurf",
    "Windsurf",
    "Sail",
    "Snowboard",
    "Ski",
    "BackcountrySki",
    "NordicSki",
    "Snowshoe",
    "RockClimbing",
    "IceClimbing",
    "AlpineSki",
    "Elliptical",
    "StairStepper",
    "WeightTraining",
    "Workout",
    "Crossfit",
    "Yoga",
    "Golf",
];

#[derive(Parser)]
#[command(name = "strava-cli")]
#[command(about = "A CLI tool to fetch kilometers from Strava since a given date")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Authenticate with Strava using OAuth
    Auth {
        /// Your Strava application's Client ID
        #[arg(short = 'i', long)]
        client_id: String,

        /// Your Strava application's Client Secret
        #[arg(short = 's', long)]
        client_secret: String,

        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },
    /// Fetch kilometers data from Strava
    Fetch {
        /// Start date in YYYY-MM-DD format
        #[arg(short, long)]
        date: String,
        
        /// Strava access token
        #[arg(short, long)]
        token: String,
        
        /// Activity types to include (comma-separated). Use 'cycling' for all cycling types, 'running' for all running types, or specify individual types
        #[arg(short = 'a', long, default_value = "cycling")]
        activity_types: String,
        
        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },
}

#[derive(Serialize, Deserialize, Debug)]
struct TokenResponse {
    token_type: String,
    expires_at: i64,
    expires_in: i64,
    refresh_token: String,
    access_token: String,
    athlete: AthleteInfo,
}

#[derive(Serialize, Deserialize, Debug)]
struct AthleteInfo {
    id: i64,
    username: Option<String>,
    firstname: Option<String>,
    lastname: Option<String>,
    city: Option<String>,
    state: Option<String>,
    country: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Activity {
    id: i64,
    name: String,
    distance: f64,
    moving_time: i32,
    elapsed_time: i32,
    total_elevation_gain: f64,
    #[serde(rename = "type")]
    activity_type: String,
    start_date: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Auth {
            client_id,
            client_secret,
            verbose,
        } => handle_auth(client_id, client_secret, verbose).await,
        Commands::Fetch {
            date,
            token,
            activity_types,
            verbose,
        } => handle_fetch(date, token, activity_types, verbose).await,
    }
}

async fn handle_auth(client_id: String, client_secret: String, verbose: bool) -> Result<()> {
    if verbose {
        println!("{}", "🔐 Starting Strava OAuth authentication...".bright_cyan().bold());
    }

    // Generate a unique state parameter for security
    let state = Uuid::new_v4().to_string();

    // Build the authorization URL
    let auth_url = build_auth_url(&client_id, &state)?;

    println!("{}", "🔗 Please open this URL in your browser to authorize the application:".bright_cyan().bold());
    println!("{}", auth_url.blue().underline());
    println!();
    println!("{}", "After authorizing, you'll be redirected to a page that can't be reached.".yellow());
    println!("{}", "Copy the ENTIRE URL from your browser's address bar and paste it here:".yellow());

    print!("{}", "Enter the redirect URL: ".green().bold());
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let redirect_url = input.trim();

    if verbose {
        println!("{} {}", "Processing redirect URL:".dimmed(), redirect_url.dimmed());
    }

    // Extract the authorization code from the redirect URL
    let auth_code = extract_auth_code(redirect_url, &state)?;

    if verbose {
        println!("{} {}", "Extracted authorization code:".dimmed(), auth_code.dimmed());
    }

    // Exchange the authorization code for tokens
    let token_response = exchange_code_for_token(&client_id, &client_secret, &auth_code).await?;

    println!("{}", "✅ Authentication successful!".bright_green().bold());
    println!(
        "{} {} {}",
        "🏃 Athlete:".bright_cyan().bold(),
        token_response.athlete.firstname.unwrap_or_default().bright_white().bold(),
        token_response.athlete.lastname.unwrap_or_default().bright_white().bold()
    );
    println!("{} {}", "🔑 Access Token:".bright_yellow().bold(), token_response.access_token.bright_white());
    println!("{} {}", "🔄 Refresh Token:".bright_blue().bold(), token_response.refresh_token.bright_white());
    println!("{} {}", "⏰ Token expires at:".bright_magenta().bold(), token_response.expires_at.to_string().bright_white());
    println!();
    println!("{}", "💡 Save your access token to use with the 'fetch' command:".bright_cyan().bold());
    println!(
        "   {} {}",
        "chain-life fetch --date 2024-01-01 --token".dimmed(),
        token_response.access_token.bright_green()
    );
    println!();

    Ok(())
}

async fn handle_fetch(date: String, token: String, activity_types: String, verbose: bool) -> Result<()> {
    if verbose {
        println!("{}", "🚀 Starting Strava data fetch...".bright_cyan().bold());
    }
    
    // Parse the input date
    let start_date = parse_date(&date).context("Failed to parse the provided date")?;
    
    if verbose {
        println!("{} {}", "📅 Parsed start date:".cyan(), start_date.to_string().bright_white().bold());
    }
    
    // Parse activity types
    let allowed_types = parse_activity_types(&activity_types)?;
    
    if verbose {
        println!("{} {}", "🔍 Filtering for activity types:".cyan(), 
                format!("{:?}", allowed_types).bright_yellow());
    }
    
    // Fetch activities from Strava
    let total_km = fetch_strava_data_since(start_date, token, allowed_types, verbose).await?;
    
    println!("{} {}: {} km", 
             "🚴 Total kilometers since".bright_green().bold(),
             date.bright_white().bold(),
             format!("{:.2}", total_km).bright_green().bold());
    
    Ok(())
}

fn build_auth_url(client_id: &str, state: &str) -> Result<String> {
    let mut url = Url::parse("https://www.strava.com/oauth/authorize")?;

    url.query_pairs_mut()
        .append_pair("client_id", client_id)
        .append_pair("response_type", "code")
        .append_pair("redirect_uri", "http://localhost/exchange_token")
        .append_pair("approval_prompt", "force")
        .append_pair("scope", "read,activity:read_all")
        .append_pair("state", state);

    Ok(url.to_string())
}

fn extract_auth_code(redirect_url: &str, expected_state: &str) -> Result<String> {
    let url = Url::parse(redirect_url).context("Invalid redirect URL format")?;

    let query_pairs: std::collections::HashMap<String, String> =
        url.query_pairs().into_owned().collect();

    // Verify state parameter for security
    if let Some(state) = query_pairs.get("state") {
        if state != expected_state {
            return Err(anyhow::anyhow!(
                "State parameter mismatch. Possible CSRF attack."
            ));
        }
    }

    // Check for authorization errors
    if let Some(error) = query_pairs.get("error") {
        return Err(anyhow::anyhow!("Authorization error: {}", error));
    }

    // Extract the authorization code
    query_pairs
        .get("code")
        .ok_or_else(|| anyhow::anyhow!("No authorization code found in redirect URL"))
        .map(|code| code.to_string())
}

async fn exchange_code_for_token(
    client_id: &str,
    client_secret: &str,
    auth_code: &str,
) -> Result<TokenResponse> {
    let client = reqwest::Client::new();

    let params = [
        ("client_id", client_id),
        ("client_secret", client_secret),
        ("code", auth_code),
        ("grant_type", "authorization_code"),
    ];

    let response = client
        .post("https://www.strava.com/oauth/token")
        .form(&params)
        .send()
        .await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(anyhow::anyhow!("Token exchange failed: {}", error_text));
    }

    let token_response: TokenResponse = response.json().await?;
    Ok(token_response)
}

/// Parse a date string in YYYY-MM-DD format
fn parse_date(date_str: &str) -> Result<NaiveDate> {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d").context("Date must be in YYYY-MM-DD format")
}

/// Fetch Strava data since the given date
async fn fetch_strava_data_since(
    start_date: NaiveDate,
    token: String,
    allowed_types: Vec<String>,
    verbose: bool,
) -> Result<f64> {
    let client = reqwest::Client::new();

    // Convert start_date to Unix timestamp
    let start_timestamp = start_date
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc()
        .timestamp();

    if verbose {
        println!("{} {}", "📡 Fetching activities since timestamp:".cyan(), 
                start_timestamp.to_string().bright_white());
    }

    let mut page = 1;
    let per_page = 200; // Max allowed by Strava
    let mut total_distance = 0.0;
    let mut total_activities = 0;
    let mut filtered_activities = 0;
    
    loop {
        let response = client
            .get("https://www.strava.com/api/v3/athlete/activities")
            .header("Authorization", format!("Bearer {token}"))
            .query(&[
                ("after", start_timestamp.to_string()),
                ("page", page.to_string()),
                ("per_page", per_page.to_string()),
            ])
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("Strava API error: {error_text}"));
        }

        let activities: Vec<Activity> = response.json().await?;

        if activities.is_empty() {
            break;
        }

        if verbose {
            println!("{} {} activities from page {}", 
                     "📄 Fetched".cyan(),
                     activities.len().to_string().bright_white().bold(),
                     page.to_string().bright_white().bold());
        }

        for activity in &activities {
            if allowed_types.contains(&activity.activity_type) {
                total_distance += activity.distance;
                total_activities += 1;

                if verbose {
                    println!(
                        "  {} {}: {} km ({})",
                        "✓".bright_green().bold(),
                        activity.name.bright_white(),
                        format!("{:.2}", activity.distance / 1000.0).bright_green().bold(),
                        activity.activity_type.bright_blue()
                    );
                }
            } else {
                filtered_activities += 1;
                if verbose {
                    println!(
                        "  {} {}: {} km ({}) - {}",
                        "✗".bright_red().bold(),
                        activity.name.dimmed(),
                        format!("{:.2}", activity.distance / 1000.0).dimmed(),
                        activity.activity_type.red(),
                        "filtered out".red().italic()
                    );
                }
            }
        }

        // If we got fewer activities than requested, we've reached the end
        if activities.len() < per_page {
            break;
        }

        page += 1;
    }

    if verbose {
        println!();
        println!("{} {}", "📊 Total activities included:".bright_green().bold(), 
                total_activities.to_string().bright_green().bold());
        println!("{} {}", "🚫 Total activities filtered out:".bright_red().bold(), 
                filtered_activities.to_string().bright_red().bold());
        println!();
    }

    // Convert from meters to kilometers
    Ok(total_distance / 1000.0)
}

/// Parse activity types from user input, supporting shortcuts like 'cycling' and 'running'
fn parse_activity_types(input: &str) -> Result<Vec<String>> {
    let mut types = Vec::new();
    
    for part in input.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        match part.to_lowercase().as_str() {
            "cycling" => {
                types.extend(CYCLING_TYPES.iter().map(|s| s.to_string()));
            }
            "running" => {
                types.extend(RUNNING_TYPES.iter().map(|s| s.to_string()));
            }
            "all" => {
                types.extend(CYCLING_TYPES.iter().map(|s| s.to_string()));
                types.extend(RUNNING_TYPES.iter().map(|s| s.to_string()));
                types.extend(OTHER_TYPES.iter().map(|s| s.to_string()));
            }
            _ => {
                types.push(part.to_string());
            }
        }
    }
    
    if types.is_empty() {
        return Err(anyhow::anyhow!("No valid activity types specified"));
    }
    
    Ok(types)
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

    #[test]
    fn test_build_auth_url() {
        let client_id = "12345";
        let state = "test-state";
        let url = build_auth_url(client_id, state).unwrap();

        assert!(url.contains("client_id=12345"));
        assert!(url.contains("response_type=code"));
        assert!(url.contains("redirect_uri=http%3A%2F%2Flocalhost%2Fexchange_token"));
        assert!(url.contains("state=test-state"));
        assert!(url.contains("scope=read%2Cactivity%3Aread_all"));
    }

    #[test]
    fn test_extract_auth_code_success() {
        let redirect_url = "http://localhost/exchange_token?state=test-state&code=abc123&scope=read,activity:read_all";
        let state = "test-state";
        let result = extract_auth_code(redirect_url, state);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "abc123");
    }

    #[test]
    fn test_extract_auth_code_missing_code() {
        let redirect_url =
            "http://localhost/exchange_token?state=test-state&scope=read,activity:read_all";
        let state = "test-state";
        let result = extract_auth_code(redirect_url, state);
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_auth_code_state_mismatch() {
        let redirect_url = "http://localhost/exchange_token?state=wrong-state&code=abc123";
        let state = "test-state";
        let result = extract_auth_code(redirect_url, state);
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_auth_code_with_error() {
        let redirect_url = "http://localhost/exchange_token?error=access_denied&state=test-state";
        let state = "test-state";
        let result = extract_auth_code(redirect_url, state);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_parse_activity_types_cycling() {
        let result = parse_activity_types("cycling");
        assert!(result.is_ok());
        let types = result.unwrap();
        assert!(types.contains(&"Ride".to_string()));
        assert!(types.contains(&"VirtualRide".to_string()));
        assert!(types.contains(&"EBikeRide".to_string()));
    }
    
    #[test]
    fn test_parse_activity_types_running() {
        let result = parse_activity_types("running");
        assert!(result.is_ok());
        let types = result.unwrap();
        assert!(types.contains(&"Run".to_string()));
        assert!(types.contains(&"TrailRun".to_string()));
    }
    
    #[test]
    fn test_parse_activity_types_mixed() {
        let result = parse_activity_types("cycling,Run,Walk");
        assert!(result.is_ok());
        let types = result.unwrap();
        assert!(types.contains(&"Ride".to_string()));
        assert!(types.contains(&"Run".to_string()));
        assert!(types.contains(&"Walk".to_string()));
    }
    
    #[test]
    fn test_parse_activity_types_empty() {
        let result = parse_activity_types("");
        assert!(result.is_err());
    }
}
