use crate::ast_implementation::Lexer;
use crate::ast_implementation::Parser;
use crate::error_handling::Error;
use crate::interpreter::Interpreter;
use crate::value::Value;

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
