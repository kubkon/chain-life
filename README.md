# Strava CLI Tool

A command-line tool to authenticate with Strava using OAuth and fetch your total kilometers since a given date.

## Features

- 🔐 OAuth 2.0 authentication with Strava
- 📅 Parse human-readable dates in YYYY-MM-DD format
- 🚴 Smart activity filtering (cycling by default, with options for running, all, or custom types)
- 🏃 Fetch total kilometers from filtered activities since a specified date
- 🎨 Beautiful colored terminal output with emojis for better readability
- 🔍 Verbose output option for debugging with activity-by-activity breakdown
- 🛡️ Secure token handling with state validation
- 📊 Real-time activity data from Strava API

## Prerequisites

1. **Rust**: Make sure you have Rust installed
2. **Strava Account**: You need a Strava account
3. **Strava API Application**: Create an application at https://www.strava.com/settings/api

### Setting up your Strava API Application

1. Go to https://www.strava.com/settings/api
2. Create a new application
3. Set the "Authorization Callback Domain" to `localhost`
4. Note your **Client ID** and **Client Secret**

## Installation

```bash
git clone <repository-url>
cd chain-life
cargo build --release
```

## Usage

The CLI has two main commands: `auth` for authentication and `fetch` for retrieving data.

### 1. Authentication

First, authenticate with Strava using OAuth:

```bash
# Authenticate with your Strava application credentials
./target/release/chain-life auth --client-id YOUR_CLIENT_ID --client-secret YOUR_CLIENT_SECRET
```

This will:
1. Generate an authorization URL
2. Ask you to open it in your browser
3. Redirect you to authorize the application
4. Ask you to paste the redirect URL back
5. Exchange the authorization code for access and refresh tokens

Example output (with colors and emojis):
```
🔐 Starting Strava OAuth authentication...
🔗 Please open this URL in your browser to authorize the application:
https://www.strava.com/oauth/authorize?client_id=12345&response_type=code&redirect_uri=http://localhost/exchange_token&approval_prompt=force&scope=read,activity:read_all&state=...

After authorizing, you'll be redirected to a page that can't be reached.
Copy the ENTIRE URL from your browser's address bar and paste it here:
Enter the redirect URL: http://localhost/exchange_token?state=...&code=abc123&scope=read,activity:read_all

✅ Authentication successful!
🏃 Athlete: John Doe
🔑 Access Token: your_access_token_here
🔄 Refresh Token: your_refresh_token_here
⏰ Token expires at: 1234567890

💡 Save your access token to use with the 'fetch' command:
   chain-life fetch --date 2024-01-01 --token your_access_token_here
```

The output uses vibrant colors to make the authentication flow clear and visually appealing.

### 2. Fetch Data

After authentication, use the access token to fetch your kilometers:

```bash
# Fetch cycling kilometers since January 1st, 2024 (default)
./target/release/chain-life fetch --date 2024-01-01 --token YOUR_ACCESS_TOKEN

# Fetch running kilometers only
./target/release/chain-life fetch --date 2024-01-01 --token YOUR_ACCESS_TOKEN --activity-types running

# Fetch specific activity types
./target/release/chain-life fetch --date 2024-01-01 --token YOUR_ACCESS_TOKEN --activity-types "Ride,VirtualRide,Run"

# With beautiful verbose output (includes colors and activity breakdown)
./target/release/chain-life fetch --date 2024-01-01 --token YOUR_ACCESS_TOKEN --verbose
```

Example output:
```
🚴 Total kilometers since 2024-01-01: 342.50 km
```

With verbose output (with beautiful colors and emojis):
```
🚀 Starting Strava data fetch...
📅 Parsed start date: 2024-01-01
🔍 Filtering for activity types: ["Ride", "VirtualRide", "EBikeRide", "MountainBikeRide", "GravelRide", "Handcycle"]
📡 Fetching activities since timestamp: 1704067200
📄 Fetched 25 activities from page 1
  ✓ Morning Bike Ride: 15.30 km (Ride)
  ✓ Evening Zwift Session: 25.40 km (VirtualRide)
  ✗ Weekend Long Run: 21.10 km (Run) - filtered out
  ✓ Gravel Adventure: 35.20 km (GravelRide)
  ...

📊 Total activities included: 18
🚫 Total activities filtered out: 7

🚴 Total kilometers since 2024-01-01: 342.50 km
```

