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

use position::Position;
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
    pub fn error_message(pos_start: Position, pos_end: Position) ->  String {
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
         let mut check_pos = Position::new(0, 0, pos_start.fname.clone(), pos_start.fcontent.clone());
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
     
             if current_char ==  '\n' {
                 current_index += 1;
                 check_pos.row += 1;
             }
         }
     
         // after this execution we got the row number to keep track to the next loop 
         // and also we have the current_index to get the function known what's the start index to add
         // the current row
      
         // start adding the row + the highlighting
         while check_pos.row <= pos_end.row {
            error_message.push(
                             match text.as_str().chars().nth(current_index) {
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
                     if position::valid_pos(check_pos.clone(), pos_start.clone(), pos_end.clone()) == false {
                          error_message.push(' ');
                          check_pos.col += 1;
                      }
                      else {
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
         
         error_message


    }
    // design a custom error message 
    pub fn design_error(&self, error_design: String) 
    {

        /*
         * General settings:
         *          context: bool default true. 
         *              It will display error through context
         *              for instance, let's say you have an error in the main() function, 
         *              and the error causes by the function foo()
         *              if you turn on this: the error message will display the main text where
         *              the error occurs, and also it will display the definition of the foo
         *              function to explain further more the code, it looks like this (sample)
         *                      
         *                      error:5:6: in foo function
         *                          4| func foo(a: str, b: str) {
         *                           |             ^^^     ^^^     the types are STRING, not
         *                                                              INTEGER  
         *                      error:3:1: in main thread 
         *                          2|      a = 3; b = 4 
         *                           |          ^      ^   there are all integers
         *                          3|      foo(a, b) 
         *                           |          ---- cause here
         *                           |
         *                      help: considering switch the types to integers.
         *              We can now use <context> definition for the context thing
         *
         * Syntax: 
         *
         * 1. !keyword
         *      Error_design variable will have some keywords like this:
         *          1. pos_start: you can now call something inside this pos_start like
         *              pos_start.row, pos_start.column, pos_start.fname, etc.
         *          2. pos_end: you can now call something inside this pos_end, just like pos_start 
         *          3. file_name: this is a file name 
         *          4. code_display: well, the text contains the error to display
         *          5. help: the help message to help the dev what to do if they meet the error
         * 2. ?customize:
         *     There are tons of thing to implement to an error message. So, that's why we have
         *     here
         *     The positions themselves is just displayed the position. So no customization.
         *     file_name has something to customize here:
         *          1. absolute_path: bool default false 
         *          2. smart_path: bool default true 
         *          3. shorten_path (no extension): bool default false 
         *          4. extension: bool default false
         *      code_display:
         *          1. diff_highlight (there will be multiple types of highlighting in an error
         *             message): bool default false
         *          2. text_in_highlight (text display with the highlighting): bool default false
         *          3. char_highlight (what character should I use for highlighting): a vector default
         *             ['~']
         *          4. line_number (display line number): bool default false
         *          5. *only if diff_highlight is turned on.
         *              diff_types (define types of highlighting, for e.g. ["main_error",
         *              "how_does_it_cause"], you can name literally anything here): a
         *              vector default None
         *          6. text_in_highlight_types: a vector, default [] 
         *              You can specify something like this: ["error", "help"]
         *      help: help message is customized to fit the error type, 
         *          1. base_on_code_display (the display text will be configured exactly the same
         *             like code_display configuration): bool default true
         *      The customization is implemented in the .json file, no worries.
         *3. Example:
         *      Default configuration: 
         *          --settings.json--
         *
         *          {
         *             --snip--
         *             context: true,
         *             error: true, // really need, so the interpreter knows that your programming
         *             language isn't so ugly.
         *
         *             error_configuration: {
         *                 file_name: {
         *                     absolute_path: false,
         *                     smart_path: true,
         *                     shorten_path: false,
         *                     extension: false
         *
         *                 },
    *                      code_display: {
        *                      diff_highlight: false,
        *                      text_in_highlight: false,
        *                      char_highlight: ['~'],
        *                      line_number: false,
        *                      // no diff_types because diff_highlight is turned off
            *                  text_in_highlight: [],
            *
            *              },
            *              help: {
                *              base_on_code_display: true,
                    *      }
         *             }
         *             --snip--
         *          }
    *               The sample syntax:
        *               "error starts at: !pos_start.row - !pos_start.col in !file_name 
        *               !code_display
        *               <context>: error occurs at: !pos_start.row - !pos_start.col in !file_name
        *               !help
        *               "
        *
        *       With that sample configuration and syntax, we will get this error message:
        *          error starts at: 3 - 1 in sample.lang 
        *               foo(a);
        *                   ~  
        *          error occurs at: 1 - 1 in sample.lang 
        *               def func foo(a: int)
        *                               ~~~ 
        *          help: considering change the type of 'a' or the definition 'a' in the 'foo'
        *          function
         * */
        
    } 
}
