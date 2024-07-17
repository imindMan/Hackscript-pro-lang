use ast::AST;
use error_handling::Error;
use lexer::Token;
use position::Position;

pub struct Parser {
    tokens: Vec<Token>,
    curr_tok: Token,
    curr_index: usize,
}

// INFO: General grammar rules with improvisation, this is the roles of all the AST nodes in
// Hackscript
// Factor (the smallest unit of Hackscript until now): Number
//         Unary: (PLUS||MINUS)((PLUS|MINUS) Number)*
//         LEFT_PAREN expr RIGHT_PAREN
// Term (or new name: FormingCalc lvl1): Factor ((MUL||DIV) Factor)*
// Expr (or new name: Forming Calc lvl2): Term ((PLUS||MINUS) Term)*
//
//

impl Parser {
    // INFO: This is the initialization method of the Parser
    pub fn new(tokens: Vec<Token>) -> Parser {
        let curr_index: usize = 0;
        let curr_tok = tokens[curr_index].clone();
        Parser {
            tokens,
            curr_index,
            curr_tok,
        }
    }

    // since the parser only has one function, that is to parse tokens into AST, this is the only
    // method beside the init method that is publicized

    // INFO: After initializing the parser, this function will parse the tokens and return the AST
    // for the interpreter
    pub fn parse(&mut self) -> (Option<AST>, Option<Error>) {
        if self.curr_tok._type == hacktypes::EOF {
            let expr: Option<AST> = Some(AST::Nil);
            let err: Option<Error> = None;
            return (expr, err);
        }
        // for now the FormingCalc AST (see the src/ast for more information) is the top node
        let (expr, err) = self.expr();
        (expr, err)
    }

    // ------------------------------------------------------------------
    // INFO: Below (until the next block of comment like this)
    // are the necessary methods that are generally used in the parser
    // -----------------------------------------------------------------

    fn advance(&mut self) {
        self.curr_index += 1;
        if self.curr_index >= self.tokens.len() {
            panic!("Cannot advance more to make the AST");
        }
        self.curr_tok = self.tokens[self.curr_index].clone();
    }

