// Start of the parser
//
// This bit of code contains some parser initialization to get into the interpreter.

use lexer::Token;
use nodes;

struct Parser {
    tokens: Vec<Token>,
    curr_tok: Token,
    curr_index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        let curr_index_: usize = 0;
        let curr_tok_ = tokens[curr_index_].clone();
        Parser {
            tokens: tokens,
            curr_index: curr_index_,
            curr_tok: curr_tok_,
        }
    }

    pub fn advance(&mut self) {
        self.curr_index += 1;
        if self.curr_index >= self.tokens.len() {
            panic!("Cannot advance more to make the AST");
        }
        self.curr_tok = self.tokens[self.curr_index].clone();
    }

    // pub fn factor(&self) -> nodes::NumberNode {}
    // pub fn term(&self) -> nodes::FormingCalc {}
    // // for now this is the highest node
    // pub fn calc(&self) -> nodes::FormingCalc {
    //     // create term
    //     let term = self.term();
    // }
}
