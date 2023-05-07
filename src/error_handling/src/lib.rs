/*
 * This module will help handling the error
 *
 * And also design the error message, too!
 * Idea:
 *
 * The error class will get the design string and then convert that to
 * the real message.
 *
 * Automatically by default
 * */

pub mod error_typing;
use position::Position;

#[derive(Debug)]
pub struct Error {
    kind: String,
    pub error_message: String,
}

impl Error {
    pub fn new(kind: String) -> Error {
        Error {
            kind: kind,
            error_message: String::from(""),
        }
    }
    pub fn error_messaging(&self, pos_start: Position, pos_end: Position) -> String {
        // IDEA:
        //
        //
        // Start by counting the row, if the row reached
        // the pos_start.row number
        // then now the error_message starts working
        //
        // Why would I do that? To get the current_index to
        // start adding the row
        //
        // We will now have a position to keep track to the right
        // position
        // Loop:
        // 1. Adding the current row to the error string
        // 2. After adding the current row, we can now add the "highlighting".
        //      1. If the current position is not in the valid scope of the
        //      pos_start and pos_end position, we just add a space
        //      2. Otherwise, we add this character "~"
        //
        //
        // needed variables
        let mut current_index = 0;
        let mut check_pos =
            Position::new(0, 0, 0, pos_start.fname.clone(), pos_start.fcontent.clone());
        let text = pos_start.fcontent.clone();
        let mut error_message = String::new();
        let mut buffer_col = 0;

        // start by counting the row
        while check_pos.row != pos_start.row {
            current_index += 1;
            let current_char = match text.as_str().chars().nth(current_index) {
                Some(char) => char,
                _ => panic!("NO EXISTED CHARACTER!!!!!"),
            };

            if current_char == '\n' {
                current_index += 1;
                check_pos.row += 1;
            }
        }
        // after this execution we got the row number to keep track to the next loop
        // and also we have the current_index to get the function known what's the start index to add
        // the current row

        // start adding the row + the highlighting
        while check_pos.row <= pos_end.row {
            error_message.push(match text.as_str().chars().nth(current_index) {
                Some(char) => char,
                _ => panic!("NO EXISTED CHARACTER!!!!!"),
            });
            current_index += 1;
            buffer_col += 1; // buffer_col basically keeps track of how many characters in the current
                             // row to create the exactly highlighted string

            let current_char = match text.as_str().chars().nth(current_index) {
                Some(char) => char,
                _ => panic!("NO EXISTED CHARACTER!!!!!"),
            };
            // ahh, we get the end of a row, let's add of highlighting!
            if current_char == '\n' {
                // first push the '\n' in the error_message
                error_message.push(current_char);
                current_index += 1;
                // now this is the main part of the highlighting
                while check_pos.col <= buffer_col - 1 {
                    if position::valid_pos(check_pos.clone(), pos_start.clone(), pos_end.clone())
                        == false
                    {
                        error_message.push(' ');
                        check_pos.col += 1;
                    } else {
                        error_message.push('~');
                        check_pos.col += 1;
                    }
                }

                // finally we add a '\n' and update the variables
                // for the next loop
                error_message.push('\n');
                check_pos.row += 1;
                buffer_col = 0;
                check_pos.col = 0;
            }
            // the loop will continue...
        }
        error_message.push_str(&error_typing::error_type_handling(self.kind.clone()));
        error_message
    }
}
