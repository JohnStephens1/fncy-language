mod config;
mod lexer;

fn main() {
    println!("main");
    config::print_config();
    lexer::lexer::main();
}
