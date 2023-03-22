/*
 
 * First start of the Lexer - one of the first step in a programming language
 *
 * ChadLang supports a lot of tools to make a programming language, and so does Lexer
 *
 * 
*/

use logger::Logger;
use std::collections::HashMap;
use serde_json::Value;
// token initialization

#[derive(Debug)]
pub struct Token {
    
    token_type: String,
    value: String,

}

// implementation for Token 

impl Token {
    fn new(token_type: String, value: String) -> Token {
        Token {
            token_type: token_type,
            value: value,
        }
        
    }
}

// lexer initialization
#[derive(Debug)]

pub struct Lexer {

    input_string: String,
    tokens: Vec<Token>,
    type_of_tokens: HashMap<String, Token>,
    log: bool,
}


/*
 *
 * Idea: 
 * Based on the .json file parsing through the .json file, the Lexer will start making 
 * tokens
 *
 * It will keep track of all the characters in the input_string, and then 
 * figure out what tokens should it make. If it has errors during the "making tokens"
 * it will stop the program and return the error.
 * For instance, let's say that the input .json file looks like this
 *  {
 *      "+": ["plus", None],
 *      "-": ["minus", None],
 *      "*": ["mul", None],
 *      "/": ["div", None],
 *      "!error": ["!error: \"This is an error in this position: (?poscol,
 *      ?posend)\nCan't scan all the tokens\"", None]
 *  }
 * The lexer will get the current character and then check if that character is valid to
 * make token (based on the .json file) and then if that's the case, then it will generate a token
 * if it isn't, it will throws an error.
 *
 *
 * */

impl Lexer {
    

    // new lexer !!!
    pub fn new(string: String) -> Lexer {

        Lexer {
            input_string: string,
            tokens: Vec::new(),
            type_of_tokens: HashMap::new(),
            log: false,
        }
    }
    // logging stuff 
    pub fn log(&mut self, signal: bool) {
        self.log = signal;
    }
    // initialize lexer tokens being
    pub fn type_of_tokens_initialization(&mut self, lexer_config: &Value) {
        let mut _logger = Logger::new();
        if let Value::Array(lexer) = lexer_config {
            let mut count_element = 0;
            for token_define in lexer {
                if let Value::Array(token_define) = &token_define {
                    // start initializing every token 
                    let token_identify = match &token_define[0] {

                        Value::String(string) => string,
                        _ => panic!("Something wrong with the second element of the lexer, inside: {:?} token definition", count_element),
                    };

                    let token_type = match &token_define[1] {
                        Value::String(string) => string,
                        _ => panic!("Something wrong with the second element of the lexer, inside: {:?} token definition", count_element),
                    };
                    let token_value = match &token_define[2] {
                        Value::String(string) => string,
                         _ => panic!("Something wrong with the third element of the lexer, inside: {:?} token definition", count_element),
                    };

                    let token = Token::new(token_type.to_string().clone(), token_value.to_string().clone());

                    self.type_of_tokens.insert(token_identify.to_string(), token);
                    count_element += 1;
                }
            }
        };

    }
    pub fn make_tokens(&self) {
        
    }


}
