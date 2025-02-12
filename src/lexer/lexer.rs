// use std::env;
use std::{char, fs};
use crate::util::types::{self, GroupingSymbol};


fn get_splits_parentheses(text: &String) -> Vec<&str> {
    let split_text: Vec<&str> = text.split(['(', ')']).collect();
    return split_text
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
        _ => ()
    }
}

fn iterate_over_text() -> types::Idx {
    let text = read_sample_file();
    let mut idx = types::Idx::default();

    for (i, char) in text.chars().enumerate() {
        handle_match(&mut idx, &char, i);
    }

    // for open_idx in &idx.grouping_symbols.parentheses.open {
    //     for (i, char) in text.chars().enumerate() {
    //         if &i == open_idx {
    //             println!("i: {i} char: {char} open idx: {open_idx}");
    //         }
    //     }
    // }

    idx
}

pub fn main() {
    // start_lexing();
    iterate_over_text();
}
