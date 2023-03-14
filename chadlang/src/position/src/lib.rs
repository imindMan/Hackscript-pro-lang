/*
 * Position implementation
 *
 * Really, position uses to keep track on the token position. 
 * Use mainly for error handling, it's good
 *
 *
 *
 *
 * **/


// Position initialization
pub struct Position
{

    pub col: i32,
    pub row: i32,
    pub fname: String,
    pub fcontent: String,
}

// implementation
impl Position {
    // ofc, a new function to allocate a Position struct
    pub fn new(column: i32, row: i32, fname: String, fcontent: String) -> Position {

        Position {
            col: column,
            row: row,
            fname: fname,
            fcontent: fcontent
        }
    }

    // make a clone
    pub fn clone(&self) -> Position {
        Position {
            col: self.col,
            row: self.row,
            fname: self.fname.clone(),
            fcontent: self.fcontent.clone(),
        }
    }
}

// NOTE: syntax while reading the doc: Position(row, column)
// check if the position is in the valid scope
//
// For example: Position(0, 1) is valid in the scope where 
//                                  pos_start: Position(0, 0)
//                                  pos_end: Position(0, 5)
//              Position(0, 2) is invalid in the scope where
//                                  pos_start: Position(0, 3)
//                                  pos_end: Position(0, 5)
pub fn valid_pos
(
    check_pos: Position,
    pos_start: Position, 
    pos_end: Position,
) -> bool
{
    if pos_start.row != pos_end.row {
        if pos_start.row < check_pos.row && check_pos.row < pos_end.row 
        {
            return true;
        } 
        else if pos_start.row == check_pos.row {
            if pos_start.col <= check_pos.col {
        
                return true; 
            }
        }
        
        else if pos_end.row == check_pos.row {
            if pos_end.col >= check_pos.col {
                return true;
            }
        
        }
    }
    else {
       if pos_start.col <= check_pos.col && check_pos.col <= pos_end.col {
            return true;
       } 
    
    }
    return false;

}
