use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn pwd_returns_non_zero_and_stderr_when_character_sets_are_all_disabled() {
    let mut command = Command::cargo_bin("rand_tool").expect("binary should build");

    command
        .args(["pwd", "-n", "-u", "-l"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("unable to generate passwords"))
        .stderr(predicate::str::contains(
            "You need to enable at least one kind of characters.",
        ));
}

#[test]
fn base64_decode_returns_non_zero_and_stderr_for_invalid_input() {
    let mut command = Command::cargo_bin("rand_tool").expect("binary should build");

    command
        .args(["base64", "-d", "%%%"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("failed to decode Base64 input"));
}

#[test]
fn base64_rejects_conflicting_encode_and_decode_arguments() {
    let mut command = Command::cargo_bin("rand_tool").expect("binary should build");

    command
        .args(["base64", "-d", "SGVsbG8=", "-e", "test"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("cannot be used with"))
        .stderr(predicate::str::contains("--decode <DECODE>"))
        .stderr(predicate::str::contains("--encode <ENCODE>"));
}

#[test]
fn base64_requires_exactly_one_operation_argument() {
    let mut command = Command::cargo_bin("rand_tool").expect("binary should build");

    command
        .args(["base64"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"))
        .stderr(predicate::str::contains("--decode <DECODE>"))
        .stderr(predicate::str::contains("--encode <ENCODE>"));
}

#[test]
fn port_unique_returns_non_zero_when_count_exceeds_range_capacity() {
    let mut command = Command::cargo_bin("rand_tool").expect("binary should build");

    command
        .args(["-c", "3", "port", "-r", "7000-7001", "--unique"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("requested 3 unique ports"))
        .stderr(predicate::str::contains("range 7000-7001 only contains 2"));
}
