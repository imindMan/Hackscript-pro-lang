use crate::error_handling::Error;
use crate::hacktypes::*;
use crate::position::Position;
use crate::value::ValueTrait;
use crate::Value;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Null {
    pub value: String,
    pub pos_start: Position,
    pub pos_end: Position,
}

impl ValueTrait for Null {
    fn get_pos_start(&self) -> Position {
        self.pos_start.clone()
    }

    fn equal(&self, value: Value) -> Result<Value, Error> {
        self.comparison_operation(value, EQUAL)
    }
    fn not_equal(&self, value: Value) -> Result<Value, Error> {
        self.comparison_operation(value, NOT_EQUAL)
    }
}

impl Null {
    pub fn new(value: String, pos_start: Position, pos_end: Position) -> Null {
        Null {
            value,
            pos_start,
            pos_end,
        }
    }
    fn comparison_operation(&self, other_value: Value, instruction: &str) -> Result<Value, Error> {
        let Value::Null(bool) = other_value.clone() else {return self.type_generate_error(other_value)};
        let value_origin: &str = self.value.as_str();
        let value_other: &str = bool.value.as_str();
        let check: bool = match instruction {
            EQUAL => value_origin == value_other,
            NOT_EQUAL => value_origin != value_other,
            _ => {
                return Err(Error::new(
                    "OperatorError".to_string(),
                    "Invalid type for such an operation".to_string(),
                    self.pos_start.clone(),
                    bool.pos_end.clone(),
                ))
            }
        };
        Ok(Value::new_boolean(
            check,
            self.pos_start.clone(),
            bool.pos_end.clone(),
        ))
    }
}
