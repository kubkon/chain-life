#!/bin/bash

# Demo script for Strava CLI Tool
# This script demonstrates how to use the OAuth authentication and data fetching features

set -e

echo "🚴 Strava CLI Tool Demo"
echo "======================"
echo

# Build the project
echo "📦 Building the project..."
cargo build --release
echo "✅ Build complete!"
echo

# Show help
echo "📋 Available commands:"
./target/release/chain-life --help
echo

# Show auth command help
echo "🔐 Authentication command help:"
./target/release/chain-life auth --help
echo

# Show fetch command help
echo "📊 Fetch command help:"
./target/release/chain-life fetch --help
echo

echo "🎯 Demo Instructions:"
echo "===================="
echo
echo "1. First, create a Strava API application:"
echo "   - Go to https://www.strava.com/settings/api"
echo "   - Create a new application"
echo "   - Set 'Authorization Callback Domain' to 'localhost'"
echo "   - Note your Client ID and Client Secret"
echo

echo "2. Authenticate with Strava:"
echo "   ./target/release/chain-life auth --client-id YOUR_CLIENT_ID --client-secret YOUR_CLIENT_SECRET"
echo

echo "3. After authentication, use the access token to fetch cycling data:"
echo "   ./target/release/chain-life fetch --date 2024-01-01 --token YOUR_ACCESS_TOKEN"
echo
echo "   By default, it filters for cycling activities only (Ride, VirtualRide, EBikeRide, etc.)"
echo

echo "4. For verbose output with activity-by-activity breakdown:"
echo "   ./target/release/chain-life fetch --date 2024-01-01 --token YOUR_ACCESS_TOKEN --verbose"
echo
echo "5. To include different activity types:"
echo "   ./target/release/chain-life fetch --date 2024-01-01 --token YOUR_ACCESS_TOKEN --activity-types running"
echo "   ./target/release/chain-life fetch --date 2024-01-01 --token YOUR_ACCESS_TOKEN --activity-types \"Ride,Run,Walk\""
echo "   ./target/release/chain-life fetch --date 2024-01-01 --token YOUR_ACCESS_TOKEN --activity-types all"
echo

echo "🔧 Example with fake credentials (will show URL generation):"
echo "============================================================="
echo
echo "Command: ./target/release/chain-life auth --client-id 12345 --client-secret fake_secret"
echo "This will generate an authorization URL that you can copy and paste into your browser."
echo

echo "🚀 Ready to use! Run the commands above with your real Strava credentials."
echo "📚 For more information, see the README.md file."
echo
echo "💡 Pro tip: The tool defaults to cycling activities, perfect for tracking your bike rides!"
echo "   Use --activity-types to customize what activities to include in your distance calculations."