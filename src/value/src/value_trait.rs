use crate::boolean::Boolean;
use crate::null::Null;
use crate::number::Number;
use crate::string::HackString;
use crate::Value;
use error_handling::Error;
use position::Position;

pub trait ValueTrait {
    fn generate_error(
        &self,
        kind: String,
        extra_string: String,
        pos_start: Position,
        pos_end: Position,
    ) -> (Option<Value>, Option<Error>) {
        let number: Option<Value> = None;
        let error: Option<Error> = Some(Error::new(
            kind,
            extra_string,
            pos_start.clone(),
            pos_end.clone(),
        ));
        (number, error)
    }
    fn get_pos_end(&self, value: Value) -> Position {
        match value {
            Value::Number(Number {
                value: _,
                identifier: _,
                pos_start: _,
                pos_end,
            }) => pos_end.clone(),
            Value::String(HackString {
                value: _,
                pos_start: _,
                pos_end,
            }) => pos_end.clone(),
            Value::Boolean(Boolean {
                boolean: _,
                pos_start: _,
                pos_end,
            }) => pos_end.clone(),
            Value::Null(Null {
                value: _,
                pos_start: _,
                pos_end,
            }) => pos_end.clone(),
            _ => panic!("Invalid operation"),
        }
    }
    fn get_pos_start(&self) -> Position;
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
    fn add_to(&self, value: Value) -> (Option<Value>, Option<Error>) {
        self.type_generate_error(value)
    }
    fn subtract_to(&self, value: Value) -> (Option<Value>, Option<Error>) {
        self.type_generate_error(value)
    }
    fn multiply_by(&self, value: Value) -> (Option<Value>, Option<Error>) {
        self.type_generate_error(value)
    }
    fn divide_by(&self, value: Value) -> (Option<Value>, Option<Error>) {
        self.type_generate_error(value)
    }
    fn greater(&self, value: Value) -> (Option<Value>, Option<Error>) {
        self.type_generate_error(value)
    }
    fn greater_or_equal(&self, value: Value) -> (Option<Value>, Option<Error>) {
        self.type_generate_error(value)
    }
    fn less(&self, value: Value) -> (Option<Value>, Option<Error>) {
        self.type_generate_error(value)
    }
    fn less_or_equal(&self, value: Value) -> (Option<Value>, Option<Error>) {
        self.type_generate_error(value)
    }
    fn equal(&self, value: Value) -> (Option<Value>, Option<Error>) {
        self.type_generate_error(value)
    }
    fn not_equal(&self, value: Value) -> (Option<Value>, Option<Error>) {
        self.type_generate_error(value)
    }
    fn and(&self, value: Value) -> (Option<Value>, Option<Error>) {
        self.type_generate_error(value)
    }
    fn or(&self, value: Value) -> (Option<Value>, Option<Error>) {
        self.type_generate_error(value)
    }
}
