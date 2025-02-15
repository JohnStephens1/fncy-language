use std::collections::btree_map::Range;
use std::collections::HashMap;
use std::fs;
use std::fs::read_to_string;
// use std::env;
use std::path::Path;
use std::result;
use crate::lexer::preprocessing;
use crate::lexer::tokenizer;

use crate::lexer::different_approach;


// fn word ( par1 : u32 , par2 : u32 ) -> u32 { }

fn get_hash_map() -> HashMap<String, String> {
    let le_map: HashMap<String, String> = vec![
        ("fun", "fn"),
        ("print", "println!"),

        (";", ""),
        ("'", "\"")
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

fn split_matching_braces(vec: Vec<String>) -> Vec<String> {
    let mut brace_count = 0;
    let mut in_brace: bool;
    let mut last_in_brace: bool = false;

    let mut current_match: Vec<String> = Vec::new();
    let mut result: Vec<String> = Vec::new();

    for string in vec {
        brace_count += if string == "{" {1} else if string == "}" {-1} else {0};
        in_brace = if brace_count > 0 { true } else { false };
    
        if last_in_brace == false && in_brace == true {
            result.push(current_match.join(" "));
            current_match.clear();
            current_match.push(string);
        } else if last_in_brace == true && in_brace == false {
            current_match.push(string);
            result.push(current_match.join(" "));
            current_match.clear();
        } else {
            current_match.push(string);
        }

        last_in_brace = in_brace;
    };

    result
}

fn split_from_brace_to_next_match(vec: &Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut brace_count = 0;
    let mut last_i = 0;

    for string in vec {
        last_i += 1;
        brace_count += if string == "{" {1} else if string == "}" {-1} else {0};

        if brace_count == 0 {
            break;
        }
    }

    if last_i == 1 { println!("first input wasn't a brace!"); }

    (vec[..last_i].to_vec(), vec[last_i..].to_vec())
}


pub fn main() {
    // let sample_file_path = Path::new("src/sample_files/first_example_file.fncy");
    // let (symbol_idx, text_chars) = preprocessing::get_symbol_idx_and_chars_from_file(sample_file_path);

    let code: Vec<String> = different_approach::main().split(" ").map(String::from).collect();

    let rustified_code = rustify_code(&code);

    testing_schtick(code.clone());

    // tokenizer::main(&text_chars, &symbol_idx);
    // preprocessing::test_char_shenanigans(&text_chars, &symbol_idx);
}


fn testing_schtick(code: Vec<String>) {
    let result = split_matching_braces(code.clone());

    println!("result: {:?}", result);
}
