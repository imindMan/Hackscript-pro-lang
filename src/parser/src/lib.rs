// Start of the parser
//
// This bit of code contains some parser initialization to get into the interpreter.

use ast;
use error_handling::Error;
use hacktypes;
use lexer::Token;

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

    // pub fn factor(&mut self) -> (Option<ast::NumberNode>, Option<Error>) {
    //     if self.curr_tok._type == hacktypes::NUMBER {
    //         self.advance();
    //         let factor: Option<ast::NumberNode> = Some(ast::NumberNode::new(self.curr_tok.clone()));
    //         let err: Option<Error> = None;
    //         (factor, err)
    //     } else {
    //         let factor: Option<ast::NumberNode> = None;
    //         let err: Option<Error> = Some(Error::new(
    //             "Expect".to_string(),
    //             "A number type token has been expected".to_string(),
    //         ));
    //         (factor, err)
    //     }
    // }
    //
    pub fn factor(&mut self) -> (Option<ast::AST>, Option<Error>) {
        if self.curr_tok._type == hacktypes::NUMBER {
            self.advance();
            let factor: Option<ast::AST> = Some(ast::AST::new_factor(self.curr_tok.clone()));
            let err: Option<Error> = None;
            (factor, err)
        } else {
            let factor: Option<ast::AST> = None;
            let err: Option<Error> = Some(Error::new(
                "Expect".to_string(),
                "A number type token has been expected".to_string(),
            ));
            (factor, err)
        }
    }
}
