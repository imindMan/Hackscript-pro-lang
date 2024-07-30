// INFO: Number initialization
use crate::value_trait::ValueTrait;
use crate::Value;
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
        write!(f, "{}", self.value)
    }
}

impl ValueTrait for Number {
    fn get_pos_start(&self) -> Position {
        self.pos_start.clone()
    }
    // NOTE: This is the plus operation of the Number
    // Cannot use this for direct plus operation, we have to go through the Value enum
    fn add_to(&self, number: Value) -> (Option<Value>, Option<Error>) {
        self.arithmetic_function(number, hacktypes::PLUS)
    }
    // NOTE: This is the minus operation of the Number
    // Cannot use this for direct minus operation, we have to go through the Value enum

    fn subtract_to(&self, number: Value) -> (Option<Value>, Option<Error>) {
        self.arithmetic_function(number, hacktypes::MINUS)
    }
    // NOTE: This is the multiply operation of the Number
    // Cannot use this for direct multiply operation, we have to go through the Value enum

    fn multiply_by(&self, number: Value) -> (Option<Value>, Option<Error>) {
        self.arithmetic_function(number, hacktypes::MULTIPLY)
    }
    // NOTE: This is the divide operation of the Number
    // Cannot use this for direct divide operation, we have to go through the Value enum

