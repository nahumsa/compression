use assert_cmd::Command;

const HELP_STR: &str = "decode a file

Usage: compression decode <FILENAME>

Arguments:
  <FILENAME>  

Options:
  -h, --help  Print help
";

#[test]
fn help_decode() {
    Command::cargo_bin("compression")
        .unwrap()
        .args(["decode", "--help"])
        .assert()
        .success()
        .stdout(HELP_STR);
}
