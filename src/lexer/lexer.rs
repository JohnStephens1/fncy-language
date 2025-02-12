// use std::env;
use std::path::Path;
use crate::lexer::preprocessing;
use crate::lexer::tokenizer;



pub fn main() {
    let sample_file_path = Path::new("src/sample_files/first_example_file.fncy");
    let (symbol_idx, text_chars) = preprocessing::get_symbol_idx_and_chars_from_file(sample_file_path);

    tokenizer::main(&text_chars, &symbol_idx);
    // preprocessing::test_char_shenanigans(&text_chars, &symbol_idx);
}
