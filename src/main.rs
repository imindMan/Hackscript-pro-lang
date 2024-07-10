// INFO: This is the Hackscript rebuild project written by imindMan
// Due to slow speed of Hackscript in Python, Hackscript will be recreated again.
// Author: imindMan
// Date: 12-02-2023 08:35 PM (GMT +7)
// Description: all the description you can find in the official repo
// https://github.com/imindMan/Hackscript-pro-lang
// Rebuild in Rust

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;
use std::io::{self, Write};

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
        run(command);
        //print!("{}", output);
    }
}

// run the command
fn run(command: String) {
    // Lexing
    let mut lexer = Lexer::new(String::from("stdin"), command);
    let (tokens, error) = lexer.make_tokens();
    if error.is_some() {
        print!(
            "HackScript detected some error(s): \n{} \n",
            error.unwrap().error_message()
        );
    } else {
        // Parsing
        let mut parser = Parser::new(tokens.unwrap());
        let (ast, err) = parser.parse();
        if err.is_some() {
            print!(
                "HackScript detected some error(s): \n{} \n",
                err.unwrap().error_message()
            );
        } else {
            let interpreter = Interpreter::new(ast.unwrap().clone());
            let (value, err_final) = interpreter.interpret();

            if err_final.is_some() {
                print!(
                    "HackScript detected some error(s): \n{} \n",
                    err_final.unwrap().error_message()
                );
            } else {
                println!("{:#?}", value);
            }
        }
    }
}
