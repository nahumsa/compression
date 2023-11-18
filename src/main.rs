use std::collections::HashMap;

use clap::{Parser, Subcommand, ValueHint};
use compression::{
    domain::{
        char_count::character_count, tree::generate_encoding_table_from_tree, tree::HuffmanNode,
    },
    io::{read_file, write_to_file},
};

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
            let character_frequency = character_count(&file_content);
            let mut sorted_vec: Vec<_> = character_frequency.iter().collect();
            sorted_vec.sort_by(|a, b| b.0.cmp(a.0));
            println!("{:?}", sorted_vec);
            let tree = HuffmanNode::from_freq_table(character_frequency);
            let mut got: HashMap<char, String> = HashMap::new();
            generate_encoding_table_from_tree(tree, &mut got, "".to_string());
            let serialized_data = serde_json::to_string(&got).unwrap();

            let mut result_vec: Vec<String> = Vec::new();
            for c in file_content.chars() {
                if let Some(output_string) = got.get(&c) {
                    result_vec.push(output_string.clone());
                } else {
                    println!("Character: '{}' not found in the map", c);
                }
            }
            println!("{:?}", result_vec);
            match write_to_file("test.huff", &serialized_data) {
                Ok(()) => println!("saved file"),
                Err(_) => println!("failed to save file"),
            };
        }
        Commands::Decode(opts) => {
            println!("decoding {}", opts.filename);
        }
    }
}
