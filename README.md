# Strava CLI Tool

A command-line tool to authenticate with Strava using OAuth and fetch your total kilometers since a given date.

## Features

- 🔐 OAuth 2.0 authentication with Strava
- 📅 Parse human-readable dates in YYYY-MM-DD format
- 🏃 Fetch total kilometers from all activities since a specified date
- 🔍 Verbose output option for debugging
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

Example output:
```
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

### 2. Fetch Data

After authentication, use the access token to fetch your kilometers:

```bash
# Fetch kilometers since January 1st, 2024
./target/release/chain-life fetch --date 2024-01-01 --token YOUR_ACCESS_TOKEN

# With verbose output
./target/release/chain-life fetch --date 2024-01-01 --token YOUR_ACCESS_TOKEN --verbose
```

Example output:
```
🏃 Total kilometers since 2024-01-01: 342.50 km
```

With verbose output:
```
Starting Strava data fetch...
Parsed start date: 2024-01-01
Fetching activities since timestamp: 1704067200
Fetched 25 activities from page 1
  - Morning Run: 5.20 km (Run)
  - Evening Bike Ride: 15.30 km (Ride)
  - Weekend Long Run: 21.10 km (Run)
  ...
Total activities processed: 25
🏃 Total kilometers since 2024-01-01: 342.50 km
```

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
- `--verbose` / `-v`: Enable verbose output

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

### Getting Help

- Check the [Strava API documentation](https://developers.strava.com/docs/)
- Ensure your application is properly configured in Strava settings
- Use `--verbose` flag for detailed output

## API Permissions

This tool requests the following Strava permissions:
- `read`: Access to read public profile information
- `activity:read_all`: Access to read all activities (including private ones)

## License

This project is open source and available under the MIT License.