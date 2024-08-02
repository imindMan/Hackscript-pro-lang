//  INFO:  Lexer first start.
//  Lexer will parse through every characters and create tokens
//  So tokens can be defined in here, too

pub use crate::error_handling::Error;
pub use crate::hacktypes::*;
pub use crate::position::Position;
use std::collections::HashMap;

#[derive(Debug, Clone)]
/*WARNING: This struct is globally accessible everywhere */
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
            curr_char: fcontent.clone().as_str().chars().next(),
            fcontent: fcontent.clone(),
            curr_pos: Position::new(0, 0, 0, fname, fcontent),
        }
    }
    // ------------------------------------------------------------------
    // INFO: Below are some methods that are frequently used in the lexer
    // ------------------------------------------------------------------

    // 'advance' is actually to iterate to the next token
    fn advance(&mut self) {
        self.curr_char = self.curr_pos.advance();
    }
    fn implement_trailing_characters(&mut self) -> Result<char, Error> {
        match self.curr_char.unwrap() {
            'n' => Ok('\n'),
            't' => Ok('\t'),
            '\\' => Ok('\\'),
            '\"' => Ok('\"'),
            _ => Err(Error::new(
                "UnknownTrailingCharacter".to_string(),
                format!(
                    "Doesn't exist this '\\{}' character",
                    self.curr_char.unwrap()
                ),
                self.curr_pos.clone(),
                self.curr_pos.clone(),
            )),
        }
    }

    fn string_token(&mut self) -> Result<Token, Error> {
        let pos_start = self.curr_pos.clone();
        let mut value: String = String::new();
        self.advance();
        while self.curr_char.is_some() && self.curr_char.unwrap() != '\"' {
            if self.curr_char.unwrap() == '\\' {
                self.advance();
                let char = self.implement_trailing_characters()?;
                value.push(char);
            } else {
                value.push(self.curr_char.unwrap());
            }
            self.advance();
        }
        if self.curr_char.is_none() {
            return Err(Error::new(
                "Expect".to_string(),
                "a \" to end the string. Found nothing -> endless string".to_string(),
                pos_start,
                self.curr_pos.clone(),
            ));
        }
        self.advance();
        Ok(Token::new(
            String::from(STRING),
            value,
            pos_start,
            self.curr_pos.clone(),
        ))
    }

    fn number_token(&mut self) -> Result<Token, Error> {
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
                    let _char = self.curr_pos.disadvance();

                    return Err(Error::new(
                        "Number error".to_string(),
                        value,
                        pos_start,
                        self.curr_pos.clone(),
                    ));
                } else {
                    value.push(self.curr_char.unwrap());
                    self.advance();
                }
            }
        }
        Ok(Token::new(
            String::from(NUMBER),
            value,
            pos_start,
            self.curr_pos.clone(),
        ))
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
    pub fn make_tokens(&mut self) -> Result<Vec<Token>, Error> {
        let mut tokens: Vec<Token> = Vec::new();
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
                    tokens.push(token);
                    self.advance();
                }
                '-' => {
                    let token: Token = Token::new(
                        String::from(MINUS),
                        String::new(),
                        self.curr_pos.clone(),
                        self.curr_pos.clone(),
                    );
                    tokens.push(token);
                    self.advance();
                }
                '*' => {
                    let token: Token = Token::new(
                        String::from(MULTIPLY),
                        String::new(),
                        self.curr_pos.clone(),
                        self.curr_pos.clone(),
                    );
                    tokens.push(token);
                    self.advance();
                }
                '/' => {
                    let token: Token = Token::new(
                        String::from(DIVIDE),
                        String::new(),
                        self.curr_pos.clone(),
                        self.curr_pos.clone(),
                    );
                    tokens.push(token);
                    self.advance();
                }

                '(' => {
                    let token: Token = Token::new(
                        String::from(PARENTHESE_OPEN),
                        String::new(),
                        self.curr_pos.clone(),
                        self.curr_pos.clone(),
                    );
                    tokens.push(token);
                    self.advance();
                }
                ')' => {
                    let token: Token = Token::new(
                        String::from(PARENTHESE_CLOSE),
                        String::new(),
                        self.curr_pos.clone(),
                        self.curr_pos.clone(),
                    );
                    tokens.push(token);
                    self.advance();
                }
                '0'..='9' | '.' => {
                    let token = self.number_token()?;
                    tokens.push(token);
                }
                '&' => {
                    let pos_start: Position = self.curr_pos.clone();
                    self.advance();
                    if self.curr_char.is_none() || self.curr_char.unwrap() != '&' {
                        return Err(Error::new(
                            "UnidentifiedIdentifier".to_string(),
                            String::new(),
                            pos_start,
                            self.curr_pos.clone(),
                        ));
                    } else {
                        let token: Token = Token::new(
                            String::from(AND),
                            String::new(),
                            pos_start,
                            self.curr_pos.clone(),
                        );
                        tokens.push(token);
                        self.advance()
                    }
                }
                '|' => {
                    let pos_start: Position = self.curr_pos.clone();
                    self.advance();
                    if self.curr_char.is_none() || self.curr_char.unwrap() != '|' {
                        return Err(Error::new(
                            "UnidentifiedIdentifier".to_string(),
                            String::new(),
                            pos_start,
                            self.curr_pos.clone(),
                        ));
                    } else {
                        let token: Token = Token::new(
                            String::from(OR),
                            String::new(),
                            pos_start,
                            self.curr_pos.clone(),
                        );
                        tokens.push(token);
                        self.advance()
                    }
                }
                '\"' => {
                    let token = self.string_token()?;
                    tokens.push(token);
                }
                '=' => {
                    let pos_start: Position = self.curr_pos.clone();
                    self.advance();
                    if self.curr_char.is_none() || self.curr_char.unwrap() != '=' {
                        return Err(Error::new(
                            "UnidentifiedIdentifier".to_string(),
                            String::from("="),
                            pos_start,
                            self.curr_pos.clone(),
                        ));
                    } else {
                        let token: Token = Token::new(
                            String::from(EQUAL),
                            String::new(),
                            pos_start,
                            self.curr_pos.clone(),
                        );
                        tokens.push(token);
                        self.advance();
                    }
                }
                '!' => {
                    let pos_start: Position = self.curr_pos.clone();
                    self.advance();
                    if self.curr_char.is_none() || self.curr_char.unwrap() != '=' {
                        return Err(Error::new(
                            "UnidentifiedIdentifier".to_string(),
                            String::from("!"),
                            pos_start,
                            self.curr_pos.clone(),
                        ));
                    } else {
                        let token: Token = Token::new(
                            String::from(NOT_EQUAL),
                            String::new(),
                            pos_start,
                            self.curr_pos.clone(),
                        );
                        tokens.push(token);
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
                        tokens.push(token);
                        self.advance();
                    } else {
                        let token: Token = Token::new(
                            String::from(LESS_OR_EQUAL),
                            String::new(),
                            pos_start,
                            self.curr_pos.clone(),
                        );
                        tokens.push(token);
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
                        tokens.push(token);
                        self.advance();
                    } else {
                        let token: Token = Token::new(
                            String::from(GREATER_OR_EQUAL),
                            String::new(),
                            pos_start,
                            self.curr_pos.clone(),
                        );
                        tokens.push(token);
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
                            tokens.push(token);
                        }
                        None => {
                            return Err(Error::new(
                                "UnidentifiedIdentifier".to_string(),
                                keyword,
                                pos_start,
                                self.curr_pos.clone(),
                            ));
                        }
                    }
                }
            };
        }

        // create an EOF token
        if !tokens.is_empty() {
            let pos_start: Position = tokens.last().unwrap().pos_start.clone();
            let pos_end: Position = tokens.last().unwrap().pos_end.clone();
            tokens.push(Token::new(
                String::from(EOF),
                String::new(),
                pos_start.clone(),
                pos_end.clone(),
            ));
        } else {
            tokens.push(Token::new(
                String::from(EOF),
                String::new(),
                self.curr_pos.clone(),
                self.curr_pos.clone(),
            ));
        }

        Ok(tokens)
    }
}
