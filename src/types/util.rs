use std::collections::HashMap;


pub fn get_param_type_hashmap() -> HashMap<String, String> {
    [
        ("int", "i32"),
        ("uint", "u32"),
        ("usize", "usize"),
        ("float", "f32"),

        ("char", "char"),
        // ("string", "str"), // todo special case, implement
        ("string", "String"),
    ]
    .into_iter()
    .map(|x| (String::from(x.0), String::from(x.1)))
    .collect()
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