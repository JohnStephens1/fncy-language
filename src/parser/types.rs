use std::{collections::HashMap, result, string};


#[derive(Debug)]
pub struct Parameter {
    // add references, mutability
    pub name: String,
    pub type_fncy: String,
    pub type_rs: String,
    pub default_value: String
}

#[derive(Debug)]
pub struct Fun {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: String,
    pub code: Vec<String>
}

#[derive(Debug)]
pub struct Variable {
    // add references
    pub name: String,
    pub is_mutable: bool,
    pub type_fncy: String,
    pub type_rs: String,
    pub value: String
}

pub fn split_fncy_type(string: &str) -> (String, String) {
    let chars: Vec<char> = string.chars().collect();
    let mut i = 0;

    let mut prev: &[char] = &chars[0..0];
    let mut le_type: &[char] = &chars[0..0];

    let mut last_char: char = ' ';
    while i < chars.len() {
        if chars[i]=='v' && last_char!='v' {
            last_char = chars[i];
            i += 1;
        } else if chars[i]=='&' {
            last_char = chars[i];
            i += 1;
        } else {
            prev=&chars[..i];
            le_type=&chars[i..];
            break
        }
    }

    let prev_res = prev.iter().collect::<String>();
    let le_type_res = le_type.iter().collect::<String>();

    (prev_res, le_type_res)
}


fn check_chars(string: &String) -> bool {
    let le_map: HashMap<String, String> = [
        ("int", "i32"),
        ("uint", "u32"),
        ("usize", "usize"),
        ("float", "f32"),
        ("string", "String"),
        ("&str", "&str"), // todo need to find a solution for u
        ("char", "char"),
    ]
    .into_iter()
    .map(|x| (String::from(x.0), String::from(x.1)))
    .collect();

    let mut le_result = "somethingv&int".to_string();
    let mut le_type_fncy = "".to_string();

    for (key, value) in le_map {
        if let Some(pos) = le_result.find(&key) {
            le_result = le_result.replacen(&key, "", 1);
            le_type_fncy.push_str(&value);
        }
    }

    dbg!(&le_result);
    dbg!(&le_type_fncy);


    let mut i = 0;

    let mut chars: Vec<char> = Vec::new();
    let mut last_char: char;

    let mut ref_count: usize = 0;
    let mut is_ref: bool = false; // actually pbb don't want this here
    let mut is_var_ref: bool = false;
    let mut is_var: bool = false;
    let mut le_type: Vec<char> = Vec::new();

    for char in string.chars().filter(|c| !c.is_whitespace()) {
        match char {
            s if s=='v' => {
                if i == 0 {
                    // nothing rly
                } else {
                    // if chars[i-1] == '&' && chars.get(i+1) != 'v'{
                    //     is_var = true;
                    // }
                }
            }, // mark v
            s if s=='&' => {}, // mark &
            _ => {} // mark type
        };

        chars.push(char);
        i += 1;
    };

    let type_fncy: String = chars.into_iter().collect();
    dbg!(&type_fncy);

    true
}

fn lets_write_another_one_xd(type_fncy: &String) -> String {
    let smth = type_fncy;


    "".to_string()
}

// todo once types have properly crystallized implement remaining cases
// limited to the more usual cases 4 now
pub fn translate_type_fncy_compact(type_fncy: &Vec<String>) -> String {
    let le_map = get_param_type_hashmap();

    let mut ref_count: usize = 0;
    let mut is_mutable = false;
    let mut type_fncy_isolated: String = "".to_string();

    for string in type_fncy {
        match string.as_str() {
            // could contain braces
            "&" => ref_count += 1,
            _ => {
                if string.starts_with("v") {
                    is_mutable = true;
                    type_fncy_isolated = string[1..].to_string();
                } else {
                    type_fncy_isolated = string.to_string();
                }

                break
            }
        }
    };

    dbg!(&type_fncy_isolated);

    let type_rs_isolated = le_map.get(&type_fncy_isolated).expect(&format!("received invalid type: {}", &type_fncy_isolated)).clone();

    let type_rs_compact =
        if ref_count > 0 { "& ".to_string() } else { "".to_string() } +
        if is_mutable { "mut " } else { "" } +
        type_rs_isolated.as_str();

    type_rs_compact
}

pub fn translate_type_fncy(type_fncy: &String) -> (bool, String) {
    let le_map = get_param_type_hashmap();

    let type_rs_original = match le_map.get(type_fncy) {
        Some(s) => s.clone(),
        _ => {
            println!("didn't get a proper type!");
            "".to_string()
        }
    };

    let type_rs_vec: Vec<&str> = 
        type_rs_original
        .split(" ").collect();

    let is_mutable = if type_rs_vec.contains(&"mut") { true } else { false };

    let type_rs: String =
        type_rs_vec
        .last()
        .unwrap_or_else(|| &"")
        .to_string();

    (is_mutable, type_rs)
}

pub fn get_variable(name: String, type_fncy: String, value: String) -> Variable {
    let (is_mutable, type_rs) = translate_type_fncy(&type_fncy);

    Variable {
        name,
        is_mutable,
        type_fncy,
        type_rs,
        value,
    }
}



pub fn get_param_type_hashmap() -> HashMap<String, String> {
    [
        ("int", "i32"),
        ("vint", "mut i32"),
        ("uint", "u32"),
        ("vuint", "mut u32"),
        ("usize", "usize"),
        ("vusize", "mut usize"),
        ("float", "f32"),
        ("vfloat", "mut f32"),
        ("string", "String"),
        ("vstring", "mut String"),
    ]
    .into_iter()
    .map(|x| (String::from(x.0), String::from(x.1)))
    .collect()
}


pub fn get_braces() -> Vec<String> {
    [
        "(", ")", "{", "}", "[", "]"
    ]
    .into_iter()
    .map(String::from)
    .collect()
}

pub fn get_string_delims() -> Vec<String> {
    [
        "\"", "'", "`"
    ]
    .into_iter()
    .map(String::from)
    .collect()
}

pub fn get_operator_list() -> Vec<String> {
    [
        "+", "-", "*", "/", "%",
        "<", ">", ":", "=", "==",
        "&&", "and", "||", "or", "!", "not",
        "."
    ]
    .into_iter()
    .map(String::from)
    .collect()
}