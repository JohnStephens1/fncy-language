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
    lexing_logic(raw_text);
}

pub fn main() {
    start_lexing();
}
