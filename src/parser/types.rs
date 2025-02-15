use std::collections::HashMap;


#[derive(Debug)]
pub struct Parameter {
    pub name: String,
    pub type_fncy: String,
    pub type_rs: String,
    pub default_value: String
}

pub struct Fun {
    name: String,

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