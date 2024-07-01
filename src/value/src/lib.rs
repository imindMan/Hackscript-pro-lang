// INFO: Value definition, which is another enum

pub mod number;
use error_handling::Error;
use lexer::Token;
use position::Position;

pub enum Value {
    Number(number::Number),
    Nil,
}

impl Value {
    // by default
    pub fn new() -> Value {
        Value::Nil
    }
    pub fn new_number(
        sign: String,
        identifier: String,
        value: String,
        pos_start: Position,
        pos_end: Position,
    ) -> Value {
        Value::Number(number::Number::new(
            sign, identifier, value, pos_start, pos_end,
        ))
    }
}
