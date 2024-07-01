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
    pub fn new(&self) -> Value {
        Value::Nil
    }
    pub fn new_number(
        &self,
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

    fn create_error(
        &self,
        r#type: String,
        extra_string: String,
        pos_start: Position,
        pos_end: Position,
    ) -> (Option<Value>, Option<Error>) {
        let value: Option<Value> = None;
        let error: Option<Error> = Some(Error::new(r#type, extra_string, pos_start, pos_end));
        (value, error)
    }
}
