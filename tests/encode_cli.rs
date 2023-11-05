use assert_cmd::Command;

const HELP_STR: &str = "encode a file

Usage: compression encode <FILENAME>

Arguments:
  <FILENAME>  filename to encode

Options:
  -h, --help  Print help
";

const ENCODE_OUTPUT_STR: &str = r#"[('w', 1), ('r', 1), ('o', 2), ('l', 3), ('h', 1), ('e', 1), ('d', 1), (' ', 1), ('\n', 1)]"#;

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
        .stdout(ENCODE_OUTPUT_STR.to_owned() + "\n");
}
