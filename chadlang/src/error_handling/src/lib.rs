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
pub struct Error 
{

    kind: String,
    error_message: String
}


impl Error {
    pub fn new(kind: String) -> Error {
        Error {

            kind: kind, 
            error_message: String::from("")
        }
    }
    pub fn design_error(&self, error_design: String) 
    {
        
    } 
}
