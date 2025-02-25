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

// the question is, include & value -> &v alue?
// guess for more complex types capital letters are in order
pub fn split_type_fncy_raw(string: &str) -> (String, String) {
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
    let type_fncy = le_type.iter().collect::<String>();

    (prev_res, type_fncy)
}

pub fn and_another_fncy_type_splitter(string: &str) -> (String, String) {
    let chars: Vec<char> = string.chars().filter(|x| !x.is_whitespace()).collect();

    let mut end = 0;
    let mut last_char = ' ';

    for (i, char) in chars.iter().enumerate() {
        match (i, char) {
            (0, 'v') => {}
            (_, 'v') => { if last_char == '&' { end+=1 } break } //final v reached
            (_, '&') => {}
            _ => { break }
        }

        end += 1;
        last_char = *char;
    }

    let prev: String = chars[..end].iter().collect();
    let type_fncy: String = chars[end..].iter().collect();

    (prev, type_fncy)
}

pub fn get_type_rs(type_fncy: &String) -> String {
    let le_map = get_param_type_hashmap();
    let type_rs = le_map.get(type_fncy).unwrap_or_else(|| { println!("didn't get a proper type! {}", type_fncy); type_fncy }).to_string();

    type_rs
}
