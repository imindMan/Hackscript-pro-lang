//  INFO:  Lexer first start.
//  Lexer will parse through every characters and create tokens
//  So tokens can be defined in here, too

use error_handling::Error;
use position::Position;

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
    fname: String,
    fcontent: String,
}

impl Lexer {
    // INFO: This is the Initialization method of Lexer
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
    // ------------------------------------------------------------------
    // INFO: Below are some methods that are frequently used in the lexer
    // ------------------------------------------------------------------

    fn create_a_token(
        &self,
        _type: String,
        value: String,
        pos_start: Position,
        pos_end: Position,
    ) -> Token {
        Token::new(_type, value, pos_start, pos_end)
    }
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

    fn number_token(&mut self) -> (Option<Token>, Option<Error>) {
        let pos_start = self.curr_pos.clone();
        let mut value: String = String::new();

        while self.curr_char.is_some() && hacktypes::NUMBERLIST.contains(self.curr_char.unwrap()) {
            if hacktypes::NUMBERLIST.contains(self.curr_char.unwrap()) {
                value.push(self.curr_char.unwrap());
                self.advance();
            } else if self.curr_char.unwrap() == '.' {
                value.push(self.curr_char.unwrap());
                self.advance();
                if self.curr_char.is_none()
                    || !hacktypes::NUMBERLIST.contains(self.curr_char.unwrap())
                {
                    // disadvance the position to match the real position of the error-taking token
                    self.curr_pos.literal_pos -= 1;
                    if self.curr_char.unwrap() == '\n' {
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

    // INFO: make some tokens
    // After initializing the lexer with the proper fname and fcontent, now we can call this function
    // to create tokens from fcontent
    pub fn make_tokens(&mut self) -> (Option<Vec<Token>>, Option<Error>) {
        let mut tokens: Option<Vec<Token>> = Some(Vec::new());
        let mut err: Option<Error> = None;
        while self.curr_char.is_some() {
            // basically a match pattern to check the current character in the lexer,
            // then create a token based on that current token
            match self.curr_char.unwrap() {
                ' ' | '\n' | '\t' => {
                    self.advance();
                }
                '+' => {
                    let token: Token = self.create_a_token(
                        String::from(hacktypes::PLUS),
                        String::from(""),
                        self.curr_pos.clone(),
                        self.curr_pos.clone(),
                    );
                    tokens.as_mut().unwrap().push(token);
                    self.advance();
                }
                '-' => {
                    let token: Token = self.create_a_token(
                        String::from(hacktypes::MINUS),
                        String::from(""),
                        self.curr_pos.clone(),
                        self.curr_pos.clone(),
                    );
                    tokens.as_mut().unwrap().push(token);
                    self.advance();
                }
                '*' => {
                    let token: Token = self.create_a_token(
                        String::from(hacktypes::MULTIPLY),
                        String::from(""),
                        self.curr_pos.clone(),
                        self.curr_pos.clone(),
                    );
                    tokens.as_mut().unwrap().push(token);
                    self.advance();
                }
                '/' => {
                    let token: Token = self.create_a_token(
                        String::from(hacktypes::DIVIDE),
                        String::from(""),
                        self.curr_pos.clone(),
                        self.curr_pos.clone(),
                    );
                    tokens.as_mut().unwrap().push(token);
                    self.advance();
                }
                '(' => {
                    let token: Token = self.create_a_token(
                        String::from(hacktypes::PARENTHESE_OPEN),
                        String::from(""),
                        self.curr_pos.clone(),
                        self.curr_pos.clone(),
                    );
                    tokens.as_mut().unwrap().push(token);
                    self.advance();
                }
                ')' => {
                    let token: Token = self.create_a_token(
                        String::from(hacktypes::PARENTHESE_CLOSE),
                        String::from(""),
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
                _ => {
                    return self.generate_error(
                        "Undefined character".to_string(),
                        self.curr_char.expect("No character").to_string(),
                        self.curr_pos.clone(),
                        self.curr_pos.clone(),
                    );
                }
            };
        }

        if err.is_none() {
            // create an EOF token
            if !tokens.as_ref().expect("No existing tokens").is_empty() {
                let pos_start: Position = tokens
                    .as_ref()
                    .expect("No existing tokens")
                    .last()
                    .unwrap()
                    .pos_start
                    .clone();
                let pos_end: Position = tokens
                    .as_ref()
                    .expect("No existing tokens")
                    .last()
                    .unwrap()
                    .pos_end
                    .clone();
                tokens.as_mut().unwrap().push(self.create_a_token(
                    String::from(hacktypes::EOF),
                    String::from(""),
                    pos_start,
                    pos_end,
                ));
            } else {
                tokens.as_mut().unwrap().push(self.create_a_token(
                    String::from(hacktypes::EOF),
                    String::from(""),
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