**Color Scheme:**
- 🟢 **Green**: Included activities, success messages, totals
- 🔴 **Red**: Filtered out activities, errors
- 🔵 **Blue**: URLs, activity types
- 🟡 **Yellow**: Warnings, filter information
- 🟣 **Cyan**: Section headers, processing steps
- ⚪ **White/Bold**: Important values (dates, distances, tokens)

## Command Reference

### Global Options

- `--help` / `-h`: Show help message

### `auth` Command

Authenticate with Strava using OAuth 2.0.

```bash
chain-life auth [OPTIONS]
```

**Options:**
- `--client-id` / `-c`: Your Strava application's Client ID (required)
- `--client-secret` / `-s`: Your Strava application's Client Secret (required)
- `--verbose` / `-v`: Enable verbose output

### `fetch` Command

Fetch kilometers data from Strava since a given date.

```bash
chain-life fetch [OPTIONS]
```

**Options:**
- `--date` / `-d`: Start date in YYYY-MM-DD format (required)
- `--token` / `-t`: Strava access token (required)
- `--activity-types` / `-a`: Activity types to include (default: "cycling")
- `--verbose` / `-v`: Enable verbose output

**Activity Type Options:**
- `cycling`: All cycling activities (Ride, VirtualRide, EBikeRide, MountainBikeRide, GravelRide, Handcycle)
- `running`: All running activities (Run, TrailRun, Treadmill, VirtualRun)
- `all`: All activity types
- Custom: Comma-separated list of specific types (e.g., "Ride,Run,Walk")

## Security Notes

- **Never share your Client Secret**: Keep it confidential
- **Access tokens expire**: Tokens expire every 6 hours
- **Refresh tokens**: Use refresh tokens to get new access tokens
- **State validation**: The CLI validates state parameters to prevent CSRF attacks

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run only unit tests
cargo test --lib

# Run only integration tests
cargo test --test cli_tests
```

### Building for Release

```bash
cargo build --release
```

## Troubleshooting

### Common Issues

1. **"This site can't be reached"** - This is expected! Just copy the URL from your browser.
2. **Invalid token** - Access tokens expire every 6 hours. Re-authenticate to get a new one.
3. **Rate limits** - Strava API has rate limits (200 requests per 15 minutes, 2000 per day).
4. **Colors not showing** - If colors don't appear, your terminal may not support them. Try a modern terminal like iTerm2, Windows Terminal, or VS Code's integrated terminal.

### Getting Help

- Check the [Strava API documentation](https://developers.strava.com/docs/)
- Ensure your application is properly configured in Strava settings
- Use `--verbose` flag for detailed output

## API Permissions

This tool requests the following Strava permissions:
- `read`: Access to read public profile information
- `activity:read_all`: Access to read all activities (including private ones)

## Activity Types

The tool supports filtering by activity type to focus on specific types of exercise:

**Cycling Activities (default):**
- Ride, VirtualRide, EBikeRide, MountainBikeRide, GravelRide, Handcycle

**Running Activities:**
- Run, TrailRun, Treadmill, VirtualRun

**Other Supported Types:**
- Walk, Hike, Swim, Rowing, Kayaking, Canoeing, StandUpPaddling, Surfing, Kitesurf, Windsurf, Sail, Snowboard, Ski, BackcountrySki, NordicSki, Snowshoe, RockClimbing, IceClimbing, AlpineSki, Elliptical, StairStepper, WeightTraining, Workout, Crossfit, Yoga, Golf

Use the `--activity-types` parameter to specify which activities to include in your distance calculations.

## License

This project is open source and available under the MIT License.