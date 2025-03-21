use std::collections::HashMap;


#[derive(Debug, Default)]
pub struct Parameter {
    // add references, mutability
    pub name: String,
    pub type_fncy: String,
    pub type_rs: String,
    pub default_value: String
}

impl Parameter {
    fn new(name: String, type_fncy: String) -> Self {
        Self {
            name,
            type_fncy,
            type_rs: "".to_string(),
            default_value: "".to_string()
        }
    }

    fn new_wd(name: String, type_fncy: String, default_value: String) -> Self {
        Self {
            name,
            type_fncy,
            type_rs: "".to_string(),
            default_value
        }
    }

}

fn get_string(string: &str) -> String { string.to_string() }



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