// INFO: First start of the boolean
// and contain the Null attribute, too
use error_handling::Error;
use position::Position;
use std::fmt::Display;
// In Hackscript, to make it simple, I'll refer Null as boolean, too

#[derive(Debug, Clone)]
pub struct Boolean {
    boolean: String,
    pos_start: Position,
    pos_end: Position,
}
impl Display for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.boolean)
    }
}

impl Boolean {
    pub fn new(boolean: String, pos_start: Position, pos_end: Position) -> Boolean {
        Boolean {
            boolean,
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
    ) -> (Option<Boolean>, Option<Error>) {
        let boolean: Option<Boolean> = None;
        let error: Option<Error> = Some(Error::new(
            kind,
            extra_string,
            pos_start.clone(),
            pos_end.clone(),
        ));
        (boolean, error)
    }

    fn comparison_operation(
        &self,
        bool: Boolean,
        instruction: &str,
    ) -> (Option<Boolean>, Option<Error>) {
        let check: bool = match instruction {
            hacktypes::EQUAL => self.boolean == bool.boolean,
            hacktypes::NOT_EQUAL => self.boolean != bool.boolean,
            _ => {
                return self.generate_error(
                    "OperatorError".to_string(),
                    "Invalid type for such an operation".to_string(),
                    self.pos_start.clone(),
                    bool.pos_end.clone(),
                )
            }
        };

        let check_value: String = match check {
            true => String::from(hacktypes::TRUE),
            false => String::from(hacktypes::FALSE),
        };

        let final_boolean: Option<Boolean> = Some(Boolean::new(
            check_value,
            self.pos_start.clone(),
            self.pos_end.clone(),
        ));
        let err: Option<Error> = None;
        (final_boolean, err)
    }
    pub fn add_to(&self, bool: Boolean) -> (Option<Boolean>, Option<Error>) {
        return self.generate_error(
            "TypeError".to_string(),
            "Cannot add a boolean to another boolean".to_string(),
            self.pos_start.clone(),
            bool.pos_end.clone(),
        );
    }
    pub fn subtract_to(&self, bool: Boolean) -> (Option<Boolean>, Option<Error>) {
        return self.generate_error(
            "TypeError".to_string(),
            "Cannot subtract a boolean to another boolean".to_string(),
            self.pos_start.clone(),
            bool.pos_end.clone(),
        );
    }
    pub fn multiply_by(&self, bool: Boolean) -> (Option<Boolean>, Option<Error>) {
        return self.generate_error(
            "TypeError".to_string(),
            "Cannot multiply a boolean by another boolean".to_string(),
            self.pos_start.clone(),
            bool.pos_end.clone(),
        );
    }
    pub fn divide_by(&self, bool: Boolean) -> (Option<Boolean>, Option<Error>) {
        return self.generate_error(
            "TypeError".to_string(),
            "Cannot divide a boolean to another boolean".to_string(),
            self.pos_start.clone(),
            bool.pos_end.clone(),
        );
    }
    pub fn greater(&self, bool: Boolean) -> (Option<Boolean>, Option<Error>) {
        return self.generate_error(
            "TypeError".to_string(),
            "Cannot compare a string \"greater than\" another string".to_string(),
            self.pos_start.clone(),
            bool.pos_end.clone(),
        );
    }
    pub fn greater_or_equal(&self, bool: Boolean) -> (Option<Boolean>, Option<Error>) {
        return self.generate_error(
            "TypeError".to_string(),
            "Cannot compare a string \"greater than or equal\" another string".to_string(),
            self.pos_start.clone(),
            bool.pos_end.clone(),
        );
    }
    pub fn less(&self, bool: Boolean) -> (Option<Boolean>, Option<Error>) {
        return self.generate_error(
            "TypeError".to_string(),
            "Cannot compare a string \"less than\" another string".to_string(),
            self.pos_start.clone(),
            bool.pos_end.clone(),
        );
    }
    pub fn less_or_equal(&self, bool: Boolean) -> (Option<Boolean>, Option<Error>) {
        return self.generate_error(
            "TypeError".to_string(),
            "Cannot compare a string \"less than or equal\" another string".to_string(),
            self.pos_start.clone(),
            bool.pos_end.clone(),
        );
    }
    pub fn equal(&self, bool: Boolean) -> (Option<Boolean>, Option<Error>) {
        self.comparison_operation(bool, hacktypes::EQUAL)
    }
    pub fn not_equal(&self, bool: Boolean) -> (Option<Boolean>, Option<Error>) {
        self.comparison_operation(bool, hacktypes::NOT_EQUAL)
    }
}