    fn generate_error(
        &mut self,
        r#type: String,
        extra_string: String,
        pos_start: Position,
        pos_end: Position,
    ) -> (Option<AST>, Option<Error>) {
        let ast: Option<AST> = Some(AST::Nil);
        let err: Option<Error> = Some(Error::new(r#type, extra_string, pos_start, pos_end));
        (ast, err)
    }

    // ----------------------------------------------------
    // INFO: Below (until the next block of comment like this)
    // are all the sub-methods for the factor method, which
    // plays a role to make the smallest units of Hackscript (see the grammar description above)
    // It's the smallest unit, but it doesn't mean it needs to be "small". It should act like it's
    // small because if it doesn't behave like that, the code will mess up.
    // ->
    // For example: (1 + 2) * 3 + 4
    // (1 + 2) is clearly not the small unit in this arithmetic expression. But in general
    // it is **indeed** a factor in the (1 + 2) * 3 method, which means it's the smallest unit
    // of the code
    // ----------------------------------------------------
    //
    fn number_making(&mut self) -> (Option<AST>, Option<Error>) {
        let factor: Option<AST> = Some(AST::new_factor(self.curr_tok.clone()));
        let err: Option<Error> = None;
        self.advance();
        (factor, err)
    }

    fn unary_number_making(&mut self) -> (Option<AST>, Option<Error>) {
        let sign = self.curr_tok.clone();
        self.advance();
        let (factor, err) = self.factor();
        if err.is_some() {
            (factor, err)
        } else {
            let unary: Option<AST> = Some(AST::new_unaryfactor(
                sign,
                Box::new(factor.unwrap().clone()),
            ));
            (unary, err)
        }
    }
    fn in_parentheses_expr(&mut self) -> (Option<AST>, Option<Error>) {
        let pos_start = self.curr_tok.pos_start.clone();
        self.advance();
        let (factor, err) = self.expr();
        if err.is_some() {
            (factor, err)
        } else if self.curr_tok._type != hacktypes::PARENTHESE_CLOSE {
            self.generate_error(
                "Expect".to_string(),
                "the expression should be closed by a ')' (close parenthese), found EOF."
                    .to_string(),
                pos_start,
                self.curr_tok.pos_end.clone(),
            )
        } else {
            self.advance();
            (factor, err)
        }
    }

    // ------------------------------------------------------
    // INFO: THIS IS THE MAIN PART OF THE PARSER
    // ------------------------------------------------------

    fn factor(&mut self) -> (Option<AST>, Option<Error>) {
        if self.curr_tok._type == hacktypes::NUMBER {
            return self.number_making();
        } else if [hacktypes::PLUS, hacktypes::MINUS].contains(&self.curr_tok._type.as_str()) {
            return self.unary_number_making();
        } else if self.curr_tok._type == hacktypes::PARENTHESE_OPEN {
            return self.in_parentheses_expr();
        } else {
            return self.generate_error(
                "Expect".to_string(),
                "a number type token, '+', '-', and '('".to_string(),
                self.curr_tok.pos_start.clone(),
                self.curr_tok.pos_end.clone(),
            );
        }
    }

    fn term(&mut self) -> (Option<AST>, Option<Error>) {
        // Parse the Factor
        let (node1, err1) = self.factor();
        if err1.is_some() {
            return (node1, err1);
        }
        let mut term: Option<AST> = node1;
        if ![
            hacktypes::PLUS,
            hacktypes::MINUS,
            hacktypes::MULTIPLY,
            hacktypes::DIVIDE,
        ]
        .contains(&self.curr_tok._type.as_str())
        {
            return self.generate_error(
                "Expect".to_string(),
                "an operator like '+', '-', '*' or '/', found a different token".to_string(),
                self.curr_tok.pos_start.clone(),
                self.curr_tok.pos_end.clone(),
            );
        }

        // Parse the ((MUL||DIV) Factor)*

        while [hacktypes::MULTIPLY, hacktypes::DIVIDE].contains(&self.curr_tok._type.as_str()) {
            let operator: Option<Token> = Some(self.curr_tok.clone());
            self.advance();
            let (factor2, err2) = self.factor();
            if err2.is_some() {
                return (factor2, err2);
            } else {
                term = Some(AST::new_formingcalc(
                    Box::new(term.clone().unwrap()),
                    operator,
                    Box::new(factor2.clone().unwrap()),
                ));
            };
        }

        let operator: Option<Token> = None;
        if term.is_none() {
            term = Some(AST::new_formingcalc(
                Box::new(term.unwrap()),
                operator,
                Box::new(AST::new()),
            ));
        }
        let err: Option<Error> = None;
        (term, err)
    }

    fn expr(&mut self) -> (Option<AST>, Option<Error>) {
        // parse the Term
        let (node1, err1) = self.term();
        if err1.is_some() {
            return (node1, err1);
        };

        let mut expr: Option<AST> = node1;
        if ![
            hacktypes::PLUS,
            hacktypes::MINUS,
            hacktypes::MULTIPLY,
            hacktypes::DIVIDE,
        ]
        .contains(&self.curr_tok._type.as_str())
        {
            return self.generate_error(
                "Expect".to_string(),
                "an operator like '+', '-', '*' or '/', found a different token".to_string(),
                self.curr_tok.pos_start.clone(),
                self.curr_tok.pos_end.clone(),
            );
        }

        // parse the ((PLUS||MINUS) Term)*
        while [hacktypes::PLUS, hacktypes::MINUS].contains(&self.curr_tok._type.as_str()) {
            let operator: Option<Token> = Some(self.curr_tok.clone());
            self.advance();
            let (term2, err2) = self.term();
            if err2.is_some() {
                return (term2, err2);
            } else {
                expr = Some(AST::new_formingcalc(
                    Box::new(expr.clone().unwrap()),
                    operator,
                    Box::new(term2.clone().unwrap()),
                ));
            }
        }

        let operator: Option<Token> = None;

        if expr.is_none() {
            expr = Some(AST::new_formingcalc(
                Box::new(expr.unwrap()),
                operator,
                Box::new(AST::new()),
            ));
        }
        let err: Option<Error> = None;
        (expr, err)
    }
}
