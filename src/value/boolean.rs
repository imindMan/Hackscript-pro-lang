// INFO: First start of the boolean
// and contain the Null attribute, too
use crate::error_handling::Error;
use crate::hacktypes::*;
use crate::position::Position;
use crate::value::Value;
use crate::value::ValueTrait;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Boolean {
    pub value: bool,
    pub pos_start: Position,
    pub pos_end: Position,
}
impl Display for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
impl ValueTrait for Boolean {
    fn get_pos_start(&self) -> Position {
        self.pos_start.clone()
    }
    fn raw_checking(&self) -> String {
        format!("bool {}", self)
    }
    fn equal(&self, bool: Value) -> Result<Value, Error> {
        self.comparison_operation(bool, EQUAL)
    }
    fn not_equal(&self, bool: Value) -> Result<Value, Error> {
        self.comparison_operation(bool, NOT_EQUAL)
    }
    fn and(&self, bool: Value) -> Result<Value, Error> {
        self.comparison_operation(bool, AND)
    }
    fn or(&self, bool: Value) -> Result<Value, Error> {
        self.comparison_operation(bool, OR)
    }
}

impl Boolean {
    pub fn new(boolean: bool, pos_start: Position, pos_end: Position) -> Boolean {
        Boolean {
            value: boolean,
            pos_start,
            pos_end,
        }
    }
    fn comparison_operation(&self, bool_value: Value, instruction: &str) -> Result<Value, Error> {
        let Value::Boolean(bool) = bool_value.clone() else {return self.type_generate_error(bool_value)};
        let check: bool = match instruction {
            EQUAL => self.value == bool.value,
            NOT_EQUAL => self.value != bool.value,
            AND => self.value && bool.value,
            OR => self.value || bool.value,
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
