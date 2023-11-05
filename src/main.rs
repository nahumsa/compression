use clap::{Parser, Subcommand, ValueHint};
use compression::io::read_file;

#[derive(Debug, Parser)]
#[command(name = "compression")]
#[command(about = "Compress files using Huffman coding", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(arg_required_else_help = true, about = "encode a file")]
    Encode(EncodeOpts),
    #[command(arg_required_else_help = true, about = "decode a file")]
    Decode(DecodeOpts),
}

#[derive(Parser, Debug)]
pub struct EncodeOpts {
    #[arg(value_hint = ValueHint::FilePath, help = "filename to encode")]
    pub filename: String,
}

#[derive(Parser, Debug)]
pub struct DecodeOpts {
    // filename to encode
    #[arg(value_hint = ValueHint::FilePath, help = "filename to decode")]
    pub filename: String,
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Encode(opts) => {
            let file_content = read_file(opts.filename.as_str());
            println!("{}", file_content)
        }
        Commands::Decode(opts) => {
            println!("decoding {}", opts.filename);
        }
    }
}
