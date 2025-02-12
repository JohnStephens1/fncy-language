// use std::env;
use std::{char, fs, path::Path};
use crate::util::types;


fn handle_match(idx: &mut types::Idx, char: &char, i: usize) {
    // for now back to chonky match
    match char {
        '(' => idx.grouping_symbols.parentheses.open.push(i),
        ')' => idx.grouping_symbols.parentheses.closed.push(i),
        '[' => idx.grouping_symbols.brackets.open.push(i),
        ']' => idx.grouping_symbols.brackets.closed.push(i),
        '{' => idx.grouping_symbols.braces.open.push(i),
        '}' => idx.grouping_symbols.braces.closed.push(i),
        '<' => idx.grouping_symbols.angle_brackets.open.push(i),
        '>' => idx.grouping_symbols.angle_brackets.closed.push(i),


        '_' => idx.symbols.underscore.push(i),
        
        '-' => idx.symbols.minus.push(i),
        '+' => idx.symbols.plus.push(i),
        '*' => idx.symbols.asterisk.push(i),
        '/' => idx.symbols.slash.push(i),
        '\\' => idx.symbols.backslash.push(i),

        '=' => idx.symbols.equal.push(i),

        '&' => idx.symbols.and.push(i),
        '|' => idx.symbols.or.push(i),
        '!' => idx.symbols.exclamation_mark.push(i),
        '?' => idx.symbols.question_mark.push(i),

        '~' => idx.symbols.tilde.push(i),


        ',' => idx.symbols.comma.push(i),
        ';' => idx.symbols.semicolon.push(i),
        '.' => idx.symbols.dot.push(i),
        ':' => idx.symbols.colon.push(i),

        '\'' => idx.symbols.apostrophe.push(i),
        '"' => idx.symbols.quotation_mark.push(i),

        '´' => idx.symbols.forward_tick.push(i),
        '`' => idx.symbols.back_tick.push(i),


        '$' => idx.symbols.dollar.push(i),
        '%' => idx.symbols.percent.push(i),
        '^' => idx.symbols.caret.push(i),
        '°' => idx.symbols.degree.push(i),

        '#' => idx.symbols.hash.push(i),
        '@' => idx.symbols.at.push(i),
        _ => ()
    }
}


fn read_file(path: &Path) -> String{
    let contents: String = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    contents
}

fn normalize_whitespaces(chars: Vec<char>) -> Vec<char> {
    let mut prev_char_was_whitespace = false;
    let mut result: Vec<char> = Vec::new();

    for char in chars {
        if char.is_whitespace() {
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


fn get_symbol_idx_from_char_vec(text: &Vec<char>) -> types::Idx {
    let mut idx = types::Idx::default();

    for (i, char) in text.iter().enumerate() {
        handle_match(&mut idx, &char, i);
    }

    idx
}

pub fn get_symbol_idx_and_chars_from_file(path: &Path) -> (types::Idx, Vec<char>) {
    let text = get_processed_text(path);
    let symbol_idx = get_symbol_idx_from_char_vec(&text);

    (symbol_idx, text)
}


pub fn main() {
    let sample_file_path = Path::new("src/sample_files/first_example_file.fncy");
    let (symbol_idx, text_chars) = get_symbol_idx_and_chars_from_file(sample_file_path);

    test_char_shenanigans(&text_chars, &symbol_idx);
}




fn test_char_shenanigans(text_chars: &Vec<char>, symbol_idx: &types::Idx) {
    let mut i = 0;

    for char in text_chars {
        println!("loop number: {i}");
        println!("text_char: {char}");
        println!("text_chars[my_int]: {}", text_chars[i]);

        if symbol_idx.symbols.colon.contains(&i) {
            println!("SYMBOL COLON FOUND AT {i}");
        }

        println!();
        i += 1;
    }
}
