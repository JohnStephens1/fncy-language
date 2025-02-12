use crate::util::types;
use crate::util::tokens;



fn shenanigans() {
    let my_string = "potato nuggit";
    let slice = &my_string[0..5];
    println!("slice: {slice}");

    my_string.find(slice);
    // my_string.split_whitespace()
}

fn check_keywords() {
    // let tokens = tokens::Tokens;
}

fn tokenize(chars: &Vec<char>, symbol_idx: &types::Idx) {

}

pub fn main(chars: &Vec<char>, symbol_idx: &types::Idx) {
    // tokenize(chars, symbol_idx);
    // check_keywords();
    shenanigans();
}