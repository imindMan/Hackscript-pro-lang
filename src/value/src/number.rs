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

    pub fn add_to(&self, number: Number) -> (Option<Number>, Option<Error>) {
        // since Hackscript doesn't differ integer or float, it just treats everything as
        // "numbers", but Rust does treat them differently, so we'll have to build our simple
        // "smart" detector to check the final number is int or float. Ofc there are more than
        // this, but Hackscript is simple in its core but confusing anyway :))
        if self.identifier.as_str() == "float"
            || number.identifier.as_str() == "float"
            || (self.identifier.as_str() == "float" && number.identifier.as_str() == "float")
        {
            let mut number1: f32 = self.value.parse().unwrap();
            let mut number2: f32 = number.value.parse().unwrap();

            let final_res = number1 + number2;
            let mut sign: String = String::from(hacktypes::PLUS);

            if final_res < 0.0 {
                sign = String::from(hacktypes::MINUS);
            }

            let final_number: Option<Number> = Some(Number::new(
                "float".to_string(),
                format!("{}", final_res),
                self.pos_start.clone(),
                self.pos_end.clone(),
            ));
            let err: Option<Error> = None;
            (final_number, err)
        } else {
            let mut number1: i32 = self.value.parse().unwrap();
            let mut number2: i32 = number.value.parse().unwrap();

            let final_res = number1 + number2;
            let mut sign: String = String::from(hacktypes::PLUS);

            if final_res < 0 {
                sign = String::from(hacktypes::MINUS);
            }

            let final_number: Option<Number> = Some(Number::new(
                self.identifier.clone(),
                format!("{}", final_res),
                self.pos_start.clone(),
                self.pos_end.clone(),
            ));
            let err: Option<Error> = None;
            (final_number, err)
        }
    }
    pub fn subtract_to(&self, number: Number) -> (Option<Number>, Option<Error>) {
        // since Hackscript doesn't differ integer or float, it just treats everything as
        // "numbers", but Rust does treat them differently, so we'll have to build our simple
        // "smart" detector to check the final number is int or float. Ofc there are more than
        // this, but Hackscript is simple in its core but confusing anyway :))
        if self.identifier.as_str() == "float"
            || number.identifier.as_str() == "float"
            || (self.identifier.as_str() == "float" && number.identifier.as_str() == "float")
        {
            let mut number1: f32 = self.value.parse().unwrap();
            let mut number2: f32 = number.value.parse().unwrap();

            let final_res = number1 - number2;
            let mut sign: String = String::from(hacktypes::PLUS);

            if final_res < 0.0 {
                sign = String::from(hacktypes::MINUS);
            }

            let final_number: Option<Number> = Some(Number::new(
                "float".to_string(),
                format!("{}", final_res),
                self.pos_start.clone(),
                self.pos_end.clone(),
            ));
            let err: Option<Error> = None;
            (final_number, err)
        } else {
            let mut number1: i32 = self.value.parse().unwrap();
            let mut number2: i32 = number.value.parse().unwrap();

            let final_res = number1 - number2;
            let mut sign: String = String::from(hacktypes::PLUS);

            if final_res < 0 {
                sign = String::from(hacktypes::MINUS);
            }

            let final_number: Option<Number> = Some(Number::new(
                self.identifier.clone(),
                format!("{}", final_res),
                self.pos_start.clone(),
                self.pos_end.clone(),
            ));
            let err: Option<Error> = None;
            (final_number, err)
        }
    }
    pub fn multiply_by(&self, number: Number) -> (Option<Number>, Option<Error>) {
        // since Hackscript doesn't differ integer or float, it just treats everything as
        // "numbers", but Rust does treat them differently, so we'll have to build our simple
        // "smart" detector to check the final number is int or float. Ofc there are more than
        // this, but Hackscript is simple in its core but confusing anyway :))
        if self.identifier.as_str() == "float"
            || number.identifier.as_str() == "float"
            || (self.identifier.as_str() == "float" && number.identifier.as_str() == "float")
        {
            let mut number1: f32 = self.value.parse().unwrap();
            let mut number2: f32 = number.value.parse().unwrap();

            let final_res = number1 * number2;
            let mut sign: String = String::from(hacktypes::PLUS);

            if final_res < 0.0 {
                sign = String::from(hacktypes::MINUS);
            }

            let final_number: Option<Number> = Some(Number::new(
                "float".to_string(),
                format!("{}", final_res),
                self.pos_start.clone(),
                self.pos_end.clone(),
            ));
            let err: Option<Error> = None;
            (final_number, err)
        } else {
            let mut number1: i32 = self.value.parse().unwrap();
            let mut number2: i32 = number.value.parse().unwrap();
            let final_res = number1 * number2;
            let mut sign: String = String::from(hacktypes::PLUS);

            if final_res < 0 {
                sign = String::from(hacktypes::MINUS);
            }

            let final_number: Option<Number> = Some(Number::new(
                self.identifier.clone(),
                format!("{}", final_res),
                self.pos_start.clone(),
                self.pos_end.clone(),
            ));
            let err: Option<Error> = None;
            (final_number, err)
        }
    }
    pub fn divide_by(&self, number: Number) -> (Option<Number>, Option<Error>) {
        // since Hackscript doesn't differ integer or float, it just treats everything as
        // "numbers", but Rust does treat them differently, so we'll have to build our simple
        // "smart" detector to check the final number is int or float. Ofc there are more than
        // this, but Hackscript is simple in its core but confusing anyway :))
        let number_test: f32 = number.value.parse().unwrap();
        if number_test == 0.0 {
            return self.generate_error(
                "DivisionByZero".to_string(),
                "Cannot divide a number to zero, based on basic math".to_string(),
                self.pos_start.clone(),
                number.pos_end.clone(),
            );
        }

        // check if the divison became a float
        let mut number1: f32 = self.value.parse().unwrap();
        let mut number2: f32 = number.value.parse().unwrap();

        let final_res: f32 = number1 / number2;

        if final_res == final_res.floor() {
            let final_result = final_res.floor() as i32;
            let mut sign: String = String::from(hacktypes::PLUS);
            if final_result < 0 {
                sign = String::from(hacktypes::MINUS);
            }
            let final_number: Option<Number> = Some(Number::new(
                "integer".to_string(),
                format!("{}", final_result.abs()),
                self.pos_start.clone(),
                self.pos_end.clone(),
            ));
            let err: Option<Error> = None;
            (final_number, err)
        } else {
            let mut sign: String = String::from(hacktypes::PLUS);

            if final_res < 0.0 {
                sign = String::from(hacktypes::MINUS);
            }

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
