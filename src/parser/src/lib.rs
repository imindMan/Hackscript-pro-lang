// Start of the parser
//
// This bit of code contains some parser initialization to get into the interpreter.

use ast::AST;
use error_handling::Error;
use lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    curr_tok: Token,
    curr_index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        let curr_index: usize = 0;
        let curr_tok = tokens[curr_index].clone();
        Parser {
            tokens,
            curr_index,
            curr_tok,
        }
    }

    pub fn parse(&mut self) -> (Option<AST>, Option<Error>) {
        if self.curr_tok._type == hacktypes::EOF {
            let expr: Option<AST> = None;
            let err: Option<Error> = None;
            return (expr, err);
        }
        let (expr, err) = self.expr();
        (expr, err)
    }

    fn advance(&mut self) {
        self.curr_index += 1;
        if self.curr_index >= self.tokens.len() {
            panic!("Cannot advance more to make the AST");
        }
        self.curr_tok = self.tokens[self.curr_index].clone();
    }

    fn factor(&mut self) -> (Option<AST>, Option<Error>) {
        if self.curr_tok._type == hacktypes::NUMBER {
            let factor: Option<AST> = Some(AST::new_factor(self.curr_tok.clone()));
            let err: Option<Error> = None;
            self.advance();
            (factor, err)
        } else if self.curr_tok._type == hacktypes::EOF {
            let factor: Option<AST> = None;
            let mut err: Option<Error> = Some(Error::new(
                "Expect".to_string(),
                "a number type token, found EOF".to_string(),
            ));
            err.as_mut().unwrap().imply_error_message(
                self.curr_tok.pos_start.clone(),
                self.curr_tok.pos_end.clone(),
            );
            (factor, err)
        } else {
            let factor: Option<AST> = None;
            let mut err: Option<Error> = Some(Error::new(
                "Expect".to_string(),
                "a number type token".to_string(),
            ));
            err.as_mut().unwrap().imply_error_message(
                self.curr_tok.pos_start.clone(),
                self.curr_tok.pos_end.clone(),
            );

            (factor, err)
        }
    }

    fn term(&mut self) -> (Option<AST>, Option<Error>) {
        let (node1, err1) = self.factor();
        if err1.is_some() {
            return (node1, err1);
        }
        if self.curr_tok._type == hacktypes::NUMBER {
            let term: Option<AST> = None;
            let mut err: Option<Error> = Some(Error::new(
                "Expect".to_string(),
                "an operator like '+', '-', '*', or '/', found a number".to_string(),
            ));
            err.as_mut().unwrap().imply_error_message(
                self.curr_tok.pos_start.clone(),
                self.curr_tok.pos_end.clone(),
            );
            return (term, err);
        }

        let mut term: Option<AST> = None;

        while [hacktypes::MULTIPLY, hacktypes::DIVIDE].contains(&self.curr_tok._type.as_str()) {
            let operator: String = self.curr_tok._type.to_string();
            self.advance();
            let (term2, err2) = self.term();
            if err2.is_some() {
                return (term2, err2);
            } else {
                term = Some(AST::new_formingcalc(
                    Box::new(node1.clone().unwrap()),
                    operator,
                    Box::new(term2.clone().unwrap()),
                ));
            }
        }

        if term.is_none() {
            term = Some(AST::new_formingcalc(
                Box::new(node1.unwrap()),
                String::new(),
                Box::new(AST::new()),
            ));
        }
        let err: Option<Error> = None;
        (term, err)
    }

    fn expr(&mut self) -> (Option<AST>, Option<Error>) {
        let (node1, err1) = self.term();
        if err1.is_some() {
            return (node1, err1);
        }
        if self.curr_tok._type == hacktypes::NUMBER {
            let expr: Option<AST> = None;
            let mut err: Option<Error> = Some(Error::new(
                "Expect".to_string(),
                "an operator like '+', '-', '*', or '/', found a number".to_string(),
            ));
            err.as_mut().unwrap().imply_error_message(
                self.curr_tok.pos_start.clone(),
                self.curr_tok.pos_end.clone(),
            );
            return (expr, err);
        }
        let mut expr: Option<AST> = None;

        while [hacktypes::PLUS, hacktypes::MINUS].contains(&self.curr_tok._type.as_str()) {
            let operator: String = self.curr_tok._type.to_string();
            self.advance();
            let (expr2, err2) = self.expr();
            if err2.is_some() {
                return (expr2, err2);
            } else {
                expr = Some(AST::new_formingcalc(
                    Box::new(node1.clone().unwrap()),
                    operator,
                    Box::new(expr2.clone().unwrap()),
                ));
            }
        }

        if expr.is_none() {
            expr = Some(AST::new_formingcalc(
                Box::new(node1.unwrap()),
                String::new(),
                Box::new(AST::new()),
            ));
        }
        let err: Option<Error> = None;
        (expr, err)
    }
}
