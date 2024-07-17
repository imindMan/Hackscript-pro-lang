// INFO: This is the Value enum, which represents the values in Hackscript language.
// Because Hackscript is an interpreted language meaning there's no distinction between data types.

pub mod number;
use error_handling::Error;
use position::Position;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Value {
    Number(number::Number),
    Nil,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(number) => write!(f, "{}", number),
            Value::Nil => write!(f, ""),
        }
    }
}

impl Value {
    // by default
    pub fn new() -> Value {
        Value::Nil
    }
    pub fn new_number(
        identifier: String,
        value: String,
        pos_start: Position,
        pos_end: Position,
    ) -> Value {
        Value::Number(number::Number::new(identifier, value, pos_start, pos_end))
    }

    fn generate_error(
        &self,
        r#type: String,
        extra_string: String,
        pos_start: Position,
        pos_end: Position,
    ) -> (Option<Value>, Option<Error>) {
        let val: Option<Value> = Some(Value::new());
        let err: Option<Error> = Some(Error::new(r#type, extra_string, pos_start, pos_end));
        (val, err)
    }

    pub fn arithmetic_operating(
        &self,
        value: Value,
        which_command: &str,
    ) -> (Option<Value>, Option<Error>) {
        let value_origin = match self {
            Value::Number(val) => val,
            Value::Nil => panic!("Cannot implement anything without a data type"),
        };
        let value_other = match &value {
            Value::Number(val) => val,
            Value::Nil => panic!("Cannot implement anything without a data type"),
        };

        if std::mem::discriminant(self) == std::mem::discriminant(&value) {
            let (temp_res_value, err) = match which_command {
                hacktypes::PLUS => value_origin.add_to(value_other.clone()),
                hacktypes::MINUS => value_origin.subtract_to(value_other.clone()),
                hacktypes::MULTIPLY => value_origin.multiply_by(value_other.clone()),
                hacktypes::DIVIDE => value_origin.divide_by(value_other.clone()),
                _ => panic!("Instruction doesn't exist"),
            };

            if err.is_some() {
                let val = Some(Value::new());
                return (val, err);
            };
            let res_value: Option<Value> = match temp_res_value.unwrap() {
                number::Number {
                    identifier,
                    value,
                    pos_start,
                    pos_end,
                } => Some(Value::new_number(identifier, value, pos_start, pos_end)),
            };
            (res_value, err)
        } else {
            self.generate_error(
                "TypeError".to_string(),
                "the types aren't the same".to_string(),
                value_origin.pos_start.clone(),
                value_origin.pos_end.clone(),
            )
        }
    }

    // INFO: All of the operation below are substances of the arithmetic_operating function

    pub fn add_to(&self, value: Value) -> (Option<Value>, Option<Error>) {
        self.arithmetic_operating(value, hacktypes::PLUS)
    }

    pub fn subtract_to(&self, value: Value) -> (Option<Value>, Option<Error>) {
        self.arithmetic_operating(value, hacktypes::MINUS)
    }

    pub fn multiply_by(&self, value: Value) -> (Option<Value>, Option<Error>) {
        self.arithmetic_operating(value, hacktypes::MULTIPLY)
    }
    pub fn divide_by(&self, value: Value) -> (Option<Value>, Option<Error>) {
        self.arithmetic_operating(value, hacktypes::DIVIDE)
    }
}
