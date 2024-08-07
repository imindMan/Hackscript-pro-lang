// NOTE: now this is the unique thing
// set is a data type gn Hackscript, which works "quite" like Python, or unique array
// normally what we would expect from set is unordered, immutable, no duplicates
// HOWEVER, Hackscript implements set in a very different way, that's not a "set" anymore
// (anyway I don't have any good names so I picked up set, that's enough)
// instead of treating the set "unordered", it treats in a very predictable pattern:
// if the element is a duplicate element, don't get it, and keep the first instance.
// Something like "loose order", let's say like that.
// E.g [2, 3, 2, 4, 5, 3, 2]
// When come to the parsing part, Hackscript will parse every single element and remove
// duplicates, only keep the first instance it has already detected.
// Expected output: [2, 3, 4, 5]
// What's good about this kind of implementation? The benefit is that it's fixed order.
// The order is not really loose, so that's why after implementing something,
// we can build a small detector for indexing (if necessary).

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
    fn raw_checking(&self) -> String {
        format!("set {}", self)
    }
    fn append(&mut self, value: Value) -> Result<Value, Error> {
        if !self.check_contain(&value) {
            self.value.push(value);

            Ok(Value::new_set(
                self.value.clone(),
                self.pos_start.clone(),
                self.pos_end.clone(),
            ))
        } else {
            Ok(Value::new_set(
                self.value.clone(),
                self.pos_start.clone(),
                self.pos_end.clone(),
            ))
        }
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
    fn check_contain(&self, value: &Value) -> bool {
        let test_vector = self
            .value
            .iter()
            .map(|x| x.raw_checking())
            .collect::<Vec<String>>();
        test_vector.contains(&value.raw_checking())
    }
}
