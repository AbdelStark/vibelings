//! CLI integration tests.
//!
//! Tests for the vibelings CLI commands to ensure they work correctly.

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// Get a Command for the vibelings binary.
#[allow(deprecated)]
fn vibelings() -> Command {
    Command::cargo_bin("vibelings").unwrap()
}

#[test]
fn test_help_command() {
    vibelings()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("vibelings"))
        .stdout(predicate::str::contains(
            "Rustlings for agentic programming",
        ));
}

#[test]
fn test_version_command() {
    vibelings()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("vibelings"));
}

#[test]
fn test_init_command() {
    let temp_dir = TempDir::new().unwrap();

    vibelings()
        .arg("init")
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Workspace ready"));

    // Verify directories were created
    assert!(temp_dir.path().join("exercises").exists());
    assert!(temp_dir.path().join("exercises/fundamentals").exists());
}

#[test]
fn test_list_command_no_exercises() {
    let temp_dir = TempDir::new().unwrap();

    // Create an empty exercises directory
    fs::create_dir_all(temp_dir.path().join("exercises")).unwrap();

    vibelings()
        .arg("list")
        .current_dir(temp_dir.path())
        .assert()
        .success();
}

#[test]
fn test_doctor_command() {
    let temp_dir = TempDir::new().unwrap();

    vibelings()
        .arg("doctor")
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("System Health Check"));
}

#[test]
fn test_run_nonexistent_exercise() {
    let temp_dir = TempDir::new().unwrap();

    // Create exercises directory
    fs::create_dir_all(temp_dir.path().join("exercises")).unwrap();

    vibelings()
        .arg("run")
        .arg("nonexistent/exercise")
        .current_dir(temp_dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found").or(predicate::str::contains("Not found")));
}

#[test]
fn test_hint_nonexistent_exercise() {
    let temp_dir = TempDir::new().unwrap();

    // Create exercises directory
    fs::create_dir_all(temp_dir.path().join("exercises")).unwrap();

    vibelings()
        .arg("hint")
        .arg("nonexistent/exercise")
        .current_dir(temp_dir.path())
        .assert()
        .failure();
}

#[test]
fn test_reset_exercise_no_progress() {
    let temp_dir = TempDir::new().unwrap();

    // Create exercises directory
    fs::create_dir_all(temp_dir.path().join("exercises")).unwrap();

    // Reset command succeeds even when there's no progress to reset
    vibelings()
        .arg("reset")
        .arg("fundamentals/json_01")
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Reset"));
}

#[test]
fn test_cost_command() {
    let temp_dir = TempDir::new().unwrap();

    vibelings()
        .arg("cost")
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("Cost"));
}

#[test]
fn test_list_with_filter() {
    let temp_dir = TempDir::new().unwrap();

    // Create exercises directory
    fs::create_dir_all(temp_dir.path().join("exercises")).unwrap();

    // Test filter flag
    vibelings()
        .arg("list")
        .arg("--track")
        .arg("fundamentals")
        .current_dir(temp_dir.path())
        .assert()
        .success();
}

#[test]
fn test_verify_command() {
    let temp_dir = TempDir::new().unwrap();

    // Create exercises directory
    fs::create_dir_all(temp_dir.path().join("exercises")).unwrap();

    vibelings()
        .arg("verify")
        .current_dir(temp_dir.path())
        .assert()
        .success();
}

#[test]
fn test_replay_nonexistent_run() {
    let temp_dir = TempDir::new().unwrap();

    vibelings()
        .arg("replay")
        .arg("nonexistent-run-id-12345")
        .current_dir(temp_dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found").or(predicate::str::contains("Not found")));
}

#[test]
fn test_init_idempotent() {
    let temp_dir = TempDir::new().unwrap();

    // First init
    vibelings()
        .arg("init")
        .current_dir(temp_dir.path())
        .assert()
        .success();

    // Second init should also succeed (idempotent)
    vibelings()
        .arg("init")
        .current_dir(temp_dir.path())
        .assert()
        .success();

    // Verify directories still exist
    assert!(temp_dir.path().join("exercises").exists());
}

#[test]
fn test_doctor_full_flag() {
    let temp_dir = TempDir::new().unwrap();

    vibelings()
        .arg("doctor")
        .arg("--full")
        .current_dir(temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("System Health Check"));
}

#[test]
fn test_list_all_flag() {
    let temp_dir = TempDir::new().unwrap();

    // Create exercises directory
    fs::create_dir_all(temp_dir.path().join("exercises")).unwrap();

    // Test --all flag
    vibelings()
        .arg("list")
        .arg("--all")
        .current_dir(temp_dir.path())
        .assert()
        .success();
}

#[test]
fn test_hint_with_level() {
    let temp_dir = TempDir::new().unwrap();

    // Create exercises directory
    fs::create_dir_all(temp_dir.path().join("exercises")).unwrap();

    // Hint with level for nonexistent exercise should fail
    vibelings()
        .arg("hint")
        .arg("--level")
        .arg("2")
        .arg("fundamentals/json_01")
        .current_dir(temp_dir.path())
        .assert()
        .failure();
}

#[test]
fn test_run_with_verbose() {
    let temp_dir = TempDir::new().unwrap();

    // Create exercises directory
    fs::create_dir_all(temp_dir.path().join("exercises")).unwrap();

    // Verbose flag should be accepted
    vibelings()
        .arg("run")
        .arg("--verbose")
        .arg("nonexistent/exercise")
        .current_dir(temp_dir.path())
        .assert()
        .failure(); // Fails because exercise doesn't exist, but flag is accepted
}

#[test]
fn test_cost_specific_exercise() {
    let temp_dir = TempDir::new().unwrap();

    vibelings()
        .arg("cost")
        .arg("--exercise")
        .arg("fundamentals/json_01")
        .current_dir(temp_dir.path())
        .assert()
        .success();
}

#[test]
fn test_invalid_subcommand() {
    vibelings()
        .arg("nonexistent-command")
        .assert()
        .failure()
        .stderr(predicate::str::contains("error"));
}
