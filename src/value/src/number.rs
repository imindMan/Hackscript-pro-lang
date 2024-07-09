// INFO: Number initialization
use error_handling::Error;
use hacktypes;
use lexer::Token;
use position::Position;

#[derive(Debug, Clone)]
pub struct Number {
    pub sign: String,
    pub identifier: String,
    pub value: String,
    pub pos_start: Position,
    pub pos_end: Position,
}

impl Number {
    pub fn new(
        sign: String,
        identifier: String,
        value: String,
        pos_start: Position,
        pos_end: Position,
    ) -> Number {
        Number {
            sign,
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
        // since Hackscript doesn't differentiate integer or float, it just treats everything as
        // "numbers", but Rust does treat them differently, so we'll have to build our simple
        // "smart" detector to check the final number is int or float. Ofc there are more than
        // this, but Hackscript is simple in its core but confusing anyway :))
        if self.identifier.as_str() == "float"
            || number.identifier.as_str() == "float"
            || (self.identifier.as_str() == "float" && number.identifier.as_str() == "float")
        {
            let mut number1: f32 = self.value.parse().unwrap();
            let mut number2: f32 = number.value.parse().unwrap();

            if self.sign.as_str() == hacktypes::MINUS {
                number1 *= -1.0;
            };
            if number.sign.as_str() == hacktypes::MINUS {
                number2 *= -1.0;
            };
            let final_res = number1 + number2;
            let mut sign: String = String::from(hacktypes::PLUS);

            if final_res < 0.0 {
                sign = String::from(hacktypes::MINUS);
            }

            let final_number: Option<Number> = Some(Number::new(
                sign,
                self.identifier.clone(),
                format!("{}", final_res.abs()),
                self.pos_start.clone(),
                self.pos_end.clone(),
            ));
            let err: Option<Error> = None;
            (final_number, err)
        } else {
            let mut number1: i32 = self.value.parse().unwrap();
            let mut number2: i32 = number.value.parse().unwrap();

            if self.sign.as_str() == hacktypes::MINUS {
                number1 = -number1;
            };
            if number.sign.as_str() == hacktypes::MINUS {
                number2 = -number2;
            };
            let final_res = number1 + number2;
            let mut sign: String = String::from(hacktypes::PLUS);

            if final_res < 0 {
                sign = String::from(hacktypes::MINUS);
            }

            let final_number: Option<Number> = Some(Number::new(
                sign,
                self.identifier.clone(),
                format!("{}", final_res.abs()),
                self.pos_start.clone(),
                self.pos_end.clone(),
            ));
            let err: Option<Error> = None;
            (final_number, err)
        }
    }
    pub fn subtract_to(&self, number: Number) -> (Option<Number>, Option<Error>) {
        // since Hackscript doesn't differentiate integer or float, it just treats everything as
        // "numbers", but Rust does treat them differently, so we'll have to build our simple
        // "smart" detector to check the final number is int or float. Ofc there are more than
        // this, but Hackscript is simple in its core but confusing anyway :))
        if self.identifier.as_str() == "float"
            || number.identifier.as_str() == "float"
            || (self.identifier.as_str() == "float" && number.identifier.as_str() == "float")
        {
            let mut number1: f32 = self.value.parse().unwrap();
            let mut number2: f32 = number.value.parse().unwrap();

            if self.sign.as_str() == hacktypes::MINUS {
                number1 *= -1.0;
            };
            if number.sign.as_str() == hacktypes::MINUS {
                number2 *= -1.0;
            };
            let final_res = number1 - number2;
            let mut sign: String = String::from(hacktypes::PLUS);

            if final_res < 0.0 {
                sign = String::from(hacktypes::MINUS);
            }

            let final_number: Option<Number> = Some(Number::new(
                sign,
                self.identifier.clone(),
                format!("{}", final_res.abs()),
                self.pos_start.clone(),
                self.pos_end.clone(),
            ));
            let err: Option<Error> = None;
            (final_number, err)
        } else {
            let mut number1: i32 = self.value.parse().unwrap();
            let mut number2: i32 = number.value.parse().unwrap();

            if self.sign.as_str() == hacktypes::MINUS {
                number1 = -number1;
            };
            if number.sign.as_str() == hacktypes::MINUS {
                number2 = -number2;
            };
            let final_res = number1 - number2;
            let mut sign: String = String::from(hacktypes::PLUS);

            if final_res < 0 {
                sign = String::from(hacktypes::MINUS);
            }

            let final_number: Option<Number> = Some(Number::new(
                sign,
                self.identifier.clone(),
                format!("{}", final_res.abs()),
                self.pos_start.clone(),
                self.pos_end.clone(),
            ));
            let err: Option<Error> = None;
            (final_number, err)
        }
    }
    pub fn multiply_by(&self, number: Number) -> (Option<Number>, Option<Error>) {
        // since Hackscript doesn't differentiate integer or float, it just treats everything as
        // "numbers", but Rust does treat them differently, so we'll have to build our simple
        // "smart" detector to check the final number is int or float. Ofc there are more than
        // this, but Hackscript is simple in its core but confusing anyway :))
        if self.identifier.as_str() == "float"
            || number.identifier.as_str() == "float"
            || (self.identifier.as_str() == "float" && number.identifier.as_str() == "float")
        {
            let mut number1: f32 = self.value.parse().unwrap();
            let mut number2: f32 = number.value.parse().unwrap();

            if self.sign.as_str() == hacktypes::MINUS {
                number1 *= -1.0;
            };
            if number.sign.as_str() == hacktypes::MINUS {
                number2 *= -1.0;
            };
            let final_res = number1 * number2;
            let mut sign: String = String::from(hacktypes::PLUS);

            if final_res < 0.0 {
                sign = String::from(hacktypes::MINUS);
            }

            let final_number: Option<Number> = Some(Number::new(
                sign,
                self.identifier.clone(),
                format!("{}", final_res.abs()),
                self.pos_start.clone(),
                self.pos_end.clone(),
            ));
            let err: Option<Error> = None;
            (final_number, err)
        } else {
            let mut number1: i32 = self.value.parse().unwrap();
            let mut number2: i32 = number.value.parse().unwrap();

            if self.sign.as_str() == hacktypes::MINUS {
                number1 = -number1;
            };
            if number.sign.as_str() == hacktypes::MINUS {
                number2 = -number2;
            };
            let final_res = number1 * number2;
            let mut sign: String = String::from(hacktypes::PLUS);

            if final_res < 0 {
                sign = String::from(hacktypes::MINUS);
            }

            let final_number: Option<Number> = Some(Number::new(
                sign,
                self.identifier.clone(),
                format!("{}", final_res.abs()),
                self.pos_start.clone(),
                self.pos_end.clone(),
            ));
            let err: Option<Error> = None;
            (final_number, err)
        }
    }
    pub fn divide_by(&self, number: Number) -> (Option<Number>, Option<Error>) {
        // since Hackscript doesn't differentiate integer or float, it just treats everything as
        // "numbers", but Rust does treat them differently, so we'll have to build our simple
        // "smart" detector to check the final number is int or float. Ofc there are more than
        // this, but Hackscript is simple in its core but confusing anyway :))
        if self.identifier.as_str() == "float"
            || number.identifier.as_str() == "float"
            || (self.identifier.as_str() == "float" && number.identifier.as_str() == "float")
        {
            let mut number1: f32 = self.value.parse().unwrap();
            let mut number2: f32 = number.value.parse().unwrap();

            if self.sign.as_str() == hacktypes::MINUS {
                number1 *= -1.0;
            };
            if number.sign.as_str() == hacktypes::MINUS {
                number2 *= -1.0;
            };
            let final_res = number1 / number2;
            let mut sign: String = String::from(hacktypes::PLUS);

            if final_res < 0.0 {
                sign = String::from(hacktypes::MINUS);
            }

            let final_number: Option<Number> = Some(Number::new(
                sign,
                self.identifier.clone(),
                format!("{}", final_res.abs()),
                self.pos_start.clone(),
                self.pos_end.clone(),
            ));
            let err: Option<Error> = None;
            (final_number, err)
        } else {
            let mut number1: i32 = self.value.parse().unwrap();
            let mut number2: i32 = number.value.parse().unwrap();

            if self.sign.as_str() == hacktypes::MINUS {
                number1 = -number1;
            };
            if number.sign.as_str() == hacktypes::MINUS {
                number2 = -number2;
            };
            let final_res = number1 / number2;
            let mut sign: String = String::from(hacktypes::PLUS);

            if final_res < 0 {
                sign = String::from(hacktypes::MINUS);
            }

            let final_number: Option<Number> = Some(Number::new(
                sign,
                self.identifier.clone(),
                format!("{}", final_res.abs()),
                self.pos_start.clone(),
                self.pos_end.clone(),
            ));
            let err: Option<Error> = None;
            (final_number, err)
        }
    }
}
