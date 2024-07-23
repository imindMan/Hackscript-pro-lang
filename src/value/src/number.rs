// INFO: Number initialization
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
}
