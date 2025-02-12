// use std::env;
use std::{char, fs};
use crate::util::types::{self, GroupingSymbol, Symbols};


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

fn get_symbol_idx_from_text(text: &String) -> types::Idx {
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
    let text = read_sample_file();
    let idx = get_symbol_idx_from_text(&text);

    for colon in idx.symbols.colon {
        println!("dis a colon: {colon}");
    }
}
