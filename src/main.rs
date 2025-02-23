#![allow(dead_code)]
#![allow(unused_variables)]
#![warn(
    clippy::pedantic
)]
// #![allow(unused_imports)]

mod lexer;
mod parser;
mod util;
mod textifier;
mod types;


fn main() {
    // println!("main");
    // config::print_config();
    // lexer::lexer::main();

    parser::parser::main();

    // parser::tests::main();
    // util::types::main();
}
