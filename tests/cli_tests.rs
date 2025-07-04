use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn test_cli_help_message() {
    let mut cmd = Command::cargo_bin("chain-life").unwrap();
    cmd.arg("--help");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A CLI tool to fetch kilometers from Strava"))
        .stdout(predicate::str::contains("auth"))
        .stdout(predicate::str::contains("fetch"));
}

#[test]
fn test_auth_subcommand_help() {
    let mut cmd = Command::cargo_bin("chain-life").unwrap();
    cmd.arg("auth").arg("--help");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Authenticate with Strava using OAuth"))
        .stdout(predicate::str::contains("--client-id"))
        .stdout(predicate::str::contains("--client-secret"));
}

#[test]
fn test_fetch_subcommand_help() {
    let mut cmd = Command::cargo_bin("chain-life").unwrap();
    cmd.arg("fetch").arg("--help");
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Fetch kilometers data from Strava"))
        .stdout(predicate::str::contains("--date"))
        .stdout(predicate::str::contains("--token"));
}

#[test]
fn test_auth_missing_client_id() {
    let mut cmd = Command::cargo_bin("chain-life").unwrap();
    cmd.arg("auth").arg("--client-secret").arg("secret123");
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("client-id"));
}

#[test]
fn test_auth_missing_client_secret() {
    let mut cmd = Command::cargo_bin("chain-life").unwrap();
    cmd.arg("auth").arg("--client-id").arg("12345");
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("client-secret"));
}

#[test]
fn test_fetch_missing_date() {
    let mut cmd = Command::cargo_bin("chain-life").unwrap();
    cmd.arg("fetch").arg("--token").arg("token123");
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("date"));
}

#[test]
fn test_fetch_missing_token() {
    let mut cmd = Command::cargo_bin("chain-life").unwrap();
    cmd.arg("fetch").arg("--date").arg("2024-01-01");
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("token"));
}

#[test]
fn test_fetch_invalid_date_format() {
    let mut cmd = Command::cargo_bin("chain-life").unwrap();
    cmd.arg("fetch")
        .arg("--date").arg("01-01-2024")
        .arg("--token").arg("fake_token");
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Date must be in YYYY-MM-DD format"));
}

#[test]
fn test_fetch_invalid_date() {
    let mut cmd = Command::cargo_bin("chain-life").unwrap();
    cmd.arg("fetch")
        .arg("--date").arg("2024-13-45")
        .arg("--token").arg("fake_token");
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Failed to parse the provided date"));
}

#[test]
fn test_no_subcommand() {
    let mut cmd = Command::cargo_bin("chain-life").unwrap();
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage: chain-life <COMMAND>"));
}

#[test]
fn test_invalid_subcommand() {
    let mut cmd = Command::cargo_bin("chain-life").unwrap();
    cmd.arg("invalid");
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("unrecognized subcommand"));
}