// INFO: This is the Hackscript rebuild project written by imindMan
// Due to slow speed of Hackscript in Python, Hackscript will be recreated again.
// Author: imindMan
// Date: 12-02-2023 08:35 PM (GMT +7)
// Description: all the description you can find in the official repo
// https://github.com/imindMan/Hackscript-pro-lang
// Rebuild in Rust

pub mod ast_implementation;
pub mod error_handling;
pub mod hacktypes;
pub mod interpreter;
pub mod position;
pub mod value;
use ast_implementation::Lexer;
use ast_implementation::Parser;
use error_handling::Error;
use interpreter::Interpreter;
use std::io::{self, Write};
use value::Value;

// INFO: Main function
// For now, this function is going to take user's inputs then print the result out,
// just like Python.

fn main() -> Result<(), io::Error> {
    loop {
        let mut command: String = String::new();

        print!("hackscript>");

        // input
        let _ = io::stdout().flush();
        io::stdin()
            .read_line(&mut command)
            .expect("Error reading from STDIN");

        match run(command) {
            Ok(ok) => println!("{}", ok),
            Err(err) => println!("{}", err),
        }
    }
}

// INFO: Main entry point of the Hackscript programming language.
pub fn run(command: String) -> Result<Value, Error> {
    // Lexing part
    let mut lexer = Lexer::new(String::from("stdin"), command);
    let tokens = lexer.make_tokens()?;
    // Parsing part
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;
    // Interpreting part
    let interpreter = Interpreter::new(ast);
    let value = interpreter.interpret()?;

    Ok(value)
}
