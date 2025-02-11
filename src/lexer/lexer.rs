// use std::env;
use std::fs;


// fn get_quote_splits(string: String) -> Vec<&str> {
//     let parts: Vec<&str> = string.split('"').collect();
//     return parts
// }


fn get_splits_parentheses(text: &String) -> Vec<&str> {
    let split_text: Vec<&str> = text.split(['(', ')']).collect();
    return split_text
}


fn lexing_logic(raw_text: String) {
    let raw_text_copy: String = raw_text.clone();

    let split: Vec<&str> = get_splits_parentheses(&raw_text);

    for (i, piece) in split.iter().enumerate() {
        if i % 2 == 0 {
            println!("not in parentheses:");
        } else {
            println!("in parentheses:");
        }
        println!("{piece}\n");
    }
}

fn get_character_iterator(text: &String) -> std::str::Chars {
    text.chars()
}


struct CharacterIdx {
    paranthesis_open_idx: Vec<usize>,
    paranthesis_close_idx: Vec<usize>,
    curly_open_idx: Vec<usize>,
    curly_close_idx: Vec<usize>,
    bracket_open_idx: Vec<usize>,
    bracket_close_idx: Vec<usize>,
    angle_bracket_open_idx: Vec<usize>,
    angle_bracket_close_idx: Vec<usize>,
}


fn something_loop(text: &String) -> (Vec<usize>, Vec<usize>) {
    let mut paranthesis_open_idx: Vec<usize> = Vec::new();
    let mut paranthesis_close_idx: Vec<usize> = Vec::new();
    let mut curly_open_idx: Vec<usize> = Vec::new();
    let mut curly_close_idx: Vec<usize> = Vec::new();
    let mut bracket_open_idx: Vec<usize> = Vec::new();
    let mut bracket_close_idx: Vec<usize> = Vec::new();
    let mut angle_bracket_open_idx: Vec<usize> = Vec::new();
    let mut angle_bracket_close_idx: Vec<usize> = Vec::new();

    for (i, char) in get_character_iterator(text).enumerate() {
        println!("{i}: {char}");
        match char {
            '(' => paranthesis_open_idx.push(i),
            ')' => paranthesis_close_idx.push(i),
            '{' => curly_open_idx.push(i),
            '}' => curly_close_idx.push(i),
            '[' => bracket_open_idx.push(i),
            ']' => bracket_close_idx.push(i),
            '<' => angle_bracket_open_idx.push(i),
            '>' => angle_bracket_close_idx.push(i),
            _ => (),
        }
    }

    return (paranthesis_open_idx, paranthesis_close_idx)
}

fn print_character_for_idx(text: &String, idx_vec: &Vec<usize>) {
    for idx in idx_vec {
        println!("idx: {idx}\nchar at idx: {}", text.chars().nth(*idx).unwrap());
    }
}


fn read_sample_file() -> String{
    let contents: String = fs::read_to_string( "src/sample_files/first_example_file.fncy")
        .expect("Should have been able to read the file");
    contents
}

fn print_sample_file() {
    let content: String = read_sample_file();
    println!("file content:\n\n{content}");
}

fn start_lexing() {
    let raw_text: String = read_sample_file();
    // lexing_logic(raw_text);
    let (paranthesis_open_idx, paranthesis_close_idx) = something_loop(&raw_text);
    print_character_for_idx(&raw_text, &paranthesis_open_idx);
    print_character_for_idx(&raw_text, &paranthesis_close_idx);
}

pub fn main() {
    start_lexing();
}
