use crate::error_handling::Error;
use crate::position::Position;
use crate::value::string;
use crate::value::value_trait::ValueTrait;
use crate::Value;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Array {
    pub value: Vec<Value>,
    pub pos_start: Position,
    pub pos_end: Position,
}
impl Display for Array {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output: String = String::new();
        output.push('[');
        for i in &self.value {
            match i {
                Value::String(string::HackString {
                    value,
                    pos_start: _,
                    pos_end: _,
                }) => output.push_str(&format!("\"{}\", ", value)),
                _ => output.push_str(&format!("{}, ", &i)),
            }
        }
        output.push(']');
        write!(f, "{}", output)
    }
}

impl ValueTrait for Array {
    fn get_pos_start(&self) -> Position {
        self.pos_start.clone()
    }
    fn raw_checking(&self) -> String {
        format!("array {}", self)
    }
    fn append(&mut self, value: Value) -> Result<Value, Error> {
        self.value.push(value);

        Ok(Value::new_array(
            self.value.clone(),
            self.pos_start.clone(),
            self.pos_end.clone(),
        ))
    }
    fn indexing(&self, value: Value) -> Result<Value, Error> {
        let Value::Number(index) = value.clone() else {return Err(Error::new(
                "IndexError".to_string(),
                "Just indexing the tuple with the number index, not anything else".to_string(),
                self.get_pos_start(),
                self.get_pos_end(value)
        ))};
        if index.identifier.as_str() == "float" {
            return Err(Error::new(
                "IndexError".to_string(),
                "Cannot index using float".to_string(),
                self.get_pos_start(),
                self.get_pos_end(value),
            ));
        }
        let real_index: usize = index.value.parse().unwrap();
        if real_index >= self.value.len() {
            Err(Error::new(
                "IndexError".to_string(),
                "Index out of range".to_string(),
                self.get_pos_start(),
                self.get_pos_end(value),
            ))
        } else {
            Ok(self.value[real_index].clone())
        }
    }
}

impl Array {
    pub fn new(value: Vec<Value>, pos_start: Position, pos_end: Position) -> Array {
        Array {
            value,
            pos_start,
            pos_end,
        }
    }
}
