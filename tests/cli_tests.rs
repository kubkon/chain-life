use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn test_cli_with_valid_date() {
    let mut cmd = Command::cargo_bin("chain-life").unwrap();
    cmd.arg("--date").arg("2024-01-01");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Total kilometers since 2024-01-01:"))
        .stdout(predicate::str::contains("km"));
}

#[test]
fn test_cli_with_invalid_date_format() {
    let mut cmd = Command::cargo_bin("chain-life").unwrap();
    cmd.arg("--date").arg("01-01-2024");
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Date must be in YYYY-MM-DD format"));
}

#[test]
fn test_cli_with_invalid_date() {
    let mut cmd = Command::cargo_bin("chain-life").unwrap();
    cmd.arg("--date").arg("2024-13-45");
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Failed to parse the provided date"));
}

#[test]
fn test_cli_without_date_argument() {
    let mut cmd = Command::cargo_bin("chain-life").unwrap();
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_cli_with_verbose_flag() {
    let mut cmd = Command::cargo_bin("chain-life").unwrap();
    cmd.arg("--date").arg("2024-01-01").arg("--verbose");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Starting Strava CLI tool..."))
        .stdout(predicate::str::contains("Parsed start date:"))
        .stdout(predicate::str::contains("Fetching Strava data since"));
}

#[test]
fn test_cli_with_token_argument() {
    let mut cmd = Command::cargo_bin("chain-life").unwrap();
    cmd.arg("--date").arg("2024-01-01").arg("--token").arg("fake_token_123");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Total kilometers since 2024-01-01:"));
}

#[test]
fn test_cli_help_message() {
    let mut cmd = Command::cargo_bin("chain-life").unwrap();
    cmd.arg("--help");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A CLI tool to fetch kilometers from Strava"))
        .stdout(predicate::str::contains("--date"))
        .stdout(predicate::str::contains("--token"))
        .stdout(predicate::str::contains("--verbose"));
}