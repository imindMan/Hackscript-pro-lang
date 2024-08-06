use crate::error_handling::Error;
use crate::position::Position;
use crate::value::value_trait::ValueTrait;
use crate::Value;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Set {
    pub value: Vec<Value>,
    pub pos_start: Position,
    pub pos_end: Position,
}
impl Display for Set {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output: String = String::new();
        output.push('[');
        for i in &self.value {
            output.push_str(&format!("{}, ", &i));
        }
        output += "]";
        write!(f, "{}", output)
    }
}

impl ValueTrait for Set {
    fn get_pos_start(&self) -> Position {
        self.pos_start.clone()
    }
    fn append(&mut self, value: Value) -> Result<Value, Error> {
        self.value.push(value);
        Ok(Value::Nil)
    }
}

impl Set {
    pub fn new(value: Vec<Value>, pos_start: Position, pos_end: Position) -> Set {
        Set {
            value,
            pos_start,
            pos_end,
        }
    }
}
