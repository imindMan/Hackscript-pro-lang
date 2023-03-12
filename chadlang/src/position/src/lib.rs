pub struct Position
{

    pub col: i32,
    pub row: i32,
    pub fname: String,
    pub fcontent: String,
}

impl Position {

    pub fn new(column: i32, row: i32, fname: String, fcontent: String) -> Position {

        Position {
            col: column,
            row: row,
            fname: fname,
            fcontent: fcontent
        }
    }
    pub fn clone(&self) -> Position {
        Position {
            col: self.col,
            row: self.row,
            fname: self.fname.clone(),
            fcontent: self.fcontent.clone(),
        }
    }
}

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
