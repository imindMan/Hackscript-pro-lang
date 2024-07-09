// INFO: Value definition, which is another enum

pub mod number;
use error_handling::Error;
use lexer::Token;
use position::Position;

#[derive(Debug, Clone)]
pub enum Value {
    Number(number::Number),
    Nil,
}

impl Value {
    // by default
    pub fn new() -> Value {
        Value::Nil
    }
    pub fn new_number(
        sign: String,
        identifier: String,
        value: String,
        pos_start: Position,
        pos_end: Position,
    ) -> Value {
        Value::Number(number::Number::new(
            sign, identifier, value, pos_start, pos_end,
        ))
    }

    fn generate_error(
        &self,
        r#type: String,
        extra_string: String,
        pos_start: Position,
        pos_end: Position,
    ) -> (Option<Value>, Option<Error>) {
        let ast: Option<Value> = Some(Value::Nil);
        let err: Option<Error> = Some(Error::new(r#type, extra_string, pos_start, pos_end));
        (ast, err)
    }

    pub fn add_to(&self, value: Value) -> (Option<Value>, Option<Error>) {
        let value_origin = match self {
            Value::Number(val) => val,
            Value::Nil => panic!("Cannot implement anything without a data type"),
        };
        let value_other = match &value {
            Value::Number(val) => val,
            Value::Nil => panic!("Cannot implement anything without a data type"),
        };

        if std::mem::discriminant(self) == std::mem::discriminant(&value) {
            let (temp_res_value, err) = value_origin.add_to(value_other.clone());
            let res_value: Option<Value> = match temp_res_value.unwrap() {
                number::Number {
                    sign,
                    identifier,
                    value,
                    pos_start,
                    pos_end,
                } => Some(Value::new_number(
                    sign, identifier, value, pos_start, pos_end,
                )),
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

    pub fn subtract_to(&self, value: Value) -> (Option<Value>, Option<Error>) {
        let value_origin = match self {
            Value::Number(val) => val,
            Value::Nil => panic!("Cannot implement anything without a data type"),
        };
        let value_other = match &value {
            Value::Number(val) => val,
            Value::Nil => panic!("Cannot implement anything without a data type"),
        };

        if std::mem::discriminant(self) == std::mem::discriminant(&value) {
            let (temp_res_value, err) = value_origin.subtract_to(value_other.clone());
            let res_value: Option<Value> = match temp_res_value.unwrap() {
                number::Number {
                    sign,
                    identifier,
                    value,
                    pos_start,
                    pos_end,
                } => Some(Value::new_number(
                    sign, identifier, value, pos_start, pos_end,
                )),
            };
            return (res_value, err);
        } else {
            return self.generate_error(
                "TypeError".to_string(),
                "the types aren't the same".to_string(),
                value_origin.pos_start.clone(),
                value_origin.pos_end.clone(),
            );
        }
    }

    pub fn multiply_by(&self, value: Value) -> (Option<Value>, Option<Error>) {
        let value_origin = match self {
            Value::Number(val) => val,
            Value::Nil => panic!("Cannot implement anything without a data type"),
        };
        let value_other = match &value {
            Value::Number(val) => val,
            Value::Nil => panic!("Cannot implement anything without a data type"),
        };

        if std::mem::discriminant(self) == std::mem::discriminant(&value) {
            let (temp_res_value, err) = value_origin.multiply_by(value_other.clone());
            let res_value: Option<Value> = match temp_res_value.unwrap() {
                number::Number {
                    sign,
                    identifier,
                    value,
                    pos_start,
                    pos_end,
                } => Some(Value::new_number(
                    sign, identifier, value, pos_start, pos_end,
                )),
            };
            return (res_value, err);
        } else {
            return self.generate_error(
                "TypeError".to_string(),
                "the types aren't the same".to_string(),
                value_origin.pos_start.clone(),
                value_origin.pos_end.clone(),
            );
        }
    }
    pub fn divide_by(&self, value: Value) -> (Option<Value>, Option<Error>) {
        let value_origin = match self {
            Value::Number(val) => val,
            Value::Nil => panic!("Cannot implement anything without a data type"),
        };
        let value_other = match &value {
            Value::Number(val) => val,
            Value::Nil => panic!("Cannot implement anything without a data type"),
        };

        if std::mem::discriminant(self) == std::mem::discriminant(&value) {
            let (temp_res_value, err) = value_origin.divide_by(value_other.clone());

            let res_value: Option<Value> = match temp_res_value.unwrap() {
                number::Number {
                    sign,
                    identifier,
                    value,
                    pos_start,
                    pos_end,
                } => Some(Value::new_number(
                    sign, identifier, value, pos_start, pos_end,
                )),
            };
            return (res_value, err);
        } else {
            return self.generate_error(
                "TypeError".to_string(),
                "the types aren't the same".to_string(),
                value_origin.pos_start.clone(),
                value_origin.pos_end.clone(),
            );
        }
    }
}
