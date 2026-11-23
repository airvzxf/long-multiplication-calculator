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
fn cli_store_suggests_mkdir_for_a_missing_directory() {
    let dir = tempfile::tempdir().expect("tempdir");
    let missing = dir.path().join("missing");
    let file = missing.join("out.txt");

    cli()
        .args(["5", "7", "--output", "store", "--file", file.to_str().unwrap()])
        .assert()
        .failure()
        .stderr(predicate::str::contains(format!("mkdir -p {}", missing.display())));
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

#[test]
fn cli_rejects_file_without_store() {
    cli()
        .args(["5", "7", "--file", "out.txt"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("--file only applies"));
}

#[test]
fn cli_both_prints_and_writes_default_named_file() {
    let dir = tempfile::tempdir().expect("tempdir");
    cli()
        .current_dir(dir.path())
        .args(["5", "7", "--output", "both"])
        .assert()
        .success()
        .stdout(predicate::str::contains("3 │ 5 ┃ P"));
    let contents = std::fs::read_to_string(dir.path().join("long-multiplication-output.txt"))
        .expect("default file");
    assert!(contents.contains("3 │ 5 ┃ P"));
}

#[test]
fn cli_max_digits_flag_and_env_are_honoured() {
    cli()
        .args(["--max-digits", "3", "1234", "5"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("maximum length of 3 digits"));
    cli()
        .env("LONG_MULTIPLICATION_MAX_DIGITS", "3")
        .args(["1234", "5"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("maximum length of 3 digits"));
    // 0 disables the limit.
    let huge = "1".repeat(1500);
    cli().args(["--max-digits", "0", huge.as_str(), "5"]).assert().success();
}

#[test]
fn cli_output_size_limit_stops_huge_tables() {
    let nines = "9".repeat(200);
    cli()
        .args([nines.as_str(), nines.as_str()])
        .env("LONG_MULTIPLICATION_MAX_OUTPUT_MIB", "16")
        .assert()
        .failure()
        .stderr(predicate::str::contains("The table would take about"))
        .stderr(predicate::str::contains("--unlimited"));
}

#[test]
fn cli_unlimited_removes_both_limits() {
    let big = "1".repeat(1100);
    cli()
        .args(["--unlimited", "--max-digits", "5", "--max-output-mib", "1", big.as_str(), "2"])
        .assert()
        .success();
    cli()
        .env("LONG_MULTIPLICATION_UNLIMITED", "true")
        .args(["--max-digits", "5", big.as_str(), "2"])
        .assert()
        .success();
}

#[test]
fn cli_long_invalid_input_is_shortened_in_errors() {
    let bad = format!("{}x", "1".repeat(100));
    cli()
        .args([bad.as_str(), "5"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("(101 characters)"));
}

#[cfg(unix)]
#[test]
fn cli_closed_pipe_exits_quietly() {
    use std::io::Read;
    use std::process::{Command as StdCommand, Stdio};

    let nines = "9".repeat(60);
    let mut child = StdCommand::new(assert_cmd::cargo::cargo_bin("long-multiplication"))
        .args([nines.as_str(), nines.as_str()])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn");
    // Read one byte, then close the pipe like `| head -c 1` does.
    let mut stdout = child.stdout.take().expect("stdout");
    stdout.read_exact(&mut [0u8; 1]).expect("read");
    drop(stdout);
    let output = child.wait_with_output().expect("wait");
    assert!(output.status.success(), "status {:?}", output.status);
    assert!(output.stderr.is_empty(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
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
fn cli_output_matches_golden_fixture_nines_12_x_12() {
    let expected = include_str!("../../../tests/fixtures/999999999999_x_999999999999.txt");
    cli().args(["999999999999", "999999999999"]).assert().success().stdout(expected.to_string());
}

#[test]
fn cli_output_matches_golden_fixture_123_x_456() {
    let expected = include_str!("../../../tests/fixtures/123_x_456.txt");
    cli().args(["123", "456"]).assert().success().stdout(expected.to_string());
}
