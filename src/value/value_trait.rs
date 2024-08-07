use crate::error_handling::Error;
use crate::position::Position;
use crate::value::boolean::Boolean;
use crate::value::null::Null;
use crate::value::number::Number;
use crate::value::string::HackString;
use crate::Value;

pub trait ValueTrait {
    fn get_pos_end(&self, value: Value) -> Position {
        match value {
            Value::Number(Number {
                value: _,
                identifier: _,
                pos_start: _,
                pos_end,
            }) => pos_end.clone(),
            Value::String(HackString {
                value: _,
                pos_start: _,
                pos_end,
            }) => pos_end.clone(),
            Value::Boolean(Boolean {
                value: _,
                pos_start: _,
                pos_end,
            }) => pos_end.clone(),
            Value::Null(Null {
                value: _,
                pos_start: _,
                pos_end,
            }) => pos_end.clone(),
            _ => panic!("Invalid operation"),
        }
    }
    fn get_pos_start(&self) -> Position;
    fn type_generate_error(&self, value: Value) -> Result<Value, Error> {
        let pos_start: Position = self.get_pos_start();
        let pos_end: Position = self.get_pos_end(value);
        Err(Error::new(
            "TypeError".to_string(),
            "Invalid types for such an operation".to_string(),
            pos_start,
            pos_end,
        ))
    }
    fn add_to(&self, value: Value) -> Result<Value, Error> {
        self.type_generate_error(value)
    }
    fn subtract_to(&self, value: Value) -> Result<Value, Error> {
        self.type_generate_error(value)
    }
    fn multiply_by(&self, value: Value) -> Result<Value, Error> {
        self.type_generate_error(value)
    }
    fn divide_by(&self, value: Value) -> Result<Value, Error> {
        self.type_generate_error(value)
    }
    fn greater(&self, value: Value) -> Result<Value, Error> {
        self.type_generate_error(value)
    }
    fn greater_or_equal(&self, value: Value) -> Result<Value, Error> {
        self.type_generate_error(value)
    }
    fn less(&self, value: Value) -> Result<Value, Error> {
        self.type_generate_error(value)
    }
    fn less_or_equal(&self, value: Value) -> Result<Value, Error> {
        self.type_generate_error(value)
    }
    fn equal(&self, value: Value) -> Result<Value, Error> {
        self.type_generate_error(value)
    }
    fn not_equal(&self, value: Value) -> Result<Value, Error> {
        self.type_generate_error(value)
    }
    fn and(&self, value: Value) -> Result<Value, Error> {
        self.type_generate_error(value)
    }
    fn or(&self, value: Value) -> Result<Value, Error> {
        self.type_generate_error(value)
    }

    fn append(&mut self, value: Value) -> Result<Value, Error> {
        self.type_generate_error(value)
    }
    fn indexing(&self, value: Value) -> Result<Value, Error> {
        self.type_generate_error(value)
    }
}
