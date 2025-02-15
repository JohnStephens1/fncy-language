use std::collections::HashMap;

use crate::lexer::lexer;
use crate::util::processing;
// use crate::parser::types;


fn vec_gen_boi() -> Vec<String> {
    ["hello"]
    .into_iter()
    .map(String::from)
    .collect()
}

fn get_matchers() -> HashMap<String, String> {
    [
        ("(", ")"),
        ("{", "}"),
        ("[", "]"),
        ("<", ">"),

        ("\"", "\""),
        ("'", "'"),
    ];

    
    let test_string = "hello there \" im a potaootahr \"".to_string();
    let le_map: HashMap<String, String> = vec![
        ("\"", "HALLO"),
        ("a", "AHHHHHH"),
        ("\"", "QUOTIATIONN MERKK"),
    ]
    .into_iter()
    .map(|x| (String::from(x.0), String::from(x.1)))
    .collect();

    le_map
}


fn statement_handler() {

}

fn fun_def_handler() {

}

fn fun_call_handler() {

}

fn let_handler() {

}


fn run_parser() {
    get_matchers();
}


pub fn main() {
    run_parser();
    // println!("parser says hello");

    // test_map();
    test_char_split_boi();
}


fn test_map() {
    let le_map = get_matchers();
    let test_string = "hello there \" im a potaootahr \"".to_string();

    let mut result = Vec::new();

    for x in test_string.split(" ") {
        if let Some(value) = le_map.get(x) {
            result.push(value.clone());
        } else {
            result.push(x.to_string().clone());
        }
    }

    let result_str = result.join(" ");
    println!("result: {:?}", result_str);



    // for string in code {
    //     if let Some(value) = le_map.get(string) {
    //         x.push(value.clone());
    //     } else {
    //         x.push(string.clone());
    //     }
    // }
}


fn test_char_split_boi() {
    let test_vec: Vec<String> = "{ hello there { } potato potato ( ( ( ) ) ) ( hello ) { } } helooooooo mah nehme heff { } ( ) ) ) { { (".split_ascii_whitespace()
    .into_iter()
    .map(String::from)
    .collect();
    let (le_match, remainder): (Vec<String>, Vec<String>) = processing::split_matching_brace(&test_vec);

    println!("test_vec: {:?}", test_vec);
    println!();
    println!("match: {:?}", le_match);
    println!("remainder: {:?}", remainder);
}