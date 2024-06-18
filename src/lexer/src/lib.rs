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
    pub value: String,
    pos_start: Position,
    pos_end: Position,
}

impl Token {
    pub fn new(_type: String, value: String, pos_start_: Position, pos_end_: Position) -> Token {
        Token {
            _type: _type,
            value: value,
            pos_start: pos_start_,
            pos_end: pos_end_,
        }
    }
}

impl Clone for Token {
    fn clone(&self) -> Self {
        Token {
            _type: self._type.clone(),
            value: self.value.clone(),
            pos_start: self.pos_start.clone(),
            pos_end: self.pos_end.clone(),
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
            curr_char: match fcontent.clone().as_str().chars().next() {
                Some(char) => Some(char),
                _ => panic!("No existed character detected"),
            },
            fname: fname.clone(),
            fcontent: fcontent.clone(),
            curr_pos: Position::new(0, 0, 0, fname, fcontent),
        }
    }
    // create a token instance
    pub fn create_a_token(
        &self,
        _type: String,
        value: String,
        pos_start: Position,
        pos_end: Position,
    ) -> Token {
        let tok = Token::new(_type, value, pos_start, pos_end);
        tok
    }

    // move to another character
    pub fn advance(&mut self) {
        // check if the position is valid
        let temp_pos = self.curr_pos.literal_pos + 1;

        let curr_char = match self
            .fcontent
            .clone()
            .as_str()
            .chars()
            .nth(temp_pos.try_into().unwrap())
        {
            Some(char) => char,
            _ => 'N',
        };
        if curr_char != 'N' {
            // change the current position
            self.curr_pos.literal_pos += 1;

            if self.curr_char.unwrap() == '\n' {
                self.curr_pos.col += 1;
                self.curr_pos.row = 0;
            } else {
                self.curr_pos.col += 1;
            };
            self.curr_char = Some(curr_char);
        } else {
            self.curr_char = None;
        }
    }
    // number creator
    pub fn number_token(&mut self) -> (Option<Token>, Option<Error>) {
        let pos_start = self.curr_pos.clone();
        let mut value: String = String::new();
        value.push(self.curr_char.unwrap());

        while hacktypes::NUMBERLIST.contains(self.curr_char.unwrap()) == true {
            self.advance();
            if hacktypes::NUMBERLIST.contains(self.curr_char.unwrap()) == true {
                value.push(self.curr_char.unwrap());
            } else if self.curr_char.unwrap() == '.' {
                value.push(self.curr_char.unwrap());
                self.advance();
                if self.curr_char == None
                    || hacktypes::NUMBERLIST.contains(self.curr_char.unwrap()) == false
                {
                    let tok: Option<Token> = None;

                    let mut err = Some(Error::new("Undefined character".to_string()));
                    self.curr_pos.literal_pos -= 1;
                    if self.curr_char.unwrap() == '\n' {
                        self.curr_pos.col -= 1;
                        self.curr_pos.row = 0;
                    } else {
                        self.curr_pos.col -= 1;
                    };

                    err.as_mut().unwrap().error_message = err
                        .as_mut()
                        .unwrap()
                        .error_messaging(self.curr_pos.clone(), self.curr_pos.clone());
                    return (tok, err);
                } else {
                    value.push(self.curr_char.unwrap());
                    continue;
                }
            }
        }
        let tok: Option<Token> = Some(Token::new(
            String::from(hacktypes::NUMBER),
            value,
            pos_start,
            self.curr_pos.clone(),
        ));
        let err: Option<Error> = None;
        (tok, err)
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
                let token: Token = self.create_a_token(
                    String::from(hacktypes::PLUS),
                    String::from(""),
                    self.curr_pos.clone(),
                    self.curr_pos.clone(),
                );
                tokens.as_mut().unwrap().push(token);
                self.advance();
                continue;
            } else if self.curr_char.unwrap() == '-' {
                let token: Token = self.create_a_token(
                    String::from(hacktypes::MINUS),
                    String::from(""),
                    self.curr_pos.clone(),
                    self.curr_pos.clone(),
                );
                tokens.as_mut().unwrap().push(token);
                self.advance();
                continue;
            } else if self.curr_char.unwrap() == '*' {
                let token: Token = self.create_a_token(
                    String::from(hacktypes::MULTIPLY),
                    String::from(""),
                    self.curr_pos.clone(),
                    self.curr_pos.clone(),
                );
                tokens.as_mut().unwrap().push(token);
                self.advance();
                continue;
            } else if self.curr_char.unwrap() == '/' {
                let token: Token = self.create_a_token(
                    String::from(hacktypes::DIVIDE),
                    String::from(""),
                    self.curr_pos.clone(),
                    self.curr_pos.clone(),
                );
                tokens.as_mut().unwrap().push(token);
                self.advance();
                continue;
            } else if self.curr_char.unwrap() == '(' {
                let token: Token = self.create_a_token(
                    String::from(hacktypes::PARENTHESE_OPEN),
                    String::from(""),
                    self.curr_pos.clone(),
                    self.curr_pos.clone(),
                );
                tokens.as_mut().unwrap().push(token);
                self.advance();
                continue;
            } else if self.curr_char.unwrap() == ')' {
                let token: Token = self.create_a_token(
                    String::from(hacktypes::PARENTHESE_CLOSE),
                    String::from(""),
                    self.curr_pos.clone(),
                    self.curr_pos.clone(),
                );
                tokens.as_mut().unwrap().push(token);
                self.advance();
                continue;
            } else if hacktypes::NUMBERLIST.contains(self.curr_char.unwrap()) == true {
                let (token, error) = self.number_token();
                if error.is_some() {
                    tokens = None;
                    err = error;
                    break;
                } else {
                    tokens.as_mut().unwrap().push(token.unwrap())
                }
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
        if err.is_none() {
            tokens.as_mut().unwrap().push(self.create_a_token(
                String::from(hacktypes::EOF),
                String::from(""),
                self.curr_pos.clone(),
                self.curr_pos.clone(),
            ));
            (tokens, err)
        } else {
            (tokens, err)
        }
    }
}
