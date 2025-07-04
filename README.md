# Strava CLI Tool

A simple command-line tool to fetch the total kilometers from your Strava account since a given date.

## Features

- Parse human-readable dates in YYYY-MM-DD format
- Fetch total kilometers from Strava activities since the specified date
- Verbose output option for debugging
- Comprehensive error handling

## Installation

Make sure you have Rust installed, then build the project:

```bash
cargo build --release
```

## Usage

### Basic Usage

```bash
# Fetch kilometers since January 1st, 2024
./target/release/chain-life --date 2024-01-01

# With a Strava access token (required for actual API calls)
./target/release/chain-life --date 2024-01-01 --token YOUR_STRAVA_TOKEN
```

### Options

- `--date` / `-d`: Start date in YYYY-MM-DD format (required)
- `--token` / `-t`: Strava access token (optional, but required for real API calls)
- `--verbose` / `-v`: Enable verbose output
- `--help` / `-h`: Show help message

### Examples

```bash
# Basic usage with verbose output
./target/release/chain-life --date 2024-01-01 --verbose

# With token for actual Strava API calls
./target/release/chain-life --date 2024-01-01 --token abc123xyz --verbose
```

## Development

### Running Tests

```bash
# Run unit tests
cargo test

# Run integration tests
cargo test --test cli_tests
```

### Current Status

This is a basic CLI framework. The Strava API integration is currently mocked and returns dummy data. Future versions will implement actual Strava API calls.

## Getting a Strava Access Token

To use this tool with real Strava data, you'll need to:

1. Create a Strava API application at https://www.strava.com/settings/api
2. Follow the OAuth flow to get an access token
3. Pass the token using the `--token` argument

## License

This project is open source and available under the MIT License.