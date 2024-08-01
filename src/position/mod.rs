/* INFO:
 * Position implementation
 * Use mainly for error handling
 * **/
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Position {
    pub col: i32,
    pub row: i32,
    pub literal_pos: i32,
    pub fname: String,
    pub fcontent: String,
}
impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

impl Position {
    // INFO: This is the initialization method for the Position
    // Although the syntax when reading the doc is (row, column)
    // The initialization method will take col value first then row value
    // after.
    pub fn new(col: i32, row: i32, literal_pos: i32, fname: String, fcontent: String) -> Position {
        Position {
            col,
            row,
            literal_pos,
            fname,
            fcontent,
        }
    }
}

// NOTE: syntax while reading the doc: Position(row, column)
// check if the position is in the valid scope
// For example: Position(0, 1) is valid in the scope where
//                                  pos_start: Position(0, 0)
//                                  pos_end: Position(0, 5)
//              Position(0, 2) is invalid in the scope where
//                                  pos_start: Position(0, 3)
//                                  pos_end: Position(0, 5)
pub fn valid_pos(check_pos: Position, pos_start: Position, pos_end: Position) -> bool {
    if pos_start.row != pos_end.row {
        if pos_start.row < check_pos.row && check_pos.row < pos_end.row {
            return true;
        } else if pos_start.row == check_pos.row {
            if pos_start.col <= check_pos.col {
                return true;
            }
        } else if pos_end.row == check_pos.row && pos_end.col >= check_pos.col {
            return true;
        }
    } else {
        if pos_start.col <= check_pos.col && check_pos.col <= pos_end.col {
            return true;
        }
    }
    false
}
