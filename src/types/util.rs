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

// currently behaves like this:
// & value -> &v alue
// & Value -> & Value
pub fn split_type_fncy_raw(string: &str) -> (String, String) {
    let chars: Vec<char> = string.chars().filter(|x| !x.is_whitespace()).collect();

    let mut end = 0;
    let mut last_char = ' ';

    for (i, char) in chars.iter().enumerate() {
        match (i, char) {
            (0, 'v') => {}
            (_, 'v') => { if last_char == '&' { end+=1 } break }
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
    if type_fncy.is_empty() { return "".to_string() }

    let le_map = get_param_type_hashmap();
    let type_rs = le_map.get(type_fncy).unwrap_or_else(|| { println!("didn't get a proper type! {}", type_fncy); type_fncy }).to_string();

    type_rs
}


pub fn get_type_name_rs_string(var_name: &String, type_rs_isolated: &String, var_info: &super::var::VarInfo) -> String {
    if type_rs_isolated.is_empty() { return "".to_string() }

    format!(
        "{}{}: {}",
        if (var_info.is_ref && var_info.is_var_ref) || (!var_info.is_ref && var_info.is_var) { "mut " } else { "" },
        var_name,
        get_type_rs_string(type_rs_isolated, var_info)
    )
}

pub fn get_type_rs_string(type_rs_isolated: &String, var_info: &super::var::VarInfo) -> String {
    if type_rs_isolated.is_empty() { return "".to_string() }
    
    format!(
        "{}{}{}",
        std::iter::repeat("&").take(var_info.ref_count).collect::<String>(),
        if var_info.is_ref && var_info.is_var { "mut " } else { "" },
        type_rs_isolated
    )
}


pub fn process_type_fncy(type_fncy_raw: &String) -> (String, String, String, super::var::VarInfo) {
    let (prev, type_fncy_isolated) = split_type_fncy_raw(&type_fncy_raw);
    let type_rs_isolated = if type_fncy_isolated.is_empty() { "".to_string() } else { get_type_rs(&type_fncy_isolated) };
    let var_info = super::var::VarInfo::new(&prev);
    let type_rs_string = get_type_rs_string(&type_rs_isolated, &var_info);

    (type_fncy_isolated, type_rs_isolated, type_rs_string, var_info)
}
