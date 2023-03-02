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

// lexer initialization
pub struct Lexer {

    input_string: String,
    tokens: Vec<Token>,
    
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
