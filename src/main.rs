use std::{env, fs};

mod ast;
mod lexer;
mod error;
mod parser;
mod interpreter;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::interpreter::Interpreter;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    clearscreen::clear().expect("Aww, We failed to clear your screen ");
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let filename = &args[1];
        let content = fs::read_to_string(filename)?;
        run(content)
    }
    else {
        repl()
    }

}

fn run(source: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize()?;
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;
    
    let interpreter = Interpreter::new();
    let result = interpreter.interpret(&ast)?;
    
    println!("Result: {}", result);
    Ok(())
}
