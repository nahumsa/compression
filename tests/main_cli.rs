use assert_cmd::Command;

const HELP_STR: &str = "Compress files using Huffman coding

Usage: compression <COMMAND>

Commands:
  encode  encode a file
  decode  decode a file
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
";

#[test]
fn help() {
    Command::cargo_bin("compression")
        .unwrap()
        .args(["--help"])
        .assert()
        .success()
        .stdout(HELP_STR);
}
