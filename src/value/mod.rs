// INFO: This is the Value enum, which represents the values in Hackscript language.
// Because Hackscript is an interpreted language meaning there's no distinction between data types.

pub mod boolean;
pub mod null;
pub mod number;
pub mod string;
pub mod value_trait;
use crate::error_handling::Error;
use crate::hacktypes::*;
use crate::position::Position;
use std::fmt::Display;
use value_trait::ValueTrait;

#[derive(Debug, Clone)]
pub enum Value {
    Number(number::Number),
    String(string::HackString),
    Boolean(boolean::Boolean),
    Null(null::Null),
    Nil,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(number) => write!(f, "{}", number),
            Value::String(string) => write!(f, "{}", string),
            Value::Boolean(bool) => write!(f, "{}", bool),
            Value::Null(_null) => write!(f, "null"),
            Value::Nil => write!(f, ""),
        }
    }
}
impl ValueTrait for Value {
    fn get_pos_start(&self) -> Position {
        match self {
            Value::Number(number::Number {
                value: _,
                identifier: _,
                pos_start,
                pos_end: _,
            }) => pos_start.clone(),
            Value::String(string::HackString {
                value: _,
                pos_start,
                pos_end: _,
            }) => pos_start.clone(),
            Value::Boolean(boolean::Boolean {
                boolean: _,
                pos_start,
                pos_end: _,
            }) => pos_start.clone(),
            Value::Null(null::Null {
                value: _,
                pos_start,
                pos_end: _,
            }) => pos_start.clone(),
            _ => panic!("Invalid operation"),
        }
    }
    fn type_generate_error(&self, value: Value) -> (Option<Value>, Option<Error>) {
        let pos_start: Position = match self {
            Value::Number(number::Number {
                value: _,
                identifier: _,
                pos_start,
                pos_end: _,
            }) => pos_start.clone(),
            Value::String(string::HackString {
                value: _,
                pos_start,
                pos_end: _,
            }) => pos_start.clone(),
            Value::Boolean(boolean::Boolean {
                boolean: _,
                pos_start,
                pos_end: _,
            }) => pos_start.clone(),
            Value::Null(null::Null {
                value: _,
                pos_start,
                pos_end: _,
            }) => pos_start.clone(),
            _ => panic!("Invalid operation"),
        };
        let pos_end: Position = match value {
            Value::Number(number::Number {
                value: _,
                identifier: _,
                pos_start: _,
                pos_end,
            }) => pos_end.clone(),
            Value::String(string::HackString {
                value: _,
                pos_start: _,
                pos_end,
            }) => pos_end.clone(),
            Value::Boolean(boolean::Boolean {
                boolean: _,
                pos_start: _,
                pos_end,
            }) => pos_end.clone(),
            Value::Null(null::Null {
                value: _,
                pos_start: _,
                pos_end,
            }) => pos_end.clone(),
            _ => panic!("Invalid operation"),
        };

        self.generate_error(
            "TypeError".to_string(),
            "Invalid types for such an operation".to_string(),
            pos_start,
            pos_end,
        )
    }
    // INFO: All of the operation below are substances of the arithmetic_operating function

    // INFO: This function performs plus operation
    // Note that every single data type value (as soon they can support plus method) can
    // universally use this function to perform the plus operation
    fn add_to(&self, value: Value) -> (Option<Value>, Option<Error>) {
        self.operation(value, PLUS)
    }
    // INFO: This function performs minus operation
    // Note that every single data type value (as soon they can support minus method) can
    // universally use this function to perform the minus operation

    fn subtract_to(&self, value: Value) -> (Option<Value>, Option<Error>) {
        self.operation(value, MINUS)
    }
    // INFO: This function performs multiply operation
    // Note that every single data type value (as soon they can support multiply method) can
    // universally use this function to perform the multiply operation

    fn multiply_by(&self, value: Value) -> (Option<Value>, Option<Error>) {
        self.operation(value, MULTIPLY)
    }
    // INFO: This function performs divide operation
    // Note that every single data type value (as soon they can support divide method) can
    // universally use this function to perform the divide operation

