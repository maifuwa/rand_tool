use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_no_subcommand() {
    let mut cmd = Command::cargo_bin("rand_tool").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("rand_tool").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("rand_tool"))
        .stdout(predicate::str::contains("Usage:"));
}

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("rand_tool").unwrap();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("rand_tool"));
}

#[test]
fn test_pwd_default() {
    let mut cmd = Command::cargo_bin("rand_tool").unwrap();
    cmd.arg("pwd");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("password:"))
        .stdout(predicate::str::contains("score:"));
}

#[test]
fn test_pwd_custom_length() {
    let mut cmd = Command::cargo_bin("rand_tool").unwrap();
    cmd.args(["pwd", "--length", "12"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("password:"));
}

#[test]
fn test_pwd_with_symbols() {
    let mut cmd = Command::cargo_bin("rand_tool").unwrap();
    cmd.args(["pwd", "--symbols"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("password:"));
}

#[test]
fn test_pwd_custom_count() {
    let mut cmd = Command::cargo_bin("rand_tool").unwrap();
    cmd.args(["--count", "3", "pwd"]);
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let password_count = stdout.matches("password:").count();
    assert_eq!(password_count, 3);
}

#[test]
fn test_pwd_without_numbers() {
    let mut cmd = Command::cargo_bin("rand_tool").unwrap();
    cmd.args(["pwd", "--numbers"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("password:"));
}

#[test]
fn test_pwd_without_uppercase() {
    let mut cmd = Command::cargo_bin("rand_tool").unwrap();
    cmd.args(["pwd", "--uppercase"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("password:"));
}

#[test]
fn test_pwd_without_lowercase() {
    let mut cmd = Command::cargo_bin("rand_tool").unwrap();
    cmd.args(["pwd", "--lowercase"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("password:"));
}

#[test]
fn test_port_default() {
    let mut cmd = Command::cargo_bin("rand_tool").unwrap();
    cmd.arg("port");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("generated port range:"));
}

#[test]
fn test_port_custom_range() {
    let mut cmd = Command::cargo_bin("rand_tool").unwrap();
    cmd.args(["port", "--range", "8000-9000"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("generated port range: 8000-9000"));
}

#[test]
fn test_port_custom_count() {
    let mut cmd = Command::cargo_bin("rand_tool").unwrap();
    cmd.args(["--count", "3", "port"]);
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines.len(), 4);
}

#[test]
fn test_uuid_default() {
    let mut cmd = Command::cargo_bin("rand_tool").unwrap();
    cmd.arg("uuid");
    cmd.assert().success();
}

#[test]
fn test_uuid_format() {
    let mut cmd = Command::cargo_bin("rand_tool").unwrap();
    cmd.arg("uuid");
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    for line in stdout.lines() {
        assert_eq!(line.len(), 36);
        assert_eq!(line.matches('-').count(), 4);
    }
}

#[test]
fn test_uuid_custom_count() {
    let mut cmd = Command::cargo_bin("rand_tool").unwrap();
    cmd.args(["--count", "10", "uuid"]);
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.lines().count(), 10);
}

#[test]
fn test_base64_encode() {
    let mut cmd = Command::cargo_bin("rand_tool").unwrap();
    cmd.args(["base64", "--encode", "Hello, World!"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("SGVsbG8sIFdvcmxkIQ=="));
}

#[test]
fn test_base64_decode() {
    let mut cmd = Command::cargo_bin("rand_tool").unwrap();
    cmd.args(["base64", "--decode", "SGVsbG8sIFdvcmxkIQ=="]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Hello, World!"));
}

#[test]
fn test_base64_decode_invalid() {
    let mut cmd = Command::cargo_bin("rand_tool").unwrap();
    cmd.args(["base64", "--decode", "Invalid@Base64!"]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Invalid"));
}

#[test]
fn test_base64_no_input() {
    let mut cmd = Command::cargo_bin("rand_tool").unwrap();
    cmd.arg("base64");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Please provide some input."));
}

#[test]
fn test_base64_encode_empty() {
    let mut cmd = Command::cargo_bin("rand_tool").unwrap();
    cmd.args(["base64", "--encode", ""]);
    cmd.assert().success();
}

#[test]
fn test_base64_decode_empty() {
    let mut cmd = Command::cargo_bin("rand_tool").unwrap();
    cmd.args(["base64", "--decode", ""]);
    cmd.assert().success();
}

#[test]
fn test_global_count_flag() {
    let mut cmd = Command::cargo_bin("rand_tool").unwrap();
    cmd.args(["--count", "2", "uuid"]);
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.lines().count(), 2);
}
