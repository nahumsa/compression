use assert_cmd::Command;

const HELP_STR: &str = "Compress files using Huffman coding

Usage: compression [OPTIONS] <INPUT> ...

Arguments:
  <INPUT>       Path of the input

Options:
  -h, --help              Print help
";

const COMPRESSED_SAMPLE_FILE: &str = "00011111";

const SAMPLE_FILE: &str = "hello world";

#[test]
fn help() {
    Command::cargo_bin("compression")
        .unwrap()
        .args(["--help"])
        .assert()
        .success()
        .stdout(HELP_STR);
}

#[test]
#[ignore]
fn encode() {
    Command::cargo_bin("compression")
        .unwrap()
        .args(["encode", "tests/sample/sample.txt"])
        .assert()
        .success()
        .stdout(COMPRESSED_SAMPLE_FILE);
}

#[test]
#[ignore]
fn decode() {
    Command::cargo_bin("compression")
        .unwrap()
        .args(["decode", "tests/sample/sample_encoded.txt"])
        .assert()
        .success()
        .stdout(SAMPLE_FILE);
}
