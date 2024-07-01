// INFO: Number initialization
use error_handling::Error;
use hacktypes;
use lexer::Token;
use position::Position;

pub struct Number {
    sign: String,
    identifier: String,
    value: String,
    pos_start: Position,
    pos_end: Position,
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
        // if self.identifier.as_str() == "int"
        //     && number.identifier.as_str() == "int"
        //     && self.sign.as_str() == hacktypes::PLUS
        //     && self.sign.as_str() == hacktypes::PLUS
        // {
        //     let number1 = self.value.parse::<i32>().unwrap();
        //     let number2 = number.value.parse::<i32>().unwrap();

        //     let final_number: Option<Number> = Some(Number::new(
        //         self.sign.clone(),
        //         self.identifier.clone(),
        //         format!("{value}", value = number1 + number2),
        //         self.pos_start.clone(),
        //         self.pos_end.clone(),
        //     ));
        //     let err: Option<Error> = None;
        //     (final_number, err)
        // } else {
        //     self.generate_error(
        //         "RuntimeError".to_string(),
        //         "Currently there's no support for float addition".to_string(),
        //         self.pos_start.clone(),
        //         self.pos_end.clone(),
        //     )
        // }

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
            let final_number: Option<Number> = Some(Number::new(
                self.sign.clone(),
                self.identifier.clone(),
                format!("{value}", value = number1 + number2),
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
            let final_number: Option<Number> = Some(Number::new(
                self.sign.clone(),
                self.identifier.clone(),
                format!("{value}", value = number1 + number2),
                self.pos_start.clone(),
                self.pos_end.clone(),
            ));
            let err: Option<Error> = None;
            (final_number, err)
        }
    }
}
