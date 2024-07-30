// INFO: First start of the boolean
// and contain the Null attribute, too
use crate::Value;
use crate::ValueTrait;
use error_handling::Error;
use position::Position;
use std::fmt::Display;
// In Hackscript, to make it simple, I'll refer Null as boolean, too

#[derive(Debug, Clone)]
pub struct Boolean {
    pub boolean: bool,
    pub pos_start: Position,
    pub pos_end: Position,
}
impl Display for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.boolean)
    }
}
impl ValueTrait for Boolean {
    fn type_generate_error(&self, value: Value) -> (Option<Value>, Option<Error>) {
        let pos_start: Position = self.get_pos_start();
        let pos_end: Position = self.get_pos_end(value);
        self.generate_error(
            "TypeError".to_string(),
            "Invalid types for such an operation".to_string(),
            pos_start,
            pos_end,
        )
    }
    fn get_pos_start(&self) -> Position {
        self.pos_start.clone()
    }
    fn equal(&self, bool: Value) -> (Option<Value>, Option<Error>) {
        self.comparison_operation(bool, hacktypes::EQUAL)
    }
    fn not_equal(&self, bool: Value) -> (Option<Value>, Option<Error>) {
        self.comparison_operation(bool, hacktypes::NOT_EQUAL)
    }
    fn and(&self, bool: Value) -> (Option<Value>, Option<Error>) {
        self.comparison_operation(bool, hacktypes::AND)
    }
    fn or(&self, bool: Value) -> (Option<Value>, Option<Error>) {
        self.comparison_operation(bool, hacktypes::OR)
    }
}

impl Boolean {
    pub fn new(boolean: bool, pos_start: Position, pos_end: Position) -> Boolean {
        Boolean {
            boolean,
            pos_start,
            pos_end,
        }
    }
    fn comparison_operation(
        &self,
        bool_value: Value,
        instruction: &str,
    ) -> (Option<Value>, Option<Error>) {
        let Value::Boolean(bool) = bool_value.clone() else {return self.type_generate_error(bool_value)};
        let check: bool = match instruction {
            hacktypes::EQUAL => self.boolean == bool.boolean,
            hacktypes::NOT_EQUAL => self.boolean != bool.boolean,
            hacktypes::AND => self.boolean && bool.boolean,
            hacktypes::OR => self.boolean || bool.boolean,
            _ => {
                return self.generate_error(
                    "OperatorError".to_string(),
                    "Invalid type for such an operation".to_string(),
                    self.pos_start.clone(),
                    bool.pos_end.clone(),
                )
            }
        };

        let final_boolean: Option<Value> = Some(Value::new_boolean(
            check,
            self.pos_start.clone(),
            bool.pos_end.clone(),
        ));
        let err: Option<Error> = None;
        (final_boolean, err)
    }
}
