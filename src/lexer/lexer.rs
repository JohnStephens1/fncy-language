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

fn split_matching_braces_old(vec: Vec<String>) -> Vec<String> {
    let mut brace_count = 0;
    let mut in_brace_vec: Vec<String> = Vec::new();
    let mut not_in_brace_vec: Vec<String> = Vec::new();
    let mut result: Vec<String> = Vec::new();

    for string in vec {
        if string == "{" {
                brace_count += 1;
                in_brace_vec.push(string) // cuz brace_count will always be > 0;
        } else if string == "}" {
            brace_count -= 1;
            in_brace_vec.push(string);
            if brace_count <= 0 {
                result.push(not_in_brace_vec.join(" "));
                not_in_brace_vec.clear();
                result.push(in_brace_vec.join(" "));
                in_brace_vec.clear();
            };
        } else {
            if brace_count > 0 {
                in_brace_vec.push(string);
            } else {
                not_in_brace_vec.push(string);
            }
        }
    };     //  always push to result?

    result
}

// not actually working properly? not returning { ... } but { ...] [ } ...
fn split_matching_braces_new(vec: Vec<String>) -> Vec<String> {
    let mut brace_count = 0;
    let mut in_brace: bool;
    let mut last_in_brace: bool = false;

    let mut current_match: Vec<String> = Vec::new();
    let mut result: Vec<String> = Vec::new();

    for string in vec {
        brace_count += if string == "{" {1} else if string == "}" {-1} else {0};
        in_brace = if brace_count > 0 { true } else { false };
    
        if in_brace != last_in_brace {
            result.push(current_match.join(" "));
            current_match.clear();
        };

        current_match.push(string);
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

    // println!("{:?}", code.join(" "));
    // println!("{:?}", rustified_code.join(" "));

    testing_schtick(code.clone());
    // println!("{:?}", &code[..10]);
    // println!("{:?}", &code[0..1]);

    // println!("code: {:?}", code);

    let (result_one, result_two) = split_from_brace_to_next_match(&code[13..].to_vec());

    // println!("result_one: {:?}\n", result_one);
    // println!("result_two: {:?}\n", result_two);

    // println!("code: {:?}", code);

    // test_for_single_match_thingy();
    // the_simplest_things_aint_working_xd();

    // tokenizer::main(&text_chars, &symbol_idx);
    // preprocessing::test_char_shenanigans(&text_chars, &symbol_idx);
}


fn testing_schtick(code: Vec<String>) {
    let result_old = split_matching_braces_old(code.clone());
    let result_new = split_matching_braces_new(code.clone());

    println!("result_old: {:?}", result_old);
    println!("result_new: {:?}", result_new);
}

fn test_for_single_match_thingy() {
    let sample_vec: Vec<String> = [
        "fun multiply_two_numbers ( num_1 : int , num_2 : int ) -> int", "{ num_1 * num_2 ; }", "fun main ( )", "{ let number_1 : int = 15 let number_2 : int = 30 let result : vint = 0 let result = multiply_two_numbers ( number_1 , number_2 ) print ( f ' result : { result } ' ) }"
        ].into_iter().map(String::from).collect();
    

    println!("sample_vec: {:?}\n", sample_vec);
    split_from_brace_to_next_match(&sample_vec);

    // println!("{:?}", result);
}


fn the_simplest_things_aint_working_xd() {
    let simple_array = ["hello", "to", "you"];
    let gud_string_vec: Vec<String> = simple_array.into_iter().map(String::from).collect();

    println!("gud_string_vec: {:?}\n", gud_string_vec);
}