use std::collections::HashMap;

use crate::lexer::{different_approach, lexer};
use crate::util::processing::{self, split_matching_parenthesis};
use crate::parser::parser_util;
use crate::parser::types;


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
    ]
    .into_iter()
    .map(|x| (String::from(x.0), String::from(x.1)))
    .collect()
}

fn get_i_of_end_of_expression(code: &Vec<String>) -> usize {
    let mut i: usize = 0;
    let mut was_last_operator = true;

    let matchers = get_matchers();
    let braces = types::get_braces();
    let string_delims = types::get_string_delims();
    let operators = types::get_operator_list();


    for _ in 0..code.len() {
        match &code[i] {
            s if braces.contains(s) || string_delims.contains(s) => {
                i += processing::get_i_of_next_matching_char(&code[i..], s, matchers.get(s).unwrap()) - 1;
                was_last_operator = false;
            },
            s if string_delims.contains(s) => {
                i += processing::get_i_of_next_delim(&code[i..], s) - 1;
                was_last_operator = false;
            }
            s if operators.contains(s) => was_last_operator = true,
            _ => if was_last_operator { was_last_operator = false } else { break }
        }

        i += 1;
    }

    i
}

fn find_end_of_expression(code: &Vec<String>) -> (Vec<String>, Vec<String>) {
    let i = get_i_of_end_of_expression(code);

    let le_match = code[0..i].to_vec();
    let remainder = code[i..].to_vec();

    (le_match, remainder)
}


fn expression_handler(code: &Vec<String>) {
    let operator_vec = types::get_operator_list();

    for string in code.iter() {
    }
}


fn get_parameter(name: String, type_fncy: String, default_value: String) -> types::Parameter {
    let le_map = types::get_param_type_hashmap();
    let type_rs = le_map.get(&type_fncy).unwrap().clone();

    types::Parameter {
        name,
        type_fncy,
        type_rs,
        default_value
    }
}

fn get_parameters(params: &[String]) -> Vec<types::Parameter> {
    let mut parameters: Vec<types::Parameter> = Vec::new();

    let mut comma_index: Vec<usize> = Vec::new();
    let mut colon_index: Vec<usize> = Vec::new();
    let mut equals_index: Vec<usize> = Vec::new();

    let mut default_value = "".to_string();

    let mut i: usize = 0;
    for string in params {
        match string.as_str() {
            "," => comma_index.push(i),
            ":" => colon_index.push(i),
            "=" => equals_index.push(i),
            _ => {}
        }

        i += 1;
    }

    i = 0;
    let mut i_equals: i32 = 0;
    for colon_idx in &colon_index {
        if let Some(equals_idx) = equals_index.get(i_equals as usize) {
            if let Some(comma_idx) = comma_index.get(i) {
                if comma_idx < equals_idx {
                    i_equals -= 1;
                } else {
                    default_value = params[*equals_idx + 1..*comma_idx].join(" ");
                }
            } else {
                default_value = params[*equals_idx + 1..].join(" ");
            }
        }

        parameters.push(get_parameter(
            params[colon_idx - 1].clone(),
            params[colon_idx + 1].clone(),
            default_value.clone()
        ));

        i_equals += 1;
        i += 1;
        default_value = "".to_string();
    }

    parameters
}

fn fun_def_handler(code: &Vec<String>) {
    if code.first().expect("no fun indeed") == "fun" {};

    let fun_name = code[1].clone();
    let (parameters, remainder) = split_matching_parenthesis(&code[2..]);
    let fun_params = get_parameters(&parameters[1..&parameters.len()-1]); 

    dbg!(fun_params);

    let my_slice = code[0..2].to_vec();
    dbg!(parameters);
    dbg!(remainder);
    dbg!(my_slice);
    println!("handling fun def")
}

fn fun_call_handler(code: &Vec<String>) {

}

fn let_handler(code: &Vec<String>) {

}

fn analyze_code(code: &Vec<String>) {
    for string in code.iter() {
        match string.as_str() {
            "fun" => fun_def_handler(code),
            _ => {} // println!("today i dont feel like doing aahnything")
        }
    }
}


fn run_parser(code: &Vec<String>) {
    get_matchers();

    analyze_code(code);
}


pub fn main() {
    let code: Vec<String> = different_approach::main().split(" ").map(String::from).collect();
    // run_parser(&code);

    // println!("parser says hello");
    // println!("code: {:?}", code);

    test_find_end_of_expression()
    // test_split_at_next_delim();
    // test_get_variable();
    // test_map();
    // test_char_split_boi();
}






fn test_find_end_of_expression() {
    let test_vec_1: Vec<String> =
        "something . something ( hell hello hello ) . guten_tag shoulda_stopped_here hello ( )"
        .split_ascii_whitespace().into_iter().map(String::from).collect();

    let test_vec_2: Vec<String> =
        "something something ( hell hello hello ) . guten_tag shoulda_stopped_here hello ( )"
        .split_ascii_whitespace().into_iter().map(String::from).collect();

    let (le_match, remainder) = find_end_of_expression(&test_vec_2);

    dbg!(le_match);
    dbg!(remainder);
}

fn test_split_at_next_delim() {
    let test_vec: Vec<String> = 
    ["\"", "hello", "chikn", "mc", "nuggit", "\"", "hello", "two", "\"", "yuhs", "\"", "eh"]
    // ["\"", "hello", "chikn", "mc", "nuggit", "\"", "hello", "two", "\"", "yuhs", "\"", "eh"]
    .into_iter()
    .map(String::from)
    .collect();

    let (le_match, remainder) = processing::split_at_next_delim(&test_vec, "\"");

    dbg!(le_match);
    dbg!(remainder);
}

fn test_get_variable() {
    let name = "isanem".to_string();
    let type_fncy = "vstring".to_string();
    let value = "hello there".to_string();
    let my_var = types::get_variable(name, type_fncy, value);

    dbg!(my_var);
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
    // let (le_match, remainder): (Vec<String>, Vec<String>) = processing::split_matching_brace(&test_vec);

    println!("test_vec: {:?}", test_vec);
    println!();
    println!("match: {:?}", le_match);
    println!("remainder: {:?}", remainder);
}