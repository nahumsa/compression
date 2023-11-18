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
            
            // generate tree from file
            let tree = HuffmanNode::from_freq_table(character_frequency);
            let mut got: HashMap<char, String> = HashMap::new();
            generate_encoding_table_from_tree(tree, &mut got, "".to_string());
            
            // converting the string to a vector of strings corresponding to bits
            let mut result_vec: Vec<String> = Vec::new();
            for c in file_content.chars() {
                if let Some(output_string) = got.get(&c) {
                    result_vec.push(output_string.clone());
                } else {
                    println!("Character: '{}' not found in the map", c);
                }
            }
            println!("{:?}", result_vec);
            
            // generate tree representation to json
            let serialized_data = serde_json::to_string(&got).unwrap();
            let result = format!("{}\n--\n{}", serialized_data, result_vec.join(" "));
            match write_to_file("test.huff", &result) {
                Ok(()) => println!("saved file"),
                Err(_) => println!("failed to save file"),
            };
        }
        Commands::Decode(opts) => {
            let file_content = read_file(opts.filename.as_str());
            
            if let Some((first_part, second_part)) = file_content.split_once("\n--\n") {
                let encoded_tree: HashMap<char, String> = serde_json::from_str(first_part).expect("failed to deserialize");
                let decoded_tree: HashMap<String, char> = encoded_tree.iter().map(|(k, v)| (v.clone(), k.clone())).collect();
                println!("First part: {:?}", decoded_tree);
                let mut result_vec: Vec<String> = Vec::new();
                for code in second_part.split(" ") {
                     result_vec.push(decoded_tree.get(code).unwrap().to_string());
                }
                println!("Second part: {}", result_vec.join(""));
            } else {
                println!("Delimiter not found in the string");
            }
        }
    }
}
