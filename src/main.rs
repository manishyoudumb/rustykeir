use std::{env, fs};

mod ast;
mod lexer;
mod error;
mod parser;
mod interpreter;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::interpreter::Interpreter;
use rustyline::error::ReadlineError;
use crate::error::KeirError;
use rustyline::Editor;


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


fn repl() -> Result<(), Box<dyn std::error::Error>> {
    let mut rl = Editor::<()>::new();
    let interpreter = Interpreter::new();

println!("|---------------------------------------------------------------------|");
println!("| Welcome to Keir Lang.                                               |");
println!("|---------------------------------------------------------------------|");
println!("| Type 'exit' or 'quit' to leave.                                     |");
println!("|---------------------------------------------------------------------|");
println!("|ᵛᵉʳˢⁱᵒⁿ 0.1.o                                   ᵃᵘᵗʰᵒʳ: manishyoudumb|");
println!("|---------------------------------------------------------------------|");

    loop {
        let readline = rl.readline("admin@keir->> ");
        match readline {
            Ok(line) => {
                if line.trim() == "exit" || line.trim() == "quit" {
                    break;
                }
                rl.add_history_entry(line.as_str());
                match eval(&line, &interpreter) {
                    Ok(result) => println!("Output is : {}", result),
                    Err(e) => eprintln!("Error : {}", e),

                }
                
            },
            
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            },

            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            },
            
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }


        }
    }
    println!("Miss me and be back soon!");
    Ok(())
}