    fn divide_by(&self, value: Value) -> (Option<Value>, Option<Error>) {
        self.operation(value, DIVIDE)
    }

    fn greater(&self, value: Value) -> (Option<Value>, Option<Error>) {
        self.operation(value, GREATER)
    }
    fn greater_or_equal(&self, value: Value) -> (Option<Value>, Option<Error>) {
        self.operation(value, GREATER_OR_EQUAL)
    }
    fn less(&self, value: Value) -> (Option<Value>, Option<Error>) {
        self.operation(value, LESS)
    }
    fn less_or_equal(&self, value: Value) -> (Option<Value>, Option<Error>) {
        self.operation(value, LESS_OR_EQUAL)
    }
    fn equal(&self, value: Value) -> (Option<Value>, Option<Error>) {
        self.operation(value, EQUAL)
    }
    fn not_equal(&self, value: Value) -> (Option<Value>, Option<Error>) {
        self.operation(value, NOT_EQUAL)
    }
    fn and(&self, value: Value) -> (Option<Value>, Option<Error>) {
        self.operation(value, AND)
    }
    fn or(&self, value: Value) -> (Option<Value>, Option<Error>) {
        self.operation(value, OR)
    }
}

impl Value {
    // INFO: This is the default Value initialization, which will return the Nil value
    pub fn new() -> Value {
        Value::Nil
    }

    // INFO: This is the initialization method for the Number
    pub fn new_number(
        identifier: String,
        value: String,
        pos_start: Position,
        pos_end: Position,
    ) -> Value {
        Value::Number(number::Number::new(identifier, value, pos_start, pos_end))
    }
    pub fn new_string(value: String, pos_start: Position, pos_end: Position) -> Value {
        Value::String(string::HackString::new(value, pos_start, pos_end))
    }

    pub fn new_boolean(value: bool, pos_start: Position, pos_end: Position) -> Value {
        Value::Boolean(boolean::Boolean::new(value, pos_start, pos_end))
    }
    pub fn new_null(value: String, pos_start: Position, pos_end: Position) -> Value {
        Value::Null(null::Null::new(value, pos_start, pos_end))
    }
    fn handling_operation<T: ValueTrait>(
        &self,
        value_origin: T,
        value_other: Value,
        instruction: &str,
    ) -> (Option<Value>, Option<Error>) {
        match instruction {
            PLUS => value_origin.add_to(value_other.clone()),
            MINUS => value_origin.subtract_to(value_other.clone()),
            MULTIPLY => value_origin.multiply_by(value_other.clone()),
            DIVIDE => value_origin.divide_by(value_other.clone()),

            GREATER => value_origin.greater(value_other.clone()),
            GREATER_OR_EQUAL => value_origin.greater_or_equal(value_other.clone()),

            LESS => value_origin.less(value_other.clone()),
            LESS_OR_EQUAL => value_origin.less_or_equal(value_other.clone()),
            EQUAL => value_origin.equal(value_other.clone()),
            NOT_EQUAL => value_origin.not_equal(value_other.clone()),
            AND => value_origin.and(value_other.clone()),
            OR => value_origin.or(value_other.clone()),
            _ => {
                return self.generate_error(
                    "TypeError".to_string(),
                    "Invalid types for such an operation".to_string(),
                    self.get_pos_start(),
                    self.get_pos_end(value_other),
                )
            }
        }
    }
    fn operation(&self, value: Value, instruction: &str) -> (Option<Value>, Option<Error>) {
        match self {
            Value::Number(value_origin) => {
                self.handling_operation(value_origin.clone(), value, instruction)
            }
            Value::String(value_origin) => {
                self.handling_operation(value_origin.clone(), value, instruction)
            }
            Value::Boolean(value_origin) => {
                self.handling_operation(value_origin.clone(), value, instruction)
            }
            Value::Null(value_origin) => {
                self.handling_operation(value_origin.clone(), value, instruction)
            }
            _ => return self.type_generate_error(value),
        }
    }
}
