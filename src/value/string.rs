// INFO: HackString initialization
use crate::error_handling::Error;
use crate::hacktypes::*;
use crate::position::Position;
use crate::value::Value;
use crate::value::ValueTrait;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct HackString {
    pub value: String,
    pub pos_start: Position,
    pub pos_end: Position,
}
impl Display for HackString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
impl ValueTrait for HackString {
    fn get_pos_start(&self) -> Position {
        self.pos_start.clone()
    }
    // NOTE: This is the plus operation of the HackString
    // Cannot use this for direct plus operation, we have to go through the Value enum
    fn add_to(&self, string: Value) -> Result<Value, Error> {
        let Value::String(string_value) = string.clone() else {return self.type_generate_error(string)};
        Ok(Value::new_string(
            self.value.clone() + string_value.value.as_str(),
            self.pos_start.clone(),
            self.get_pos_end(string),
        ))
    }
    fn multiply_by(&self, number_value: Value) -> Result<Value, Error> {
        let Value::Number(number) = number_value.clone() else {return self.type_generate_error(number_value)};
        if number.identifier.as_str() == "float" {
            return self.generate_error(
                "TypeError".to_string(),
                "Cannot multiply a string with a float".to_string(),
                self.pos_start.clone(),
                number.pos_end.clone(),
            );
        } else {
            let value_number: i32 = number.value.parse().unwrap();
            if value_number < 0 {
                return self.generate_error(
                    "TypeError".to_string(),
                    "Cannot multiply a string with a negative number".to_string(),
                    self.pos_start.clone(),
                    number.pos_end.clone(),
                );
            } else if value_number == 0 {
                return self.generate_error("ValueError".to_string(), "Cannot multiply a string with '0'. If you want to empty the string, use an already existed module for string implementation".to_string(), self.pos_start.clone(), number.pos_end.clone());
            } else {
                let mut value_string: String = String::new();
                for _i in 0..value_number {
                    value_string += self.value.clone().as_str();
                }
                Ok(Value::new_string(
                    value_string,
                    self.pos_start.clone(),
                    number.pos_end.clone(),
                ))
            }
        }
    }
    fn equal(&self, string: Value) -> Result<Value, Error> {
        self.comparison_operation(string, EQUAL)
    }
    fn not_equal(&self, string: Value) -> Result<Value, Error> {
        self.comparison_operation(string, NOT_EQUAL)
    }
}

impl HackString {
    // INFO: This is the initialization function of the HackString
    pub fn new(value: String, pos_start: Position, pos_end: Position) -> HackString {
        HackString {
            value,
            pos_start,
            pos_end,
        }
    }

    fn comparison_operation(&self, string: Value, instruction: &str) -> Result<Value, Error> {
        let Value::String(string_value) = string.clone() else {return self.type_generate_error(string)};
        let check: bool = match instruction {
            EQUAL => self.value == string_value.value,
            NOT_EQUAL => self.value != string_value.value,
            _ => {
                return self.generate_error(
                    "TypeError".to_string(),
                    "Invalid types for such an operation".to_string(),
                    self.pos_start.clone(),
                    self.get_pos_end(string),
                )
            }
        };

        Ok(Value::new_boolean(
            check,
            self.pos_start.clone(),
            self.get_pos_end(string),
        ))
    }
}
