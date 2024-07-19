// INFO: HackString initialization
use error_handling::Error;
use hacktypes;
use position::Position;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct HackString {
    pub value: String,
    pub pos_start: Position,
    pub pos_end: Position,
}
impl Display for HackString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\"{}\"", self.value)
    }
}

impl HackString {
    // INFO: This is the initialization function of the HackString
    pub fn new(value: String, pos_start: Position, pos_end: Position) -> HackString {
        HackString {
            value,
            pos_start,
            pos_end,
        }
    }

    fn generate_error(
        &self,
        kind: String,
        extra_string: String,
        pos_start: Position,
        pos_end: Position,
    ) -> (Option<HackString>, Option<Error>) {
        let string: Option<HackString> = None;
        let error: Option<Error> = Some(Error::new(
            kind,
            extra_string,
            pos_start.clone(),
            pos_end.clone(),
        ));
        (string, error)
    }

    // NOTE: This is the plus operation of the HackString
    // Cannot use this for direct plus operation, we have to go through the Value enum
    pub fn add_to(&self, string: HackString) -> (Option<HackString>, Option<Error>) {
        let new_string: Option<HackString> = Some(HackString::new(
            self.value.clone() + string.value.as_str(),
            self.pos_start.clone(),
            string.pos_end.clone(),
        ));
        let err: Option<Error> = None;
        (new_string, err)
    }
    pub fn subtract_to(&self, string: HackString) -> (Option<HackString>, Option<Error>) {
        return self.generate_error(
            "OperationError".to_string(),
            "Cannot divide a string to another string".to_string(),
            self.pos_start.clone(),
            string.pos_end.clone(),
        );
    }
    pub fn multiply_by(&self, string: HackString) -> (Option<HackString>, Option<Error>) {
        return self.generate_error(
            "OperationError".to_string(),
            "Cannot divide a string to another string".to_string(),
            self.pos_start.clone(),
            string.pos_end.clone(),
        );
    }

    pub fn divide_by(&self, string: HackString) -> (Option<HackString>, Option<Error>) {
        return self.generate_error(
            "OperationError".to_string(),
            "Cannot divide a string to another string".to_string(),
            self.pos_start.clone(),
            string.pos_end.clone(),
        );
    }
}
