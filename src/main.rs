use anyhow::{Context, Result};
use chrono::NaiveDate;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use url::Url;
use uuid::Uuid;

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
            verbose,
        } => handle_fetch(date, token, verbose).await,
    }
}

async fn handle_auth(client_id: String, client_secret: String, verbose: bool) -> Result<()> {
    if verbose {
        println!("Starting Strava OAuth authentication...");
    }

    // Generate a unique state parameter for security
    let state = Uuid::new_v4().to_string();

    // Build the authorization URL
    let auth_url = build_auth_url(&client_id, &state)?;

    println!("🔗 Please open this URL in your browser to authorize the application:");
    println!("{auth_url}");
    println!();
    println!("After authorizing, you'll be redirected to a page that can't be reached.");
    println!("Copy the ENTIRE URL from your browser's address bar and paste it here:");

    print!("Enter the redirect URL: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let redirect_url = input.trim();

    if verbose {
        println!("Processing redirect URL: {redirect_url}");
    }

    // Extract the authorization code from the redirect URL
    let auth_code = extract_auth_code(redirect_url, &state)?;

    if verbose {
        println!("Extracted authorization code: {auth_code}");
    }

    // Exchange the authorization code for tokens
    let token_response = exchange_code_for_token(&client_id, &client_secret, &auth_code).await?;

    println!("✅ Authentication successful!");
    println!(
        "🏃 Athlete: {} {}",
        token_response.athlete.firstname.unwrap_or_default(),
        token_response.athlete.lastname.unwrap_or_default()
    );
    println!("🔑 Access Token: {}", token_response.access_token);
    println!("🔄 Refresh Token: {}", token_response.refresh_token);
    println!("⏰ Token expires at: {}", token_response.expires_at);
    println!();
    println!("💡 Save your access token to use with the 'fetch' command:");
    println!(
        "   strava-cli fetch --date 2024-01-01 --token {}",
        token_response.access_token
    );

    Ok(())
}

async fn handle_fetch(date: String, token: String, verbose: bool) -> Result<()> {
    if verbose {
        println!("Starting Strava data fetch...");
    }

    // Parse the input date
    let start_date = parse_date(&date).context("Failed to parse the provided date")?;

    if verbose {
        println!("Parsed start date: {start_date}");
    }

    // Fetch activities from Strava
    let total_km = fetch_strava_data_since(start_date, token, verbose).await?;

    println!("🏃 Total kilometers since {date}: {total_km:.2} km");

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
        println!("Fetching activities since timestamp: {start_timestamp}");
    }

    let mut page = 1;
    let per_page = 200; // Max allowed by Strava
    let mut total_distance = 0.0;
    let mut total_activities = 0;

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
            println!("Fetched {} activities from page {}", activities.len(), page);
        }

        for activity in &activities {
            total_distance += activity.distance;
            total_activities += 1;

            if verbose {
                println!(
                    "  - {}: {:.2} km ({})",
                    activity.name,
                    activity.distance / 1000.0,
                    activity.activity_type
                );
            }
        }

        // If we got fewer activities than requested, we've reached the end
        if activities.len() < per_page {
            break;
        }

        page += 1;
    }

    if verbose {
        println!("Total activities processed: {total_activities}");
    }

    // Convert from meters to kilometers
    Ok(total_distance / 1000.0)
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
}
