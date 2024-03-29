// INFO: This is the Hackscript rebuild project written by imindMan
// Due to slow speed of Hackscript in Python, Hackscript will be recreated again.
// Author: imindMan
// Date: 12-02-2023 08:35 PM (GMT +7)
// Description: all the description you can find in the official repo
// https://github.com/imindMan/Hackscript-pro-lang
// Rebuild in Rust

// main file hackscript

use lexer::Lexer;
use std::io::{self, Write};
// input a command then run Hackscript

fn main() -> Result<(), io::Error> {
    loop {
        // create a string to store command
        let mut command = String::new();

        print!("hackscript>");

        // input a command
        let _ = io::stdout().flush();
        io::stdin()
            .read_line(&mut command)
            .expect("Error reading from STDIN");
        let output = run(command);
        //print!("{}", output);
    }
}

// run the command
fn run(command: String) {
    // start lexing
    let mut lexer = Lexer::new(String::from("stdin").clone(), command.clone());
    let (tokens, error) = lexer.make_tokens();
    if error.is_some() {
        print!(
            "HackScript detected some error(s): \n{} \n",
            error.unwrap().error_message
        );
    } else {
        print!("{:#?}", tokens);
    }
}
