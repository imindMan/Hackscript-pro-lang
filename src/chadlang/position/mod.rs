/*  
 * Position defining in the project
 *
 * Idea: the position is used to keep track all the position 
 * in the program, so it can be used for error handling and stuff like that
 *
 * Position will secretly update on the main codeflow, so that if the user needs to 
 * tell where's the error, it will be there then.
 * */




pub struct Position 
{

    column: i32,
    row: i32,
    current_index: i32,
}

impl Position {

    pub fn new(column: i32, row: i32, current_index: i32) -> Position {

        Position {
            column: column,
            row: row,
            current_index: current_index,
        }
    }
}
