// use std::env;
use std::fs;


fn read_file() -> String{
    let contents: String = fs::read_to_string( "src/sample_files/first_example_file.fncy")
        .expect("Should have been able to read the file");
    contents
}

pub fn main() {
    let sample_file_content: String  = read_file();

    println!("file content:\n\n{sample_file_content}");
}