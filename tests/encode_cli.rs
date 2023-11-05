use assert_cmd::Command;

const HELP_STR: &str = "encode a file

Usage: compression encode <FILENAME>

Arguments:
  <FILENAME>  filename to encode

Options:
  -h, --help  Print help
";

const ENCODE_OUTPUT_STR: &str = "hello world";

#[test]
fn help_encode() {
    Command::cargo_bin("compression")
        .unwrap()
        .args(["encode", "--help"])
        .assert()
        .success()
        .stdout(HELP_STR);
}

#[test]
fn encode_output() {
    Command::cargo_bin("compression")
        .unwrap()
        .args(["encode", "tests/samples/sample.txt"])
        .assert()
        .success()
        .stdout(ENCODE_OUTPUT_STR);
}
