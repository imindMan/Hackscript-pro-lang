
/*
 * Lexer first start
 *
 * Lexer will parse through every characters and create tokens
 * So tokens can be defined in here, too
 *
 * */

use position::Position;

// Token definition

struct Token {
    _type: String,
    token: String,

}

// Lexer definition
pub struct Lexer {
    curr_char: char,
    curr_pos: Position,
    fname: String,
    fcontent: String
}

// implement Lexer 
impl Lexer {

    // Lexer constructor
    pub fn new(fname: String, fcontent: String) -> Lexer {
        Lexer {
            curr_char: match String::from(" ").as_str().chars().nth(0) {
                                 Some(char) => char, 
                                 _ => panic!("NO EXISTED CHARACTER!!!!!"),
                             },
            fname: fname.clone(),
            fcontent: fcontent.clone(),
            curr_pos: Position::new(0, 0, fname, fcontent)
        }
    }

    // make some tokens
    pub fn make_tokens()  {

    }

}


