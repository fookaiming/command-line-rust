use assert_cmd::Command;

#[test]
fn hello_test() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.assert().success().stdout("hello world!");
}

#[test]
fn exit_normally_test() {
    let mut cmd = Command::cargo_bin("exit_normally").unwrap();
    cmd.assert().success();
}

#[test]
fn abort_test() {
    let mut cmd = Command::cargo_bin("abort").unwrap();
    cmd.assert().failure();
}
