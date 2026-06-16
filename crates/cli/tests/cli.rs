//! End-to-end tests for the `long-multiplication` CLI binary.
//!
//! These tests build the binary with `assert_cmd` and exercise the
//! full argument → core algorithm → stdout pipeline. They guard
//! against accidental changes to the CLI's user-facing behaviour.

use assert_cmd::Command;
use predicates::prelude::*;

fn cli() -> Command {
    Command::cargo_bin("long-multiplication").expect("binary should build")
}

#[test]
fn cli_runs_for_small_input() {
    cli()
        .args(["5", "7"])
        .assert()
        .success()
        .stdout(predicate::str::contains("3 │ 5 ┃ P"))
        .stdout(predicate::str::contains("Symbols"));
}

#[test]
fn cli_runs_for_large_input() {
    cli()
        .args(["13597", "8642"])
        .assert()
        .success()
        .stdout(predicate::str::contains("1 │ 1 │ 7 │ 5 │ 0 │ 5 │ 2 │ 7 │ 4 ┃ P"));
}

#[test]
fn cli_rejects_non_digit_input() {
    cli()
        .args(["abc", "123"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid character 'a'"));
}

#[test]
fn cli_rejects_empty_input() {
    cli().args(["", "5"]).assert().failure();
}

#[test]
fn cli_rejects_overlong_input() {
    let huge = "1".repeat(1500);
    cli()
        .args([huge.as_str(), "5"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("exceeds the maximum length"));
}

#[test]
fn cli_store_writes_file() {
    let dir = tempfile::tempdir().expect("tempdir");
    let file = dir.path().join("out.txt");

    cli()
        .args(["5", "7", "--output", "store", "--file", file.to_str().unwrap()])
        .assert()
        .success();

    let contents = std::fs::read_to_string(&file).expect("read file");
    assert!(contents.contains("3 │ 5 ┃ P"), "stored file missing product: {contents}");
}

#[test]
fn cli_help_prints_usage() {
    cli()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("long-multiplication"))
        .stdout(predicate::str::contains("MULTIPLICAND"));
}

#[test]
fn cli_version_prints_version() {
    cli()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("long-multiplication 2."));
}

/// Byte-for-byte regression test: the table for 13597 × 8642 must
/// match the v1.0.0 reference fixture exactly.
#[test]
fn cli_output_matches_golden_fixture_13597_x_8642() {
    let expected = include_str!("../../../tests/fixtures/13597_x_8642.txt");
    cli().args(["13597", "8642"]).assert().success().stdout(expected.to_string());
}

#[test]
fn cli_output_matches_golden_fixture_5_x_7() {
    let expected = include_str!("../../../tests/fixtures/5_x_7.txt");
    cli().args(["5", "7"]).assert().success().stdout(expected.to_string());
}

#[test]
fn cli_output_matches_golden_fixture_123_x_456() {
    let expected = include_str!("../../../tests/fixtures/123_x_456.txt");
    cli().args(["123", "456"]).assert().success().stdout(expected.to_string());
}
