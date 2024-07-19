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

fn eval(input: &str, interpreter: &Interpreter) -> Result<f64, KeirError> {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize()?;
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;
    
    interpreter.interpret(&ast)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_addition() {
        let interpreter = Interpreter::new();
        let result = eval("2 + 3", &interpreter).unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_eval_subtraction() {
        let interpreter = Interpreter::new();
        let result = eval("10 - 4", &interpreter).unwrap();
        assert_eq!(result, 6.0);
    }

    #[test]
    fn test_eval_multiplication() {
        let interpreter = Interpreter::new();
        let result = eval("3 * 4", &interpreter).unwrap();
        assert_eq!(result, 12.0);
    }

    #[test]
    fn test_eval_division() {
        let interpreter = Interpreter::new();
        let result = eval("15 / 3", &interpreter).unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_eval_complex_expression() {
        let interpreter = Interpreter::new();
        let result = eval("(2 + 3) * 4 - 6 / 2", &interpreter).unwrap();
        assert_eq!(result, 17.0);
    }

    #[test]
    fn test_eval_unary_minus() {
        let interpreter = Interpreter::new();
        let result = eval("-5 + 3", &interpreter).unwrap();
        assert_eq!(result, -2.0);
    }

    #[test]
    #[should_panic(expected = "DivisionByZero")]
    fn test_eval_division_by_zero() {
        let interpreter = Interpreter::new();
        eval("10 / 0", &interpreter).unwrap();
    }

    #[test]
    #[should_panic(expected = "InvalidCharacter")]
    fn test_eval_invalid_character() {
        let interpreter = Interpreter::new();
        eval("2 + @", &interpreter).unwrap();
    }

    #[test]
    fn test_eval_nested_parentheses() {
        let interpreter = Interpreter::new();
        let result = eval("(3 + 2) / 5", &interpreter).unwrap();
        assert_eq!(result, 1.0, "Wow, you can handle nested parentheses. I'm so impressed.");
    }

    #[test]
    fn test_eval_floating_point() {
        let interpreter = Interpreter::new();
        let result = eval("3.14159 * 2", &interpreter).unwrap();
        assert!(result > 6.28 && result < 6.29, "Oh look, it can do decimals. How revolutionary.");
    }

    #[test]
    fn test_eval_whitespace_madness() {
        let interpreter = Interpreter::new();
        let result = eval("  1   +    2    *         3   ", &interpreter).unwrap();
        assert_eq!(result, 7.0, "Spaces, the final frontier. You've conquered them. Golf clap.");
    }

    #[test]
    fn test_eval_unary_minus_chain() {
        let interpreter = Interpreter::new();
        let result = eval("----5", &interpreter).unwrap();
        assert_eq!(result, 5.0, "Minus, minus, minus, minus. It's like you're trying to bore me to death.");
    }

    #[test]
    #[should_panic(expected = "InvalidNumber")]
    fn test_eval_multiple_decimal_points() {
        let interpreter = Interpreter::new();
        eval("3.14.15", &interpreter).unwrap();
        // This comment is just to say: "You've successfully confused a computer. Achievement unlocked!"
    }
}
