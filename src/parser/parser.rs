use std::collections::HashMap;
use std::str::Chars;

use crate::lexer::{different_approach, lexer};
use crate::util::processing::{self, split_matching_parenthesis};

use super::parser_util;
use super::types;


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

    // todo can theoretically go out of bounds, should test for that
    // excplicit none test, safe indexing or while i < code.len
    // last is best i think
    for _ in 0..code.len() {
        match &code[i] {
            s if braces.contains(s) => {
                i += processing::get_i_of_next_matching_char(&code[i..], s, matchers.get(s).unwrap());
                was_last_operator = false;
            },
            s if string_delims.contains(s) => {
                i += processing::get_i_of_next_delim(&code[i..], s);
                was_last_operator = false;
            }
            s if operators.contains(s) => was_last_operator = true,
            _ => if was_last_operator { was_last_operator = false } else { break }
        }

        i += 1;
    }

    i-1
}

pub fn find_end_of_expression(code: &Vec<String>) -> (Vec<String>, Vec<String>) {
    let i = get_i_of_end_of_expression(code);

    let (le_match, remainder) = processing::get_match_and_remainder_from_i(code, i);

    (le_match, remainder)
}


fn expression_handler(code: &Vec<String>) {
    let operator_vec = types::get_operator_list();

    for string in code.iter() {
    }
}


// todo implement translate_type_fncy
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

// todo fixed, but still hideous xd feel free to update
fn get_parameters(mut params: &[String]) -> Vec<types::Parameter> {
    params = &params[1..params.len() - 1];

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


fn get_return_type(code: &Vec<String>) -> (String, String) {
    if code.is_empty() {
        ("".to_string(), "".to_string())
    } else {
        let return_type_fncy = code[1..].join(" ");
        let (is_mutable, return_type_rs) = types::translate_type_fncy(&return_type_fncy);

        (return_type_fncy, return_type_rs)
    }
}

fn smth(a_string: & mut String) -> & mut String {
    a_string

}


fn get_return_type_updated(code: &Vec<String>) -> String {
    let smth = match code.first() {
        Some(s) => code[1..].join(" "),
        _ => "".to_string()
    };

    let smth = if code.is_empty() { "".to_string() } else { code[1..].join(" ") };

    let pos_next_brace = code.iter().position(|s| s == "{").unwrap(); // should never panic
    let return_type = code[1..pos_next_brace].join(" ");

    return_type
}

fn get_fun(slice: &[String]) {

}

fn fun_def_handler(code: &Vec<String>) -> types::Fun {
    if code.first().expect("no fun, nothing at all tbh") != "fun" { panic!("no fun indeed") };

    let fun_name = code[1].clone();

    let end_of_params = processing::get_i_of_next_matching_parenthesis(&code[2..]) + 2;
    let raw_params = &code[2..=end_of_params];
    let fun_params = get_parameters(raw_params);

    let start_of_code = end_of_params + code[end_of_params..].iter().position(|s| s == "{").expect("no { after fun_def");
    let raw_return_type = &code[end_of_params+1..start_of_code];
    let return_type = if raw_return_type.is_empty() { "".to_string() } else { raw_return_type[1..].join(" ") };

    let end_of_code = start_of_code + processing::get_i_of_next_matching_brace(&code[start_of_code..]);
    let fun_code = code[start_of_code..=end_of_code].to_vec();

    types::Fun {
        name: fun_name,
        parameters: fun_params,
        return_type,
        code: fun_code
    }
}

fn fun_call_handler(code: &Vec<String>) {

}

fn let_handler(code: &Vec<String>) {

}


fn analyze_code(code: &Vec<String>) {
    let mut i = 0;

    for _ in 0..code.len() {
        match code[i].as_str() {
            "fun" => {
                let my_fun = fun_def_handler(code);

                dbg!(&my_fun);
            },
            _ => {} // println!("today i dont feel like doing aahnything")
        }

        i += 1;
    }
}

fn run_parser(code: &Vec<String>) {
    get_matchers();

    // analyze_code(code);
    analyze_code(code);
}


pub fn main() {
    let code: Vec<String> = different_approach::main().split(" ").map(String::from).collect();
    run_parser(&code);
    // types::test_extract_var_info();
    // imatacompletefkinlossxd();
    // test_get_variable();
    // test_map();
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
}
