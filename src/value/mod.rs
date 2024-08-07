// INFO: This is the Value enum, which represents the values in Hackscript language.
// Because Hackscript is an interpreted language meaning there's no distinction between data types.
mod array;
mod boolean;
mod null;
mod number;
mod set;
mod string;
mod tuple;
pub mod value_trait;
use crate::error_handling::Error;
use crate::hacktypes::*;
use crate::position::Position;
use std::fmt::Display;
use value_trait::ValueTrait;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value {
    Number(number::Number),
    String(string::HackString),
    Boolean(boolean::Boolean),
    Null(null::Null),
    Tuple(tuple::Tuple),
    Set(set::Set),
    Array(array::Array),
    Nil,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(number) => write!(f, "{}", number),
            Value::String(string) => write!(f, "{}", string),
            Value::Boolean(bool) => write!(f, "{}", bool),
            Value::Null(_null) => write!(f, "null"),
            Value::Tuple(tuple) => write!(f, "{}", tuple),
            Value::Set(set) => write!(f, "{}", set),
            Value::Array(array) => write!(f, "{}", array),
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
                value: _,
                pos_start,
                pos_end: _,
            }) => pos_start.clone(),
            Value::Tuple(tuple::Tuple {
                value: _,
                pos_end: _,
                pos_start,
            }) => pos_start.clone(),
            Value::Set(set::Set {
                value: _,
                pos_start,
                pos_end: _,
            }) => pos_start.clone(),
            Value::Array(array::Array {
                value: _,
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
    fn raw_checking(&self) -> String {
        match self {
            Value::Number(number) => number.raw_checking(),
            Value::String(string) => string.raw_checking(),
            Value::Boolean(bool) => bool.raw_checking(),
            Value::Null(null) => null.raw_checking(),
            Value::Tuple(tuple) => tuple.raw_checking(),
            Value::Array(array) => array.raw_checking(),
            Value::Set(set) => set.raw_checking(),
            Value::Nil => String::new(),
        }
    }
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
    // INFO: All of the operation below are substances of the arithmetic_operating function

    // INFO: This function performs plus operation
    // Note that every single data type value (as soon they can support plus method) can
    // universally use this function to perform the plus operation
    fn add_to(&self, value: Value) -> Result<Value, Error> {
        self.operation(value, PLUS)
    }
    // INFO: This function performs minus operation
    // Note that every single data type value (as soon they can support minus method) can
    // universally use this function to perform the minus operation

    fn subtract_to(&self, value: Value) -> Result<Value, Error> {
        self.operation(value, MINUS)
    }
    // INFO: This function performs multiply operation
    // Note that every single data type value (as soon they can support multiply method) can
    // universally use this function to perform the multiply operation

    fn multiply_by(&self, value: Value) -> Result<Value, Error> {
        self.operation(value, MULTIPLY)
    }
    // INFO: This function performs divide operation
    // Note that every single data type value (as soon they can support divide method) can
    // universally use this function to perform the divide operation

    fn divide_by(&self, value: Value) -> Result<Value, Error> {
        self.operation(value, DIVIDE)
    }

    fn greater(&self, value: Value) -> Result<Value, Error> {
        self.operation(value, GREATER)
    }
    fn greater_or_equal(&self, value: Value) -> Result<Value, Error> {
        self.operation(value, GREATER_OR_EQUAL)
    }
    fn less(&self, value: Value) -> Result<Value, Error> {
        self.operation(value, LESS)
    }
    fn less_or_equal(&self, value: Value) -> Result<Value, Error> {
        self.operation(value, LESS_OR_EQUAL)
    }
    fn equal(&self, value: Value) -> Result<Value, Error> {
        self.operation(value, EQUAL)
    }
    fn not_equal(&self, value: Value) -> Result<Value, Error> {
        self.operation(value, NOT_EQUAL)
    }
    fn and(&self, value: Value) -> Result<Value, Error> {
        self.operation(value, AND)
    }
    fn or(&self, value: Value) -> Result<Value, Error> {
        self.operation(value, OR)
    }
    fn append(&mut self, value: Value) -> Result<Value, Error> {
        match self {
            Value::Set(value_origin) => value_origin.append(value),
            Value::Array(value_origin) => value_origin.append(value),
            _ => self.type_generate_error(value),
        }
    }
    fn indexing(&self, value: Value) -> Result<Value, Error> {
        self.operation(value, INDEXING)
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

    pub fn new_tuple(value: Vec<Value>, pos_start: Position, pos_end: Position) -> Value {
        Value::Tuple(tuple::Tuple::new(value, pos_start, pos_end))
    }

    pub fn new_set(value: Vec<Value>, pos_start: Position, pos_end: Position) -> Value {
        Value::Set(set::Set::new(value, pos_start, pos_end))
    }
    pub fn new_array(value: Vec<Value>, pos_start: Position, pos_end: Position) -> Value {
        Value::Array(array::Array::new(value, pos_start, pos_end))
    }
    fn handling_operation<T: ValueTrait>(
        &self,
        value_origin: T,
        value_other: Value,
        instruction: &str,
    ) -> Result<Value, Error> {
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
            INDEXING => value_origin.indexing(value_other.clone()),
            _ => Err(Error::new(
                "TypeError".to_string(),
                "Invalid types for such an operation".to_string(),
                self.get_pos_start(),
                self.get_pos_end(value_other),
            )),
        }
    }
    fn operation(&self, value: Value, instruction: &str) -> Result<Value, Error> {
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
            Value::Tuple(value_origin) => {
                self.handling_operation(value_origin.clone(), value, instruction)
            }
            Value::Set(value_origin) => {
                self.handling_operation(value_origin.clone(), value, instruction)
            }
            Value::Array(value_origin) => {
                self.handling_operation(value_origin.clone(), value, instruction)
            }
            Value::Null(value_origin) => {
                self.handling_operation(value_origin.clone(), value, instruction)
            }
            _ => self.type_generate_error(value),
        }
    }
}
