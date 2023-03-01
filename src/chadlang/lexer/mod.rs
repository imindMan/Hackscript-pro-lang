/*
 *
 * First start of the Lexer - one of the first step in a programming language
 *
 * ChadLang supports a lot of tools to make a programming language, and so does Lexer
 *
 * 
*/

// token initialization
pub struct Token {
    
    _type: String,
    value: String,

}


pub struct Lexer {

    input_string: String,
    tokens: Vec<Token>,
    
}
