#!/bin/bash

# Color Demo Script for Strava CLI Tool
# This script demonstrates the beautiful colored output without needing real Strava credentials

set -e

echo "🎨 Strava CLI Color Demo"
echo "========================"
echo

echo "🚀 Building the project first..."
cargo build --release
echo

echo "🎯 Demo 1: Help commands with colored output"
echo "============================================"
echo
./target/release/chain-life --help
echo

echo "🔐 Demo 2: Auth command help (shows colored options)"
echo "===================================================="
echo
./target/release/chain-life auth --help
echo

echo "📊 Demo 3: Fetch command help (shows activity filtering options)"
echo "================================================================"
echo
./target/release/chain-life fetch --help
echo

echo "🚴 Demo 4: Simulated fetch output (will show colorful error with fake token)"
echo "============================================================================"
echo "Command: ./target/release/chain-life fetch --date 2024-01-01 --token fake_token --verbose"
echo
echo "Expected output (with beautiful colors and emojis):"
echo "🚀 Starting Strava data fetch..."
echo "📅 Parsed start date: 2024-01-01"
echo "🔍 Filtering for activity types: [cycling activities]"
echo "📡 Fetching activities since timestamp: 1704067200"
echo "❌ Error: Strava API error (expected with fake token)"
echo

echo "🏃 Demo 5: Running activities filter example"
echo "============================================"
echo "Command: ./target/release/chain-life fetch --date 2024-01-01 --token fake --activity-types running --verbose"
echo
echo "This would show filtering for: [\"Run\", \"TrailRun\", \"Treadmill\", \"VirtualRun\"]"
echo

echo "🎨 Color Features Demonstrated:"
echo "==============================="
echo "✅ Green checkmarks for included activities"
echo "❌ Red X marks for filtered out activities" 
echo "🔵 Blue highlighting for URLs and important links"
echo "🟡 Yellow text for warnings and instructions"
echo "🟣 Cyan/Purple for section headers and progress"
echo "⚪ Bold white for important values (dates, distances)"
echo "📱 Emojis for visual categorization and fun"
echo

echo "💡 To see the real colorful output, run:"
echo "   ./target/release/chain-life auth --client-id YOUR_ID --client-secret YOUR_SECRET"
echo "   ./target/release/chain-life fetch --date 2024-01-01 --token YOUR_TOKEN --verbose"
echo

echo "🌈 The tool uses the 'colored' crate for beautiful terminal output that works across platforms!"