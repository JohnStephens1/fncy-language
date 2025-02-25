use std::collections::HashMap;

use crate::lexer::different_approach;
use crate::util::processing;
use crate::util::char_sets;
use crate::types::{var, fun};


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

fn get_i_of_end_of_expression(code: &[String]) -> usize {
    let mut i: usize = 0;
    let mut was_last_operator = true;

    let matchers = get_matchers();
    let braces = char_sets::get_braces();
    let string_delims = char_sets::get_string_delims();
    let operators = char_sets::get_operator_list();

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
    let operator_vec = char_sets::get_operator_list();

    for string in code.iter() {
    }
}


// todo implement translate_type_fncy
// fn get_parameter(name: String, type_fncy: String, default_value: String) -> types::Parameter {
//     let le_map = types::get_param_type_hashmap();
//     let type_rs = le_map.get(&type_fncy).unwrap().clone();

//     types::Parameter {
//         name,
//         type_fncy,
//         type_rs,
//         default_value
//     }
// }


fn get_fun(slice: &[String]) {

}

// fn fun_def_handler(code: &[String]) -> (Fun, usize) {
//     if code.first().expect("no fun, nothing at all tbh") != "fun" { panic!("no fun indeed") };

//     let fun_name = code[1].clone();

//     let end_of_params = processing::get_i_of_next_matching_parenthesis(&code[2..]) + 2;
//     let raw_params = &code[2..=end_of_params];
//     let fun_params = Fun::fun::get_parameters(raw_params);

//     let start_of_code = end_of_params + code[end_of_params..].iter().position(|s| s == "{").expect("no { after fun_def");
//     let raw_return_type = &code[end_of_params+1..start_of_code];
//     let return_type = if raw_return_type.is_empty() { "".to_string() } else { raw_return_type[1..].join(" ") };

//     let end_of_code = start_of_code + processing::get_i_of_next_matching_brace(&code[start_of_code..]);
//     let fun_code = code[start_of_code..=end_of_code].to_vec();

//     let my_fun = Fun {
//         name: fun_name,
//         parameters: fun_params,
//         return_type,
//         code: fun_code
//     };

//     (my_fun, end_of_code)
// }

fn fun_call_handler(code: &Vec<String>) {

}

// currently requires an assignment
pub fn let_handler(code: &[String]) -> (var::Var, usize) {
    if code.first().expect("received empty string") != "let" { panic!("no let") };

    // case open:
    // let name : type -> let name : < type >

    let var_name = code[1].clone();

    let equals_pos = code.iter().position(|s| s=="=").expect("no equals");
    let le_type = code[3..equals_pos].join(" ");

    let exp_end_i = get_i_of_end_of_expression(&code[equals_pos+1..]) + equals_pos+1;
    let exp = code[equals_pos+1..=exp_end_i].to_vec();

    let my_var = var::Var::new(
        var_name,
        le_type,
        exp
    );

    (my_var, exp_end_i)
}


fn analyze_code(code: &Vec<String>) {
    let mut i = 0;
    let mut start;
    let mut fun_start_end: Vec<(usize, usize)> = Vec::new();

    // might want to find an alternative to the loop
    // dont like the index return as is
    // rn only reads the functions
    while i < code.len() {
        match code[i].as_str() {
            "fun" => {
                start = i;
                let (my_fun, end) = fun::fun_def_handler(&code[i..]);
                fun_start_end.push((start, end));
                i += end;
            },
            "let" => {
                start = i;
                let (my_var, end) = let_handler(&code[i..]);
                fun_start_end.push((start, end));
                i += end;
            },
            _ => {} // println!("today i dont feel like doing aahnything")
        }

        i += 1;
    }
}


fn analyze_code_posified(code: &Vec<String>) {
    // code.iter().position(predicate)
}

fn run_parser(code: &Vec<String>) {
    // get_matchers();

    analyze_code(code);
}


pub fn main() {
    let code: Vec<String> = different_approach::main().split(" ").map(String::from).collect();
    run_parser(&code);
    
    // test_le_fun();

    // test_let_handler();
    // types::test_extract_var_info();
    // imatacompletefkinlossxd();
    // test_get_variable();
    // test_map();
}



fn test_le_fun() {
    let test: Vec<String> = "( hello : && vint , my : int = 5 , name : int = 5 * 5 * 5 , jeff : int )".split(" ").map(String::from).collect();
    // dbg!(&test[1..test.len()-1]);

    crate::types::fun::get_parameters(&test);
}


fn test_get_variable() {
    let name = "isanem".to_string();
    let type_fncy = "vstring".to_string();
    let value = vec!["hello there".to_string()];
    let my_var = var::Var::new(name, type_fncy, value);

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
