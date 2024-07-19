use std::{env, fs};

mod ast;
mod lexer;
mod error;
mod parser;
mod interpreter;




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

