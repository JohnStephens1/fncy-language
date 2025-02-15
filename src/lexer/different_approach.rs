use std::{collections::HashMap, fs, path::Path};



fn get_symbols() -> [String; 19] {
    [
        "(", ")",
        "{", "}",
        "[", "]",
        "<", ">",
        ",", ";",
        ".", ":",
        "+", "-",
        "*", "/",
        "=",
        "\"", "'",

    ].map(ToString::to_string)
}

fn read_file(path: &Path) -> String{
    let contents: String = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    contents
}

fn normalize_whitespaces(chars: Vec<char>) -> Vec<char> {
    let mut prev_char_was_whitespace = false;
    let mut result: Vec<char> = Vec::new();
    // println!("hello");

    for char in chars {
        // if char == '\n' {
        //     println!("newline {char}");
        // }
        if char.is_whitespace() {
            // if char.is_whitespace() && !(char == '\n') {
            if !prev_char_was_whitespace {
                result.push(' ');
                prev_char_was_whitespace = true;
            }
        } else {
            result.push(char);
            prev_char_was_whitespace = false;
        }
    }

    result
}

fn get_processed_text(path: &Path) -> Vec<char> {
    let text_raw = read_file(path);
    let text_chars_raw: Vec<char> = text_raw.chars().collect();
    let text_chars = normalize_whitespaces(text_chars_raw);

    text_chars
}

fn chars_to_string_vec(chars: Vec<char>) -> Vec<String> {
    let string: String = chars.into_iter().collect();

    string.split_whitespace().map(String::from).collect()
}

fn reduce_whitespaces_from_string(string: &String) -> String {
    let words: Vec<_> = string.split_whitespace().collect();
    words.join(" ")
}

fn spacify_character(string: &String, character: &String) -> String{
    // don't quite work for in strings
    let char_with_spaces: String = format!(" {character} ");

    string.replace(character, &char_with_spaces)
}

fn update_string(original_string: String) -> String {
    let symbols = get_symbols();
    let mut string = original_string;

    // let le_map: HashMap<String, String> = vec![
    //     ("- >", "->"),
    // ]
    // .into_iter()
    // .map(|x| (String::from(x.0), String::from(x.1)))
    // .collect();

    for symbol in symbols {
        // println!("symbol: {symbol}");
        if symbol == "->" { continue; }
        string = spacify_character(&string, &symbol);
    }

    string = string.replace("-  >", "->");

    string = reduce_whitespaces_from_string(&string);

    // let char_vec: Vec<char> = string.chars().collect();
    // string = normalize_whitespaces(char_vec).into_iter().collect();

    string
}


pub fn main() -> String{
    let sample_file_path = Path::new("src/sample_files/first_example_file.fncy");
    let original_text: String = read_file(sample_file_path);

    let updated_text = update_string(original_text);

    // println!("updated_text: {updated_text}");

    updated_text

    // let split_boi: Vec<String> = trimmed_text.split_inclusive("(").map(String::from).collect();
}
