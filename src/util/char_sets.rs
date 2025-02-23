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
