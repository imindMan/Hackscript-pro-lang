// INFO: Number initialization
use crate::boolean_and_null::Boolean;
use error_handling::Error;
use hacktypes;
use position::Position;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Number {
    pub identifier: String,
    pub value: String,
    pub pos_start: Position,
    pub pos_end: Position,
}
impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.value)
    }
}

impl Number {
    // INFO: This is the initialization function of the Number
    pub fn new(
        identifier: String,
        value: String,
        pos_start: Position,
        pos_end: Position,
    ) -> Number {
        Number {
            identifier,
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
    ) -> (Option<Number>, Option<Error>) {
        let number: Option<Number> = None;
        let error: Option<Error> = Some(Error::new(
            kind,
            extra_string,
            pos_start.clone(),
            pos_end.clone(),
        ));
        (number, error)
    }
    fn generate_boolean_error(
        &self,
        kind: String,
        extra_string: String,
        pos_start: Position,
        pos_end: Position,
    ) -> (Option<Boolean>, Option<Error>) {
        let number: Option<Boolean> = None;
        let error: Option<Error> = Some(Error::new(
            kind,
            extra_string,
            pos_start.clone(),
            pos_end.clone(),
        ));
        (number, error)
    }

    fn arithmetic_function(
        &self,
        number: Number,
        operation: &str,
    ) -> (Option<Number>, Option<Error>) {
        // since Hackscript doesn't differ integer or float, it just treats everything as
        // "numbers", but Rust does treat them differently, so we'll have to build our simple
        // "smart" detector to check the final number is int or float. Ofc there are more than
        // this, but Hackscript is simple in its core but confusing anyway :))

        let err: Option<Error> = None;

        // first thing to do is to define the data type of the final number value.
        // In such operations like addition, subtraction and multiplication, it's ideal
        // to assume that if all (two) numbers have the same data type, the final
        // result will also have that data type. If they have different data type, the result will
        // immediately have the float data type. We need to specify the same data type
        // case with two cases "float" and "int", since if we combine it together it's basically
        // impossible because Rust cannot really adapt the data type dynamically using a simple
        // match check.
        //
        // Note that this logic only works with addition, subtraction, and multiplication.
        // Division will need a whole new logic, since it doesn't care about data types
        // of the factors, but rather the result
        if (self.identifier.as_str() == "float" || number.identifier.as_str() == "float")
            && operation != hacktypes::DIVIDE
        {
            let number1: f32 = self.value.parse().unwrap();
            let number2: f32 = number.value.parse().unwrap();

            let final_res: f32 = match operation {
                hacktypes::PLUS => number1 + number2,
                hacktypes::MINUS => number1 - number2,
                hacktypes::MULTIPLY => number1 * number2,
                &_ => panic!("Invalid instruction"),
            };

            let final_num: Option<Number> = Some(Number::new(
                "float".to_string(),
                format!("{}", final_res),
                self.pos_start.clone(),
                self.pos_end.clone(),
            ));
            (final_num, err)
        } else if self.identifier.as_str() == "integer"
            && number.identifier.as_str() == "integer"
            && operation != hacktypes::DIVIDE
        {
            let number1: i32 = self.value.parse().unwrap();
            let number2: i32 = number.value.parse().unwrap();

            let final_res: i32 = match operation {
                hacktypes::PLUS => number1 + number2,
                hacktypes::MINUS => number1 - number2,
                hacktypes::MULTIPLY => number1 * number2,
                &_ => panic!("Invalid instruction"),
            };

            let final_num: Option<Number> = Some(Number::new(
                "integer".to_string(),
                format!("{}", final_res),
                self.pos_start.clone(),
                self.pos_end.clone(),
            ));
            (final_num, err)
        }
        // After checking for every single possible cases, we end up in the final case: division.
        // In this case we just need to convert every number to f32 then divide it like normal.
        // then we'll have a simple logic to convert the f32 to i32 if necessary.
        else {
            let number1: f32 = self.value.parse().unwrap();
            let number2: f32 = number.value.parse().unwrap();

            let final_res: f32 = match operation {
                hacktypes::PLUS => number1 + number2,
                hacktypes::MINUS => number1 - number2,
                hacktypes::MULTIPLY => number1 * number2,
                hacktypes::DIVIDE => number1 / number2,
                &_ => panic!("No existing instruction"),
            };

            // this simple logic here can be used for verifying if the number can
            // be converted to i32, because f32 is scary.
            if final_res == final_res.floor() {
                let final_result = final_res.floor() as i32;

                let final_number: Option<Number> = Some(Number::new(
                    "integer".to_string(),
                    format!("{}", final_result),
                    self.pos_start.clone(),
                    self.pos_end.clone(),
                ));
                let err: Option<Error> = None;
                (final_number, err)
            } else {
                let final_number: Option<Number> = Some(Number::new(
                    "float".to_string(),
                    format!("{}", final_res),
                    self.pos_start.clone(),
                    self.pos_end.clone(),
                ));
                let err: Option<Error> = None;
                (final_number, err)
            }
        }
    }
    fn comparison_operation(
        &self,
        number: Number,
        instruction: &str,
    ) -> (Option<Boolean>, Option<Error>) {
        // Idea: since we have to deal with two cases: the same types or not the same types of
        // number, in this case, int or float, we gonna need two different checks for it.
        // For the same case: convert all the numbers to that data types (int or float), and then
        // compare it like normal.
        // For different case: convert all of them to f32 then work like normal ?
        if self.identifier.as_str() != number.identifier.as_str()
            || (self.identifier.as_str() == number.identifier.as_str()
                && number.identifier.as_str() == "float")
        {
            let value_origin: f32 = self.value.parse().unwrap();
            let value_other: f32 = number.value.parse().unwrap();

            let check: bool = match instruction {
                hacktypes::GREATER => value_origin > value_other,
                hacktypes::GREATER_OR_EQUAL => value_origin >= value_other,
                hacktypes::LESS => value_origin < value_other,
                hacktypes::LESS_OR_EQUAL => value_origin <= value_other,
                hacktypes::EQUAL => value_origin == value_other,
                hacktypes::NOT_EQUAL => value_origin != value_other,
                _ => {
                    return self.generate_boolean_error(
                        "TypeError".to_string(),
                        "Invalid types for such an operation".to_string(),
                        self.pos_start.clone(),
                        number.pos_end.clone(),
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
                number.pos_end.clone(),
            ));
            let err: Option<Error> = None;
            (final_bool, err)
        } else {
            let value_origin: i32 = self.value.parse().unwrap();
            let value_other: i32 = number.value.parse().unwrap();

            let check: bool = match instruction {
                hacktypes::GREATER => value_origin > value_other,
                hacktypes::GREATER_OR_EQUAL => value_origin >= value_other,
                hacktypes::LESS => value_origin < value_other,
                hacktypes::LESS_OR_EQUAL => value_origin <= value_other,
                hacktypes::EQUAL => value_origin == value_other,
                hacktypes::NOT_EQUAL => value_origin != value_other,
                _ => {
                    return self.generate_boolean_error(
                        "TypeError".to_string(),
                        "Invalid types for such an operation".to_string(),
                        self.pos_start.clone(),
                        number.pos_end.clone(),
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
                number.pos_end.clone(),
            ));
            let err: Option<Error> = None;
            (final_bool, err)
        }
    }

    // NOTE: This is the plus operation of the Number
    // Cannot use this for direct plus operation, we have to go through the Value enum
    pub fn add_to(&self, number: Number) -> (Option<Number>, Option<Error>) {
        self.arithmetic_function(number, hacktypes::PLUS)
    }
    // NOTE: This is the minus operation of the Number
    // Cannot use this for direct minus operation, we have to go through the Value enum

    pub fn subtract_to(&self, number: Number) -> (Option<Number>, Option<Error>) {
        self.arithmetic_function(number, hacktypes::MINUS)
    }
    // NOTE: This is the multiply operation of the Number
    // Cannot use this for direct multiply operation, we have to go through the Value enum

    pub fn multiply_by(&self, number: Number) -> (Option<Number>, Option<Error>) {
        self.arithmetic_function(number, hacktypes::MULTIPLY)
    }
    // NOTE: This is the divide operation of the Number
    // Cannot use this for direct divide operation, we have to go through the Value enum

    pub fn divide_by(&self, number: Number) -> (Option<Number>, Option<Error>) {
        let number_test: f32 = number.value.parse().unwrap();
        if number_test == 0.0 {
            return self.generate_error(
                "DivisionByZero".to_string(),
                "Cannot divide a number to zero, based on basic math".to_string(),
                self.pos_start.clone(),
                number.pos_end.clone(),
            );
        };

        self.arithmetic_function(number, hacktypes::DIVIDE)
    }
    // NOTE: This is the greater operation of the Number
    pub fn greater(&self, number: Number) -> (Option<Boolean>, Option<Error>) {
        self.comparison_operation(number, hacktypes::GREATER)
    }
    // NOTE: This is the greater or equal operation of the Number
    pub fn greater_or_equal(&self, number: Number) -> (Option<Boolean>, Option<Error>) {
        self.comparison_operation(number, hacktypes::GREATER_OR_EQUAL)
    } // NOTE: This is the less operation of the Number
    pub fn less(&self, number: Number) -> (Option<Boolean>, Option<Error>) {
        self.comparison_operation(number, hacktypes::LESS)
    } // NOTE: This is the less or equal operation of the Number
    pub fn less_or_equal(&self, number: Number) -> (Option<Boolean>, Option<Error>) {
        self.comparison_operation(number, hacktypes::LESS_OR_EQUAL)
    } // NOTE: This is the equal operation of the Number
    pub fn equal(&self, number: Number) -> (Option<Boolean>, Option<Error>) {
        self.comparison_operation(number, hacktypes::EQUAL)
    } // NOTE: This is the not equal operation of the Number
    pub fn not_equal(&self, number: Number) -> (Option<Boolean>, Option<Error>) {
        self.comparison_operation(number, hacktypes::NOT_EQUAL)
    }

    pub fn and(&self, number: Number) -> (Option<Boolean>, Option<Error>) {
        self.generate_boolean_error(
            "TypeError".to_string(),
            "Number is not a Boolean".to_string(),
            self.pos_start.clone(),
            number.pos_end.clone(),
        )
    }
    pub fn or(&self, number: Number) -> (Option<Boolean>, Option<Error>) {
        self.generate_boolean_error(
            "TypeError".to_string(),
            "Number is not a Boolean".to_string(),
            self.pos_start.clone(),
            number.pos_end.clone(),
        )
    }
}
