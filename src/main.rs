#![allow(dead_code)]
#![allow(unused_variables)]
#![warn(
    clippy::pedantic
)]
// #![allow(unused_imports)]

mod config;
mod lexer;
mod parser;
mod util;


fn main() {
    // println!("main");
    // config::print_config();
    lexer::lexer::main();
    // parser::parser::main();
    // util::types::main();
}
