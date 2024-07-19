use std::env;

mod ast;
mod lexer;
mod error;
mod parser;
mod interpreter;



fn main() -> Result<(), Box<dyn std::error::Error>> {
    clearscreen::clear().expect("Aww, We failed to clear your screen ");
    let args: Vec<String> = env::args().collect();

    if args

}
