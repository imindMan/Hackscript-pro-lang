// INFO: HackString initialization
use crate::boolean_and_null::Boolean;
use crate::number::Number;
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
        writeln!(f, "{}", self.value)
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

    fn generate_error(
        &self,
        kind: String,
        extra_string: String,
        pos_start: Position,
        pos_end: Position,
    ) -> (Option<HackString>, Option<Error>) {
        let string: Option<HackString> = None;
        let error: Option<Error> = Some(Error::new(
            kind,
            extra_string,
            pos_start.clone(),
            pos_end.clone(),
        ));
        (string, error)
    }

    fn generate_boolean_error(
        &self,
        kind: String,
        extra_string: String,
        pos_start: Position,
        pos_end: Position,
    ) -> (Option<Boolean>, Option<Error>) {
        let string: Option<Boolean> = None;
        let error: Option<Error> = Some(Error::new(
            kind,
            extra_string,
            pos_start.clone(),
            pos_end.clone(),
        ));
        (string, error)
    }
    fn comparison_operation(
        &self,
        string: HackString,
        instruction: &str,
    ) -> (Option<Boolean>, Option<Error>) {
        let check: bool = match instruction {
            hacktypes::EQUAL => self.value == string.value,
            hacktypes::NOT_EQUAL => self.value != string.value,
            _ => {
                return self.generate_boolean_error(
                    "TypeError".to_string(),
                    "Invalid types for such an operation".to_string(),
                    self.pos_start.clone(),
                    string.pos_end.clone(),
                )
            }
        };

        let check_value: String = match check {
            true => String::from(hacktypes::TRUE),
            false => String::from(hacktypes::FALSE),
        };

        let final_bool: Option<Boolean> = Some(Boolean::new(
            check_value,
            self.pos_start.clone(),
            string.pos_end.clone(),
        ));
        let err: Option<Error> = None;
        (final_bool, err)
    }

    // NOTE: This is the plus operation of the HackString
    // Cannot use this for direct plus operation, we have to go through the Value enum
    pub fn add_to(&self, string: HackString) -> (Option<HackString>, Option<Error>) {
        let new_string: Option<HackString> = Some(HackString::new(
            self.value.clone() + string.value.as_str(),
            self.pos_start.clone(),
            string.pos_end.clone(),
        ));
        let err: Option<Error> = None;
        (new_string, err)
    }
    pub fn subtract_to(&self, string: HackString) -> (Option<HackString>, Option<Error>) {
        return self.generate_error(
            "TypeError".to_string(),
            "Cannot subtract a string to another string".to_string(),
            self.pos_start.clone(),
            string.pos_end.clone(),
        );
    }
    pub fn multiply_by(&self, number: Number) -> (Option<HackString>, Option<Error>) {
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
                let new_string: Option<HackString> = Some(HackString::new(
                    value_string,
                    self.pos_start.clone(),
                    number.pos_end.clone(),
                ));
                let err: Option<Error> = None;
                (new_string, err)
            }
        }
    }

    pub fn divide_by(&self, string: HackString) -> (Option<HackString>, Option<Error>) {
        return self.generate_error(
            "TypeError".to_string(),
            "Cannot divide a string to another string".to_string(),
            self.pos_start.clone(),
            string.pos_end.clone(),
        );
    }
    pub fn greater(&self, string: HackString) -> (Option<Boolean>, Option<Error>) {
        return self.generate_boolean_error(
            "TypeError".to_string(),
            "Cannot compare a string \"greater than\" another string".to_string(),
            self.pos_start.clone(),
            string.pos_end.clone(),
        );
    }
    pub fn greater_or_equal(&self, string: HackString) -> (Option<Boolean>, Option<Error>) {
        return self.generate_boolean_error(
            "TypeError".to_string(),
            "Cannot compare a string \"greater than or equal\" another string".to_string(),
            self.pos_start.clone(),
            string.pos_end.clone(),
        );
    }
    pub fn less(&self, string: HackString) -> (Option<Boolean>, Option<Error>) {
        return self.generate_boolean_error(
            "TypeError".to_string(),
            "Cannot compare a string \"less than\" another string".to_string(),
            self.pos_start.clone(),
            string.pos_end.clone(),
        );
    }
    pub fn less_or_equal(&self, string: HackString) -> (Option<Boolean>, Option<Error>) {
        return self.generate_boolean_error(
            "TypeError".to_string(),
            "Cannot compare a string \"less than or equal\" another string".to_string(),
            self.pos_start.clone(),
            string.pos_end.clone(),
        );
    }
    pub fn equal(&self, string: HackString) -> (Option<Boolean>, Option<Error>) {
        self.comparison_operation(string, hacktypes::EQUAL)
    }
    pub fn not_equal(&self, string: HackString) -> (Option<Boolean>, Option<Error>) {
        self.comparison_operation(string, hacktypes::NOT_EQUAL)
    }
    pub fn and(&self, string: HackString) -> (Option<Boolean>, Option<Error>) {
        self.generate_boolean_error(
            "TypeError".to_string(),
            "String is not a Boolean".to_string(),
            self.pos_start.clone(),
            string.pos_end.clone(),
        )
    }
    pub fn or(&self, string: HackString) -> (Option<Boolean>, Option<Error>) {
        self.generate_boolean_error(
            "TypeError".to_string(),
            "String is not a Boolean".to_string(),
            self.pos_start.clone(),
            string.pos_end.clone(),
        )
    }
}
