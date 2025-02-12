use std::collections::HashMap;
use std::fs;
use std::fs::read_to_string;
// use std::env;
use std::path::Path;
use crate::lexer::preprocessing;
use crate::lexer::tokenizer;

use crate::lexer::different_approach;


// fn word ( par1 : u32 , par2 : u32 ) -> u32 { }

fn get_hash_map() -> HashMap<String, String> {
    let le_map: HashMap<String, String> = vec![
        ("fun", "fn"),
        ("print", "println!"),

        (";", "")
    ]
    .into_iter()
    .map(|x| (String::from(x.0), String::from(x.1)))
    .collect();

    le_map
}


fn rustify_code(code: &Vec<String>) -> Vec<String> {
    let mut rustified_code: Vec<String> = Vec::new();
    let my_map = get_hash_map();

    for string in code {
        if let Some(value) = my_map.get(string) {
            rustified_code.push(value.clone());
        } else {
            rustified_code.push(string.clone());
        }
    }

    rustified_code
}

pub fn main() {
    // let sample_file_path = Path::new("src/sample_files/first_example_file.fncy");
    // let (symbol_idx, text_chars) = preprocessing::get_symbol_idx_and_chars_from_file(sample_file_path);

    let code: Vec<String> = different_approach::main().split(" ").map(String::from).collect();

    let rustified_code = rustify_code(&code);

    println!("{:?}", rustified_code.join(" "));


    // tokenizer::main(&text_chars, &symbol_idx);
    // preprocessing::test_char_shenanigans(&text_chars, &symbol_idx);
}
