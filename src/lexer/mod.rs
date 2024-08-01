//  INFO:  Lexer first start.
//  Lexer will parse through every characters and create tokens
//  So tokens can be defined in here, too

use crate::error_handling::Error;
use crate::hacktypes::*;
use crate::position::Position;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Token {
    pub _type: String,
    pub value: String,
    pub pos_start: Position,
    pub pos_end: Position,
}

impl Token {
    pub fn new(_type: String, value: String, pos_start: Position, pos_end: Position) -> Token {
        Token {
            _type,
            value,
            pos_start,
            pos_end,
        }
    }
}

pub struct Lexer {
    curr_char: Option<char>,
    curr_pos: Position,
    fcontent: String,
}

impl Lexer {
    // INFO: This is the Initialization method of Lexer
    pub fn new(fname: String, fcontent: String) -> Lexer {
        Lexer {
            curr_char: match fcontent.clone().as_str().chars().next() {
                Some(char) => Some(char),
                _ => None,
            },
            fcontent: fcontent.clone(),
            curr_pos: Position::new(0, 0, 0, fname, fcontent),
        }
    }
    // ------------------------------------------------------------------
    // INFO: Below are some methods that are frequently used in the lexer
    // ------------------------------------------------------------------

    // This function is for the whole Vec<Token>, since we are working with two main objects:
    // Vec<Token> and Token (the real reason is to serve for the number_token() to work)
    fn generate_error(
        &self,
        r#type: String,
        extra_string: String,
        pos_start: Position,
        pos_end: Position,
    ) -> (Option<Vec<Token>>, Option<Error>) {
        let tok: Option<Vec<Token>> = None;
        let err: Option<Error> = Some(Error::new(r#type, extra_string, pos_start, pos_end));
        (tok, err)
    }
    // This function is for only Token item, since we are working with two main objects:
    // Vec<Token> and Token (the real reason is to serve for the number_token() to work)
    fn generate_individual_error(
        &self,
        r#type: String,
        extra_string: String,
        pos_start: Position,
        pos_end: Position,
    ) -> (Option<Token>, Option<Error>) {
        let tok: Option<Token> = None;
        let err: Option<Error> = Some(Error::new(r#type, extra_string, pos_start, pos_end));
        (tok, err)
    }
    // 'advance' is actually to iterate to the next token
    fn advance(&mut self) {
        let temp_pos = self.curr_pos.literal_pos + 1;
        let curr_char: Option<char> = self
            .fcontent
            .clone()
            .as_str()
            .chars()
            .nth(temp_pos.try_into().unwrap());

        if curr_char.is_some() {
            // change the current position
            self.curr_pos.literal_pos += 1;

            if self.curr_char.unwrap() == '\n' {
                self.curr_pos.col += 1;
                self.curr_pos.row = 0;
            } else {
                self.curr_pos.col += 1;
            };
            self.curr_char = Some(curr_char.unwrap());
        } else {
            self.curr_char = None;
        }
    }
    fn implement_trailing_characters(&mut self) -> (char, Option<Error>) {
        let mut err: Option<Error> = None;
        match self.curr_char.unwrap() {
            'n' => ('\n', err),
            't' => ('\t', err),
            '\\' => ('\\', err),
            '\"' => ('\"', err),
            _ => {
                err = Some(Error::new(
                    "UnknownTrailingCharacter".to_string(),
                    format!(
                        "Doesn't exist this '\\{}' character",
                        self.curr_char.unwrap()
                    ),
                    self.curr_pos.clone(),
                    self.curr_pos.clone(),
                ));
                return (' ', err);
            }
        }
    }

    fn string_token(&mut self) -> (Option<Token>, Option<Error>) {
        let pos_start = self.curr_pos.clone();
        let mut value: String = String::new();
        self.advance();
        while self.curr_char.is_some() && self.curr_char.unwrap() != '\"' {
            if self.curr_char.unwrap() == '\\' {
                self.advance();
                let (char, err) = self.implement_trailing_characters();
                if err.is_some() {
                    let tok: Option<Token> = None;
                    return (tok, err);
                }
                value.push(char);
            } else {
                value.push(self.curr_char.unwrap());
            }
            self.advance();
        }
        if self.curr_char.is_none() {
            return self.generate_individual_error(
                "Expect".to_string(),
                "a \" to end the string. Found nothing -> endless string".to_string(),
                pos_start,
                self.curr_pos.clone(),
            );
        }
        let tok: Option<Token> = Some(Token::new(
            String::from(STRING),
            value,
            pos_start,
            self.curr_pos.clone(),
        ));
        let err: Option<Error> = None;
        (tok, err)
    }

    fn number_token(&mut self) -> (Option<Token>, Option<Error>) {
        let pos_start = self.curr_pos.clone();
        let mut value: String = String::new();

        while self.curr_char.is_some() && NUMBERLIST.contains(self.curr_char.unwrap()) {
            if NUMBERLIST.contains(self.curr_char.unwrap()) && self.curr_char.unwrap() != '.' {
                value.push(self.curr_char.unwrap());
                self.advance();
            } else if self.curr_char.unwrap() == '.' {
                value.push(self.curr_char.unwrap());
                self.advance();
                if self.curr_char.is_none()
                    || !NUMBERLIST.contains(self.curr_char.unwrap())
                    || self.curr_char.unwrap() == '.'
                {
                    // disadvance the position to match the real position of the error-taking token
                    self.curr_pos.literal_pos -= 1;
                    if self.curr_char.is_some() && self.curr_char.unwrap() == '\n' {
                        self.curr_pos.col -= 1;
                        self.curr_pos.row = 0;
                    } else {
                        self.curr_pos.col -= 1;
                    };

                    return self.generate_individual_error(
                        "Number error".to_string(),
                        value,
                        pos_start,
                        self.curr_pos.clone(),
                    );
                } else {
                    value.push(self.curr_char.unwrap());
                    self.advance();
                }
            }
        }
        let tok: Option<Token> = Some(Token::new(
            String::from(NUMBER),
            value,
            pos_start,
            self.curr_pos.clone(),
        ));
        let err: Option<Error> = None;
        (tok, err)
    }

    fn make_word(&mut self) -> (String, Position) {
        let pos_start: Position = self.curr_pos.clone();
        let mut word: String = String::new();
        while self.curr_char.is_some() && AVAILABLE_CHARACTERS.contains(self.curr_char.unwrap()) {
            word.push(self.curr_char.unwrap());
            self.advance();
        }

        (word, pos_start)
    }

    // INFO: make some tokens
    // After initializing the lexer with the proper fname and fcontent, now we can call this function
    // to create tokens from fcontent
    pub fn make_tokens(&mut self) -> (Option<Vec<Token>>, Option<Error>) {
        let mut tokens: Option<Vec<Token>> = Some(Vec::new());
        let mut err: Option<Error> = None;
        let keywords: HashMap<&str, &str> = AVAILABLE_KEYWORDS.iter().cloned().collect();
        while self.curr_char.is_some() {
            // basically a match pattern to check the current character in the lexer,
            // then create a token based on that current token
            match self.curr_char.unwrap() {
                ' ' | '\n' | '\t' => {
                    self.advance();
                }
                '+' => {
                    let token: Token = Token::new(
                        String::from(PLUS),
                        String::new(),
                        self.curr_pos.clone(),
                        self.curr_pos.clone(),
                    );
                    tokens.as_mut().unwrap().push(token);
                    self.advance();
                }
                '-' => {
                    let token: Token = Token::new(
                        String::from(MINUS),
                        String::new(),
                        self.curr_pos.clone(),
                        self.curr_pos.clone(),
                    );
                    tokens.as_mut().unwrap().push(token);
                    self.advance();
                }
                '*' => {
                    let token: Token = Token::new(
                        String::from(MULTIPLY),
                        String::new(),
                        self.curr_pos.clone(),
                        self.curr_pos.clone(),
                    );
                    tokens.as_mut().unwrap().push(token);
                    self.advance();
                }
                '/' => {
                    let token: Token = Token::new(
                        String::from(DIVIDE),
                        String::new(),
                        self.curr_pos.clone(),
                        self.curr_pos.clone(),
                    );
                    tokens.as_mut().unwrap().push(token);
                    self.advance();
                }

                '(' => {
                    let token: Token = Token::new(
                        String::from(PARENTHESE_OPEN),
                        String::new(),
                        self.curr_pos.clone(),
                        self.curr_pos.clone(),
                    );
                    tokens.as_mut().unwrap().push(token);
                    self.advance();
                }
                ')' => {
                    let token: Token = Token::new(
                        String::from(PARENTHESE_CLOSE),
                        String::new(),
                        self.curr_pos.clone(),
                        self.curr_pos.clone(),
                    );
                    tokens.as_mut().unwrap().push(token);
                    self.advance();
                }
                '0'..='9' | '.' => {
                    let (token, error) = self.number_token();
                    if error.is_some() {
                        tokens = None;
                        err = error;
                        break;
                    } else {
                        tokens.as_mut().unwrap().push(token.unwrap());
                    }
                }
                '&' => {
                    let pos_start: Position = self.curr_pos.clone();
                    self.advance();
                    if self.curr_char.is_none() || self.curr_char.unwrap() != '&' {
                        return self.generate_error(
                            "UnidentifiedIdentifier".to_string(),
                            String::new(),
                            pos_start,
                            self.curr_pos.clone(),
                        );
                    } else {
                        let token: Token = Token::new(
                            String::from(AND),
                            String::new(),
                            pos_start,
                            self.curr_pos.clone(),
                        );
                        tokens.as_mut().unwrap().push(token);
                        self.advance()
                    }
                }
                '|' => {
                    let pos_start: Position = self.curr_pos.clone();
                    self.advance();
                    if self.curr_char.is_none() || self.curr_char.unwrap() != '|' {
                        return self.generate_error(
                            "UnidentifiedIdentifier".to_string(),
                            String::new(),
                            pos_start,
                            self.curr_pos.clone(),
                        );
                    } else {
                        let token: Token = Token::new(
                            String::from(OR),
                            String::new(),
                            pos_start,
                            self.curr_pos.clone(),
                        );
                        tokens.as_mut().unwrap().push(token);
                        self.advance()
                    }
                }
                '\"' => {
                    let (token, error) = self.string_token();
                    if error.is_some() {
                        tokens = None;
                        err = error;
                        break;
                    } else {
                        tokens.as_mut().unwrap().push(token.unwrap());
                        self.advance();
                    }
                }
                '=' => {
                    let pos_start: Position = self.curr_pos.clone();
                    self.advance();
                    if self.curr_char.is_none() || self.curr_char.unwrap() != '=' {
                        return self.generate_error(
                            "UnidentifiedIdentifier".to_string(),
                            String::from("="),
                            pos_start,
                            self.curr_pos.clone(),
                        );
                    } else {
                        let token: Token = Token::new(
                            String::from(EQUAL),
                            String::new(),
                            pos_start,
                            self.curr_pos.clone(),
                        );
                        tokens.as_mut().unwrap().push(token);
                        self.advance();
                    }
                }
                '!' => {
                    let pos_start: Position = self.curr_pos.clone();
                    self.advance();
                    if self.curr_char.is_none() || self.curr_char.unwrap() != '=' {
                        return self.generate_error(
                            "UnidentifiedIdentifier".to_string(),
                            String::from("!"),
                            pos_start,
                            self.curr_pos.clone(),
                        );
                    } else {
                        let token: Token = Token::new(
                            String::from(NOT_EQUAL),
                            String::new(),
                            pos_start,
                            self.curr_pos.clone(),
                        );
                        tokens.as_mut().unwrap().push(token);
                        self.advance();
                    }
                }
                '<' => {
                    let pos_start: Position = self.curr_pos.clone();
                    self.advance();
                    if self.curr_char.is_none() || self.curr_char.unwrap() != '=' {
                        let token: Token = Token::new(
                            String::from(LESS),
                            String::new(),
                            pos_start,
                            self.curr_pos.clone(),
                        );
                        tokens.as_mut().unwrap().push(token);
                        self.advance();
                    } else {
                        let token: Token = Token::new(
                            String::from(LESS_OR_EQUAL),
                            String::new(),
                            pos_start,
                            self.curr_pos.clone(),
                        );
                        tokens.as_mut().unwrap().push(token);
                        self.advance();
                    }
                }
                '>' => {
                    let pos_start: Position = self.curr_pos.clone();
                    self.advance();
                    if self.curr_char.is_none() || self.curr_char.unwrap() != '=' {
                        let token: Token = Token::new(
                            String::from(GREATER),
                            String::new(),
                            pos_start,
                            self.curr_pos.clone(),
                        );
                        tokens.as_mut().unwrap().push(token);
                        self.advance();
                    } else {
                        let token: Token = Token::new(
                            String::from(GREATER_OR_EQUAL),
                            String::new(),
                            pos_start,
                            self.curr_pos.clone(),
                        );
                        tokens.as_mut().unwrap().push(token);
                        self.advance();
                    }
                }

                _ => {
                    let (keyword, pos_start) = self.make_word();
                    match keywords.get(keyword.as_str()) {
                        Some(val) => {
                            let token: Token = Token::new(
                                val.to_string(),
                                String::new(),
                                pos_start,
                                self.curr_pos.clone(),
                            );
                            tokens.as_mut().unwrap().push(token);
                        }
                        None => {
                            return self.generate_error(
                                "UnidentifiedIdentifier".to_string(),
                                keyword,
                                pos_start,
                                self.curr_pos.clone(),
                            );
                        }
                    }
                }
            };
        }

        if err.is_none() {
            // create an EOF token
            if !tokens
                .as_ref()
                .expect("Something went wrong with the tokens")
                .is_empty()
            {
                let pos_start: Position = tokens
                    .as_mut()
                    .expect("Something went wrong with the pos_start")
                    .last()
                    .unwrap()
                    .pos_start
                    .clone();
                let pos_end: Position = tokens
                    .as_mut()
                    .expect("Something went wrong with the pos_end")
                    .last()
                    .unwrap()
                    .pos_end
                    .clone();
                tokens.as_mut().unwrap().push(Token::new(
                    String::from(EOF),
                    String::new(),
                    pos_start.clone(),
                    pos_end.clone(),
                ));
            } else {
                tokens.as_mut().unwrap().push(Token::new(
                    String::from(EOF),
                    String::new(),
                    self.curr_pos.clone(),
                    self.curr_pos.clone(),
                ));
            }

            // then return it
            (tokens, err)
        } else {
            (tokens, err)
        }
    }
}
