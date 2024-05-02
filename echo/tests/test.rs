use std::fs;

use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn fail_when_no_arg() {
    let mut cmd = Command::cargo_bin("echo").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
}

#[test]
fn success() {
    let mut cmd = Command::cargo_bin("echo").unwrap();
    cmd.arg("hello")
        .assert()
        .success()
        .stdout(predicate::str::is_match("hello\n").unwrap());
}

#[test]
fn success_when_omit_new_line() {
    let mut cmd = Command::cargo_bin("echo").unwrap();
    cmd.arg("hello")
        .arg("-n")
        .assert()
        .success()
        .stdout(predicate::str::is_match("hello").unwrap());
}

type GenericResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn test_with_examples_output() -> GenericResult {
    let _ = verify(&["Hello there"], "tests/expected/hello1.txt");
    let _ = verify(&["Hello", "there"], "tests/expected/hello2.txt");
    let _ = verify(&["-n", "Hello there"], "tests/expected/hello3.txt");
    let _ = verify(&["-n", "Hello", "there"], "tests/expected/hello4.txt");
    Ok(())
}

fn verify(args: &[&str], path: &str) -> GenericResult {
    let expected = fs::read_to_string(path)?;
    Command::cargo_bin("echo")?
        .args(args)
        .assert()
        .stdout(expected);
    Ok(())
}
