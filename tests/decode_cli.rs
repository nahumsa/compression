use assert_cmd::Command;

const HELP_STR: &str = "decode a file

Usage: compression decode <FILENAME>

Arguments:
  <FILENAME>  filename to decode

Options:
  -h, --help  Print help
";

const DECODE_OUTPUT_STR: &str = "decoding tests/samples/sample.txt\n";

#[test]
fn help_decode() {
    Command::cargo_bin("compression")
        .unwrap()
        .args(["decode", "--help"])
        .assert()
        .success()
        .stdout(HELP_STR);
}

#[test]
fn decode_output() {
    Command::cargo_bin("compression")
        .unwrap()
        .args(["decode", "tests/samples/sample.txt"])
        .assert()
        .success()
        .stdout(DECODE_OUTPUT_STR);
}
