// INFO: This is the Value enum, which represents the values in Hackscript language.
// Because Hackscript is an interpreted language meaning there's no distinction between data types.

pub mod number;
pub mod string;
use error_handling::Error;
use position::Position;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Value {
    Number(number::Number),
    String(string::HackString),
    Nil,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(number) => write!(f, "{}", number),
            Value::String(string) => write!(f, "{}", string),
            Value::Nil => write!(f, ""),
        }
    }
}

impl Value {
    // INFO: This is the default Value initialization, which will return the Nil value
    pub fn new() -> Value {
        Value::Nil
    }

    // INFO: This is the initialization method for the Number
    pub fn new_number(
        identifier: String,
        value: String,
        pos_start: Position,
        pos_end: Position,
    ) -> Value {
        Value::Number(number::Number::new(identifier, value, pos_start, pos_end))
    }
    pub fn new_string(value: String, pos_start: Position, pos_end: Position) -> Value {
        Value::String(string::HackString::new(value, pos_start, pos_end))
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

    fn operation(&self, value: Value, which_command: &str) -> (Option<Value>, Option<Error>) {
        if matches!(self, Value::Number(_)) {
            let Value::Number(value_origin) = self else { panic!("Expected that type should pass") };
            if std::mem::discriminant(self) == std::mem::discriminant(&value) {
                let Value::Number(value_other) = value else { panic!("Expected that type should pass") };
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
                    "The types aren't the same".to_string(),
                    value_origin.pos_start.clone(),
                    value_origin.pos_end.clone(),
                )
            }
        } else if matches!(self, Value::String(_)) {
            let Value::String(value_origin) = self else { panic!("Expected that type should pass") };
            if std::mem::discriminant(self) == std::mem::discriminant(&value) {
                let Value::String(value_other) = value else { panic!("Expected that type should pass") };
                let (temp_res_value, err) = match which_command {
                    hacktypes::PLUS => value_origin.add_to(value_other.clone()),
                    hacktypes::MINUS => value_origin.subtract_to(value_other.clone()),
                    hacktypes::MULTIPLY => {
                        return self.generate_error(
                            "TypeError".to_string(),
                            "Invalid types for such an operation".to_string(),
                            value_origin.pos_start.clone(),
                            value_origin.pos_end.clone(),
                        );
                    }
                    hacktypes::DIVIDE => value_origin.divide_by(value_other.clone()),
                    _ => panic!("Instruction doesn't exist"),
                };

                if err.is_some() {
                    let val = Some(Value::new());
                    return (val, err);
                };
                let res_value: Option<Value> = match temp_res_value.unwrap() {
                    string::HackString {
                        value,
                        pos_start,
                        pos_end,
                    } => Some(Value::new_string(value, pos_start, pos_end)),
                };
                (res_value, err)
            } else if matches!(value, Value::Number(_)) {
                let Value::Number(value_other) = value else { panic!("Expected that type should pass") };
                let (temp_res_value, err) = match which_command {
                    hacktypes::MULTIPLY => value_origin.multiply_by(value_other.clone()),
                    _ => {
                        return self.generate_error(
                            "TypeError".to_string(),
                            "Invalid types for such an operation".to_string(),
                            value_origin.pos_start.clone(),
                            value_origin.pos_end.clone(),
                        );
                    }
                };

                if err.is_some() {
                    let val = Some(Value::new());
                    return (val, err);
                };
                let res_value: Option<Value> = match temp_res_value.unwrap() {
                    string::HackString {
                        value,
                        pos_start,
                        pos_end,
                    } => Some(Value::new_string(value, pos_start, pos_end)),
                };
                (res_value, err)
            } else {
                self.generate_error(
                    "TypeError".to_string(),
                    "Invalid types for such an operation".to_string(),
                    value_origin.pos_start.clone(),
                    value_origin.pos_end.clone(),
                )
            }
        } else {
            panic!("Doesn't work");
        }
    }

    // INFO: All of the operation below are substances of the arithmetic_operating function

    // INFO: This function performs plus operation
    // Note that every single data type value (as soon they can support plus method) can
    // universally use this function to perform the plus operation
    pub fn add_to(&self, value: Value) -> (Option<Value>, Option<Error>) {
        self.operation(value, hacktypes::PLUS)
    }
    // INFO: This function performs minus operation
    // Note that every single data type value (as soon they can support minus method) can
    // universally use this function to perform the minus operation

    pub fn subtract_to(&self, value: Value) -> (Option<Value>, Option<Error>) {
        self.operation(value, hacktypes::MINUS)
    }
    // INFO: This function performs multiply operation
    // Note that every single data type value (as soon they can support multiply method) can
    // universally use this function to perform the multiply operation

    pub fn multiply_by(&self, value: Value) -> (Option<Value>, Option<Error>) {
        self.operation(value, hacktypes::MULTIPLY)
    }
    // INFO: This function performs divide operation
    // Note that every single data type value (as soon they can support divide method) can
    // universally use this function to perform the divide operation

    pub fn divide_by(&self, value: Value) -> (Option<Value>, Option<Error>) {
        self.operation(value, hacktypes::DIVIDE)
    }
}