    fn divide_by(&self, number: Value) -> (Option<Value>, Option<Error>) {
        if !matches!(number, Value::Number(_)) {
            self.type_generate_error(number)
        } else {
            let Value::Number(number_to_test) = number.clone() else { panic!("Should work!")};
            let number_test: f32 = number_to_test.value.parse().unwrap();
            if number_test == 0.0 {
                return self.generate_error(
                    "DivisionByZero".to_string(),
                    "Cannot divide a number to zero, based on basic math".to_string(),
                    self.pos_start.clone(),
                    self.get_pos_end(number),
                );
            };

            self.arithmetic_function(number, hacktypes::DIVIDE)
        }
    }
    // NOTE: This is the greater operation of the Number
    fn greater(&self, number: Value) -> (Option<Value>, Option<Error>) {
        self.comparison_operation(number, hacktypes::GREATER)
    }
    // NOTE: This is the greater or equal operation of the Number
    fn greater_or_equal(&self, number: Value) -> (Option<Value>, Option<Error>) {
        self.comparison_operation(number, hacktypes::GREATER_OR_EQUAL)
    } // NOTE: This is the less operation of the Number
    fn less(&self, number: Value) -> (Option<Value>, Option<Error>) {
        self.comparison_operation(number, hacktypes::LESS)
    } // NOTE: This is the less or equal operation of the Number
    fn less_or_equal(&self, number: Value) -> (Option<Value>, Option<Error>) {
        self.comparison_operation(number, hacktypes::LESS_OR_EQUAL)
    } // NOTE: This is the equal operation of the Number
    fn equal(&self, number: Value) -> (Option<Value>, Option<Error>) {
        self.comparison_operation(number, hacktypes::EQUAL)
    } // NOTE: This is the not equal operation of the Number
    fn not_equal(&self, number: Value) -> (Option<Value>, Option<Error>) {
        self.comparison_operation(number, hacktypes::NOT_EQUAL)
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

    fn arithmetic_function(
        &self,
        number: Value,
        operation: &str,
    ) -> (Option<Value>, Option<Error>) {
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
        let Value::Number(value_other) = number.clone() else { return self.type_generate_error(number)};
        if (self.identifier.as_str() == "float" || value_other.identifier.as_str() == "float")
            && operation != hacktypes::DIVIDE
        {
            let number1: f32 = self.value.parse().unwrap();
            let number2: f32 = value_other.value.parse().unwrap();

            let final_res: f32 = match operation {
                hacktypes::PLUS => number1 + number2,
                hacktypes::MINUS => number1 - number2,
                hacktypes::MULTIPLY => number1 * number2,
                &_ => panic!("Invalid instruction"),
            };

            let final_num: Option<Value> = Some(Value::new_number(
                "float".to_string(),
                format!("{}", final_res),
                self.pos_start.clone(),
                self.pos_end.clone(),
            ));
            (final_num, err)
        } else if self.identifier.as_str() == "integer"
            && value_other.identifier.as_str() == "integer"
            && operation != hacktypes::DIVIDE
        {
            let number1: i32 = self.value.parse().unwrap();
            let number2: i32 = value_other.value.parse().unwrap();

            let final_res: i32 = match operation {
                hacktypes::PLUS => number1 + number2,
                hacktypes::MINUS => number1 - number2,
                hacktypes::MULTIPLY => number1 * number2,
                &_ => panic!("Invalid instruction"),
            };

            let final_num: Option<Value> = Some(Value::new_number(
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
            let number2: f32 = value_other.value.parse().unwrap();

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

                let final_number: Option<Value> = Some(Value::new_number(
                    "integer".to_string(),
                    format!("{}", final_result),
                    self.pos_start.clone(),
                    self.pos_end.clone(),
                ));
                let err: Option<Error> = None;
                (final_number, err)
            } else {
                let final_number: Option<Value> = Some(Value::new_number(
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
        number: Value,
        instruction: &str,
    ) -> (Option<Value>, Option<Error>) {
        // Idea: since we have to deal with two cases: the same types or not the same types of
        // number, in this case, int or float, we gonna need two different checks for it.
        // For the same case: convert all the numbers to that data types (int or float), and then
        // compare it like normal.
        // For different case: convert all of them to f32 then work like normal
        let Value::Number(value_other) = number.clone() else { return self.type_generate_error(number);};
        if self.identifier.as_str() != value_other.identifier.as_str()
            || (self.identifier.as_str() == value_other.identifier.as_str()
                && value_other.identifier.as_str() == "float")
        {
            let value_origin: f32 = self.value.parse().unwrap();
            let value_other: f32 = value_other.value.parse().unwrap();

            let check: bool = match instruction {
                hacktypes::GREATER => value_origin > value_other,
                hacktypes::GREATER_OR_EQUAL => value_origin >= value_other,
                hacktypes::LESS => value_origin < value_other,
                hacktypes::LESS_OR_EQUAL => value_origin <= value_other,
                hacktypes::EQUAL => value_origin == value_other,
                hacktypes::NOT_EQUAL => value_origin != value_other,
                _ => {
                    return self.generate_error(
                        "TypeError".to_string(),
                        "Invalid types for such an operation".to_string(),
                        self.pos_start.clone(),
                        self.get_pos_end(number),
                    )
                }
            };

            let final_bool: Option<Value> = Some(Value::new_boolean(
                check,
                self.pos_start.clone(),
                self.get_pos_end(number),
            ));
            let err: Option<Error> = None;
            (final_bool, err)
        } else {
            let value_origin: i32 = self.value.parse().unwrap();
            let value_other: i32 = value_other.value.parse().unwrap();

            let check: bool = match instruction {
                hacktypes::GREATER => value_origin > value_other,
                hacktypes::GREATER_OR_EQUAL => value_origin >= value_other,
                hacktypes::LESS => value_origin < value_other,
                hacktypes::LESS_OR_EQUAL => value_origin <= value_other,
                hacktypes::EQUAL => value_origin == value_other,
                hacktypes::NOT_EQUAL => value_origin != value_other,
                _ => {
                    return self.generate_error(
                        "TypeError".to_string(),
                        "Invalid types for such an operation".to_string(),
                        self.pos_start.clone(),
                        self.get_pos_end(number),
                    )
                }
            };

            let final_bool: Option<Value> = Some(Value::new_boolean(
                check,
                self.pos_start.clone(),
                self.get_pos_end(number),
            ));
            let err: Option<Error> = None;
            (final_bool, err)
        }
    }
}
