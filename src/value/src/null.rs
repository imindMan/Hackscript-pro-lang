use crate::value_trait::ValueTrait;
use crate::Value;
use error_handling::Error;
use position::Position;

#[derive(Debug, Clone)]
pub struct Null {
    pub value: String,
    pub pos_start: Position,
    pub pos_end: Position,
}

impl ValueTrait for Null {
    fn get_pos_start(&self) -> Position {
        self.pos_start.clone()
    }

    fn equal(&self, value: Value) -> (Option<Value>, Option<Error>) {
        self.comparison_operation(value, hacktypes::EQUAL)
    }
    fn not_equal(&self, value: Value) -> (Option<Value>, Option<Error>) {
        self.comparison_operation(value, hacktypes::NOT_EQUAL)
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
    fn comparison_operation(
        &self,
        other_value: Value,
        instruction: &str,
    ) -> (Option<Value>, Option<Error>) {
        let Value::Null(bool) = other_value.clone() else {return self.type_generate_error(other_value)};
        let value_origin: &str = self.value.as_str();
        let value_other: &str = bool.value.as_str();
        let check: bool = match instruction {
            hacktypes::EQUAL => value_origin == value_other,
            hacktypes::NOT_EQUAL => value_origin != value_other,
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
