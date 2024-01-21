/*
 * Lexer first start
 *
 * Lexer will parse through every characters and create tokens
 * So tokens can be defined in here, too
 *
 * */

use error_handling::Error;
use position::Position;

// Token definition
#[derive(Debug)]
pub struct Token {
    _type: String,
    value: String,
}

impl Token {
    pub fn new(_type: String, value: String) -> Token {
        Token {
            _type: _type.clone(),
            value: value.clone(),
        }
    }
}
// Lexer definition
pub struct Lexer {
    curr_char: Option<char>,
    curr_pos: Position,
    fname: String,
    fcontent: String,
}

// implement Lexer
impl Lexer {
    // Lexer constructor
    pub fn new(fname: String, fcontent: String) -> Lexer {
        Lexer {
            curr_char: match String::from(fcontent.clone()).as_str().chars().nth(0) {
                Some(char) => Some(char),
                _ => panic!("NO EXISTED CHARACTER!!!!!"),
            },
            fname: fname.clone(),
            fcontent: fcontent.clone(),
            curr_pos: Position::new(0, 0, 0, fname, fcontent),
        }
    }
    // create a token instance
    pub fn create_a_token(&self, _type: String, value: String) -> Token {
        let tok = Token::new(_type.clone(), value.clone());
        tok
    }

    // move to another character
    pub fn advance(&mut self) {
        // check if the position is valid
        let temp_pos = self.curr_pos.literal_pos + 1;

        let curr_char = match String::from(self.fcontent.clone())
            .as_str()
            .chars()
            .nth(temp_pos.try_into().unwrap())
        {
            Some(char) => char,
            _ => 'N',
        };
        if curr_char != 'N' {
            // change the current position
            self.curr_pos.literal_pos = self.curr_pos.literal_pos + 1;

            if self.curr_char.unwrap() == '\n' {
                self.curr_pos.col = self.curr_pos.col + 1;
                self.curr_pos.row = 0;
            } else {
                self.curr_pos.col = self.curr_pos.col + 1;
            };
            self.curr_char = Some(curr_char);
        } else {
            self.curr_char = None;
        }
    }
    // make some tokens
    pub fn make_tokens(&mut self) -> (Option<Vec<Token>>, Option<Error>) {
        let mut tokens: Option<Vec<Token>> = Some(Vec::new());
        let mut err: Option<Error> = None;
        while self.curr_char.is_some() {
            if self.curr_char.unwrap() == ' ' || self.curr_char.unwrap() == '\n' {
                self.advance();
                continue;
            } else if self.curr_char.unwrap() == '+' {
                let token: Token =
                    self.create_a_token(String::from(hacktypes::PLUS), String::from(""));
                tokens.as_mut().unwrap().push(token);
                self.advance();
                continue;
            } else if self.curr_char.unwrap() == '-' {
                let token: Token =
                    self.create_a_token(String::from(hacktypes::MINUS), String::from(""));
                tokens.as_mut().unwrap().push(token);
                self.advance();
                continue;
            } else if self.curr_char.unwrap() == '*' {
                let token: Token =
                    self.create_a_token(String::from(hacktypes::MULTIPLY), String::from(""));
                tokens.as_mut().unwrap().push(token);
                self.advance();
                continue;
            } else if self.curr_char.unwrap() == '/' {
                let token: Token =
                    self.create_a_token(String::from(hacktypes::DIVIDE), String::from(""));
                tokens.as_mut().unwrap().push(token);
                self.advance();
                continue;
            } else {
                tokens = None;
                err = Some(Error::new("Undefined character".to_string()));
                err.as_mut().unwrap().error_message = err
                    .as_mut()
                    .unwrap()
                    .error_messaging(self.curr_pos.clone(), self.curr_pos.clone());
                break; // Exit the loop in case of an undefined character
            }
        }

        (tokens, err)
    }
}
