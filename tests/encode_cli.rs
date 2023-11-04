use assert_cmd::Command;

const HELP_STR: &str = "encode a file

Usage: compression encode <FILENAME>

Arguments:
  <FILENAME>  filename to encode

Options:
  -h, --help  Print help
";

#[test]
fn help_encode() {
    Command::cargo_bin("compression")
        .unwrap()
        .args(["encode", "--help"])
        .assert()
        .success()
        .stdout(HELP_STR);
}
