// INFO: HackString initialization
use crate::Value;
use crate::ValueTrait;
use error_handling::Error;
use position::Position;
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
    fn type_generate_error(&self, value: Value) -> (Option<Value>, Option<Error>) {
        let pos_start: Position = self.pos_start.clone();
        let pos_end: Position = self.get_pos_end(value);
        self.generate_error(
            "TypeError".to_string(),
            "Invalid types for such an operation".to_string(),
            pos_start,
            pos_end,
        )
    }
    fn get_pos_start(&self) -> Position {
        self.pos_start.clone()
    }
    // NOTE: This is the plus operation of the HackString
    // Cannot use this for direct plus operation, we have to go through the Value enum
    fn add_to(&self, string: Value) -> (Option<Value>, Option<Error>) {
        let Value::String(string_value) = string.clone() else {return self.type_generate_error(string)};
        let new_string: Option<Value> = Some(Value::new_string(
            self.value.clone() + string_value.value.as_str(),
            self.pos_start.clone(),
            self.get_pos_end(string),
        ));
        let err: Option<Error> = None;
        (new_string, err)
    }
    fn multiply_by(&self, number_value: Value) -> (Option<Value>, Option<Error>) {
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
                let new_string: Option<Value> = Some(Value::new_string(
                    value_string,
                    self.pos_start.clone(),
                    number.pos_end.clone(),
                ));
                let err: Option<Error> = None;
                (new_string, err)
            }
        }
    }
    fn equal(&self, string: Value) -> (Option<Value>, Option<Error>) {
        self.comparison_operation(string, hacktypes::EQUAL)
    }
    fn not_equal(&self, string: Value) -> (Option<Value>, Option<Error>) {
        self.comparison_operation(string, hacktypes::NOT_EQUAL)
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

    fn comparison_operation(
        &self,
        string: Value,
        instruction: &str,
    ) -> (Option<Value>, Option<Error>) {
        let Value::String(string_value) = string.clone() else {return self.type_generate_error(string)};
        let check: bool = match instruction {
            hacktypes::EQUAL => self.value == string_value.value,
            hacktypes::NOT_EQUAL => self.value != string_value.value,
            _ => {
                return self.generate_error(
                    "TypeError".to_string(),
                    "Invalid types for such an operation".to_string(),
                    self.pos_start.clone(),
                    self.get_pos_end(string),
                )
            }
        };

        let check_value: String = match check {
            true => String::from(hacktypes::TRUE),
            false => String::from(hacktypes::FALSE),
        };

        let final_bool: Option<Value> = Some(Value::new_boolean_or_null(
            check_value,
            self.pos_start.clone(),
            self.get_pos_end(string),
        ));
        let err: Option<Error> = None;
        (final_bool, err)
    }
}
