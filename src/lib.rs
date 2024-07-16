// INFO: This is the Hackscript rebuild project written by imindMan
// Due to slow speed of Hackscript in Python, Hackscript will be recreated again.
// Author: imindMan
// Date: 12-02-2023 08:35 PM (GMT +7)
// Description: all the description you can find in the official repo
// https://github.com/imindMan/Hackscript-pro-lang
// Rebuild in Rust

use error_handling::Error;
use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;
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
            Ok(ok) => print!("{}", ok),
            Err(err) => print!("{}", err),
        }
    }
}
// run the command
pub fn run(command: String) -> Result<Value, Error> {
    // Lexing
    let mut lexer = Lexer::new(String::from("stdin"), command);
    let (tokens, error_lexer) = lexer.make_tokens();
    if let Some(..) = error_lexer {
        Err(error_lexer.unwrap())
    } else {
        // Parsing
        let mut parser = Parser::new(tokens.unwrap());
        let (ast, error_parser) = parser.parse();
        if let Some(..) = error_parser {
            Err(error_parser.unwrap())
        } else {
            println!("{:#?}", ast);
            let interpreter = Interpreter::new(ast.unwrap());
            let (value, error_interpreter) = interpreter.interpret();

            if let Some(..) = error_interpreter {
                Err(error_interpreter.unwrap())
            } else {
                Ok(value.unwrap())
            }
        }
    }
}
