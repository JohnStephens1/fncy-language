use std::collections::HashMap;


#[derive(Debug)]
pub struct Parameter {
    // add references, mutability
    pub name: String,
    pub type_fncy: String,
    pub type_rs: String,
    pub default_value: String
}

pub struct Fun {
    name: String,

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

pub fn get_variable(name: String, type_fncy: String, value: String) -> Variable {
    let le_map = get_param_type_hashmap();
    // let type_rs = le_map.get(&type_fncy).unwrap().clone();

    let type_rs_original = match le_map.get(&type_fncy) {
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
        ("float", "f32"),
        ("vfloat", "mut f32"),
        ("string", "String"),
        ("vstring", "mut String"),
    ]
    .into_iter()
    .map(|x| (String::from(x.0), String::from(x.1)))
    .collect()
}

pub fn get_operator_list() -> Vec<String> {
    [
        "+", "-", "*", "/", "%",
        "(", ")", "{", "}", "[", "]", "<", ">",
        ":", "=", "==",
        "&&", "and", "||", "or", "!", "not"
    ]
    .into_iter()
    .map(String::from)
    .collect()
}